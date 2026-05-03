use anyhow::{Context, Result};
use crossbeam_channel::{bounded, Receiver, Sender};
use gstreamer::prelude::*;
use gstreamer_app::AppSink;
use gstreamer::DeviceMonitor;
use std::sync::{Arc, atomic::{AtomicBool, Ordering}};

pub struct VideoFrame {
    pub data: Vec<u8>,
    pub width: u32,
    pub height: u32,
}

pub struct PipelineController {
    pipeline: gstreamer::Pipeline,
    receiver: Receiver<VideoFrame>,
    heartbeat_stop: Arc<AtomicBool>,
    video_device_id: Option<String>,
    audio_device_id: Option<String>,
    _log_sender: Sender<String>,
}

#[derive(Debug, Clone)]
pub struct DeviceInfo {
    pub name: String,
    pub internal_id: String,
    pub caps: Vec<String>,
}

impl PipelineController {
    pub fn new(
        video_device: Option<String>,
        audio_device: Option<String>,
        resolution: Option<(i32, i32)>,
        fps: Option<i32>,
        pixel_format: Option<&str>,
        log_sender: Sender<String>,
    ) -> Result<Self> {
        let video_id_clone = video_device.clone();
        let audio_id_clone = audio_device.clone();
        let log_clone = log_sender.clone();
        gstreamer::init().context("Failed to initialize GStreamer")?;

        let video_src = match video_device {
            Some(ref name) => {
                let is_logitech = name.to_lowercase().contains("logitech")
                    || name.to_lowercase().contains("c922");

                // Build caps constraint suffix (comma-prefixed, appended after media type)
                let res_fps = match (resolution, fps) {
                    (Some((w, h)), Some(f)) => format!(",width={},height={},framerate={}/1", w, h, f),
                    (Some((w, h)), None)    => format!(",width={},height={}", w, h),
                    (None, Some(f))         => format!(",framerate={}/1", f),
                    _                       => String::new(),
                };

                match pixel_format {
                    Some("MJPG") => {
                        // Explicit MJPG: ksvideosrc (Kernel Streaming) bypasses MF 15fps cap
                        format!("mfvideosrc device-name=\"{}\" ! image/jpeg{} ! jpegdec", name, res_fps)
                    }
                    Some("YUY2") => {
                        // Explicit YUY2 raw video
                        format!("mfvideosrc device-name=\"{}\" ! video/x-raw,format=YUY2{}", name, res_fps)
                    }
                    _ => {
                        // Auto mode: ksvideosrc + MJPG to unlock 60fps on Logitech
                        if !res_fps.is_empty() {
                            // Specific res/fps requested
                            format!("mfvideosrc device-name=\"{}\" ! image/jpeg{} ! jpegdec", name, res_fps)
                        } else if is_logitech {
                            // Known Logitech: KS + MJPG to escape Windows MF 15fps limit
                            format!("mfvideosrc device-name=\"{}\" ! image/jpeg ! jpegdec", name)
                        } else {
                            // Generic device
                            format!("mfvideosrc device-name=\"{}\" ! decodebin", name)
                        }
                    }
                }
            }
            None => "videotestsrc pattern=black".to_string(),
        };

        let audio_src = match audio_device {
            Some(ref guid) if !guid.is_empty() => {
                // 100ms buffer (leaky=downstream) reduces audio parasite/glitch from capture cards
                format!(
                    "wasapi2src device=\"{}\" low-latency=true ! queue max-size-time=100000000 leaky=downstream ! audioconvert ! audioresample ! volume name=vol ! directsoundsink sync=false",
                    guid
                )
            }
            _ => String::new(),
        };

        let pipeline_str = if audio_src.is_empty() {
            format!(
                "{} ! videobalance name=balance ! videoconvert ! video/x-raw,format=RGBA ! appsink name=sink drop=true max-buffers=1 sync=false",
                video_src
            )
        } else {
            format!(
                "{} ! videobalance name=balance ! videoconvert ! video/x-raw,format=RGBA ! appsink name=sink drop=true max-buffers=1 sync=false {}",
                video_src, audio_src
            )
        };

        let _ = log_sender.send(format!("DEBUG: Launching pipeline with Video: {:?}, Audio: {:?}", video_device, audio_device));
        let _ = log_sender.send(format!("DEBUG: Full string: {}", pipeline_str));

        let pipeline = gstreamer::parse::launch(&pipeline_str)
            .context("Failed to parse GStreamer pipeline")?
            .downcast::<gstreamer::Pipeline>()
            .map_err(|_| anyhow::anyhow!("Expected a Pipeline"))?;

        let bus = pipeline.bus().context("Failed to get pipeline bus")?;
        let log_bus = log_clone.clone();
        bus.set_sync_handler(move |_, msg| {
            use gstreamer::MessageView;
            match msg.view() {
                MessageView::Error(err) => {
                    let _ = log_bus.send(format!("GSTREAMER ERROR: {}", err.error()));
                }
                MessageView::Warning(warn) => {
                    let _ = log_bus.send(format!("GSTREAMER WARNING: {}", warn.error()));
                }
                _ => (),
            }
            gstreamer::BusSyncReply::Drop
        });

        // Heartbeat with AtomicBool stop flag — prevents thread leak on pipeline restart
        let heartbeat_stop = Arc::new(AtomicBool::new(false));
        let pipeline_clone = pipeline.clone();
        let stop_flag = heartbeat_stop.clone();
        std::thread::spawn(move || {
            while !stop_flag.load(Ordering::Relaxed) {
                std::thread::sleep(std::time::Duration::from_secs(5));
                if stop_flag.load(Ordering::Relaxed) { break; }
                let state = pipeline_clone.state(gstreamer::ClockTime::NONE).1;
                println!("HEARTBEAT: Pipeline state: {:?}", state);
            }
        });

        let appsink = pipeline
            .by_name("sink")
            .context("Failed to find appsink")?
            .downcast::<AppSink>()
            .map_err(|_| anyhow::anyhow!("Expected an AppSink"))?;

        // Bounded channel: prevents unbounded memory growth when render is slower than capture
        let (sender, receiver) = bounded(2);

        appsink.set_callbacks(
            gstreamer_app::AppSinkCallbacks::builder()
                .new_sample(move |sink| {
                    match Self::pull_sample(sink, &sender) {
                        Ok(_) => Ok(gstreamer::FlowSuccess::Ok),
                        Err(err) => {
                            eprintln!("Failed to pull sample: {}", err);
                            Err(gstreamer::FlowError::Error)
                        }
                    }
                })
                .build(),
        );

        pipeline
            .set_state(gstreamer::State::Playing)
            .context("Failed to set pipeline state to playing")?;

        println!("PIPELINE INITIALIZED SUCCESSFULLY");

        let heartbeat_stop = Arc::new(AtomicBool::new(false));
        Ok(Self { 
            pipeline, 
            receiver, 
            heartbeat_stop,
            video_device_id: video_id_clone,
            audio_device_id: audio_id_clone,
            _log_sender: log_sender,
        })
    }

    pub fn video_device_id(&self) -> Option<String> {
        self.video_device_id.clone()
    }

    pub fn audio_device_id(&self) -> Option<String> {
        self.audio_device_id.clone()
    }

    pub fn receiver(&self) -> Receiver<VideoFrame> {
        self.receiver.clone()
    }

    pub fn update_adjustments(&self, brightness: f32, contrast: f32, saturation: f32, hue: f32, volume: f32, is_muted: bool) {
        if let Some(balance) = self.pipeline.by_name("balance") {
            let _ = balance.set_property("brightness", brightness as f64);
            let _ = balance.set_property("contrast", contrast as f64);
            let _ = balance.set_property("saturation", saturation as f64);
            let _ = balance.set_property("hue", hue as f64);
        }
        if let Some(vol) = self.pipeline.by_name("vol") {
            let v = if is_muted { 0.0 } else { volume as f64 };
            let _ = vol.set_property("volume", v);
        }
    }

    fn pull_sample(sink: &AppSink, sender: &Sender<VideoFrame>) -> Result<()> {
        let sample = sink.pull_sample().context("Failed to pull sample")?;
        // NOTE: println! removed — at 60fps it caused significant I/O scheduling pressure
        let buffer = sample.buffer().context("Sample has no buffer")?;
        let caps = sample.caps().context("Sample has no caps")?;
        let structure = caps.structure(0).context("Caps have no structure")?;

        let width  = structure.get::<i32>("width").context("No width in caps")? as u32;
        let height = structure.get::<i32>("height").context("No height in caps")? as u32;

        let map = buffer.map_readable().context("Failed to map buffer")?;

        let frame = VideoFrame {
            data: map.as_slice().to_vec(),
            width,
            height,
        };

        // try_send: if channel full (bounded 2), drop oldest frame — keeps preview real-time
        let _ = sender.try_send(frame);

        Ok(())
    }

    pub fn discover_devices() -> (Vec<DeviceInfo>, Vec<DeviceInfo>) {
        let _ = gstreamer::init();
        let monitor = DeviceMonitor::new();
        monitor.add_filter(Some("Video/Source"), None);
        monitor.add_filter(Some("Audio/Source"), None);

        let mut video_devices: Vec<DeviceInfo> = Vec::new();
        let mut audio_devices: Vec<DeviceInfo> = Vec::new();

        if monitor.start().is_ok() {
            for device in monitor.devices() {
                let name = device.display_name().to_string();
                let klass = device.device_class().to_string();
                let props = device.properties();

                if klass.contains("Video") {
                    if !video_devices.iter().any(|d| d.name == name) {
                        let mut cap_list = Vec::new();
                        if let Some(caps) = device.caps() {
                            for i in 0..caps.size() {
                                if let Some(s) = caps.structure(i) {
                                    if let (Ok(w), Ok(h)) = (s.get::<i32>("width"), s.get::<i32>("height")) {
                                        let mut available_fps = Vec::new();
                                        if let Ok(f) = s.get::<gstreamer::Fraction>("framerate") {
                                            available_fps.push(f.numer() / f.denom());
                                        } else if let Ok(r) = s.get::<gstreamer::FractionRange>("framerate") {
                                            let min = r.min().numer() / r.min().denom();
                                            let max = r.max().numer() / r.max().denom();
                                            for common in [30, 60] {
                                                if common >= min && common <= max {
                                                    available_fps.push(common);
                                                }
                                            }
                                            if !available_fps.contains(&max) { available_fps.push(max); }
                                        }

                                        for fps in available_fps {
                                            if fps < 24 { continue; }
                                            let cap_str = format!("{}x{} @ {}fps", w, h, fps);
                                            if !cap_list.contains(&cap_str) {
                                                cap_list.push(cap_str);
                                            }
                                        }
                                    }
                                }
                            }
                        }

                        // Sort: largest resolution first, then highest FPS
                        cap_list.sort_by(|a, b| {
                            let parse = |s: &str| -> (i32, i32) {
                                let parts: Vec<&str> = s.split(" @ ").collect();
                                let dims: Vec<&str> = parts[0].split('x').collect();
                                let w = dims[0].parse::<i32>().unwrap_or(0);
                                let h = dims[1].parse::<i32>().unwrap_or(0);
                                let f = parts[1].replace("fps", "").parse::<i32>().unwrap_or(0);
                                (w * h, f)
                            };
                            let (res_a, fps_a) = parse(a);
                            let (res_b, fps_b) = parse(b);
                            res_b.cmp(&res_a).then(fps_b.cmp(&fps_a))
                        });

                        video_devices.push(DeviceInfo { name: name.clone(), internal_id: name, caps: cap_list });
                    }
                } else if klass.contains("Audio") {
                    if !audio_devices.iter().any(|d| d.name == name) {
                        let id = if let Some(p) = props {
                            p.get::<&str>("device.strid")
                             .or_else(|_| p.get::<&str>("device.id"))
                             .unwrap_or(&name)
                             .to_string()
                        } else {
                            name.clone()
                        };
                        audio_devices.push(DeviceInfo { name, internal_id: id, caps: Vec::new() });
                    }
                }
            }
            monitor.stop();
        }

        (video_devices, audio_devices)
    }
}

impl Drop for PipelineController {
    fn drop(&mut self) {
        // Signal heartbeat thread to stop before pipeline teardown
        self.heartbeat_stop.store(true, Ordering::Relaxed);
        let _ = self.pipeline.set_state(gstreamer::State::Null);
    }
}
