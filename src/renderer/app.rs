use crossbeam_channel::{Receiver, Sender};
use std::sync::Arc;
use winit::raw_window_handle::{HasWindowHandle, RawWindowHandle};
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;

use crate::media::pipeline::{DeviceInfo, PipelineController};
use crate::ui::overlay::Overlay;
 
#[cfg(target_os = "windows")]
fn get_hwnd(window: &winit::window::Window) -> *mut std::os::raw::c_void {
    if let Ok(handle) = window.window_handle() {
        if let RawWindowHandle::Win32(win32_handle) = handle.as_raw() {
            win32_handle.hwnd.get() as *mut _
        } else {
            std::ptr::null_mut()
        }
    } else {
        std::ptr::null_mut()
    }
}

pub struct App {
    pipeline: PipelineController,
    video_infos: Vec<DeviceInfo>,
    audio_infos: Vec<DeviceInfo>,
    log_sender: Sender<String>,
    log_receiver: Receiver<String>,
}

impl App {
    pub fn new(video_device: Option<String>, audio_device: Option<String>) -> anyhow::Result<Self> {
        let (video_infos, audio_infos) = PipelineController::discover_devices();
        let (log_sender, log_receiver) = crossbeam_channel::unbounded();

        let video_to_use = video_device.or_else(|| {
            let priority_keywords = ["usb3", "capture", "game"];
            for keyword in priority_keywords {
                if let Some(dev) = video_infos
                    .iter()
                    .find(|d| d.name.to_lowercase().contains(keyword))
                {
                    let _ = log_sender.send(format!(
                        "DEBUG: Priority device found on startup: {}",
                        dev.name
                    ));
                    return Some(dev.internal_id.clone());
                }
            }
            video_infos.first().map(|d| d.internal_id.clone())
        });

        let v_id_match = video_to_use.clone();
        let a_sender = log_sender.clone();
        let audio_to_use = audio_device.or_else(|| {
            if let Some(ref v_name) = v_id_match {
                Self::match_audio_for_video(v_name, &audio_infos, a_sender)
            } else {
                audio_infos.first().map(|d| d.internal_id.clone())
            }
        });

        let pipeline = PipelineController::new(
            video_to_use,
            audio_to_use,
            None,
            None,
            None,
            log_sender.clone(),
        )?;
        Ok(Self {
            pipeline,
            video_infos,
            audio_infos,
            log_sender,
            log_receiver,
        })
    }

    fn match_audio_for_video(
        video_name: &str,
        audio_infos: &[DeviceInfo],
        log_sender: Sender<String>,
    ) -> Option<String> {
        let v_lower = video_name.to_lowercase();
        let _ = log_sender.send(format!("DEBUG: Searching audio for video: {}", video_name));

        // 1. Aşama: Tam isim veya parça eşleşmesi
        for audio in audio_infos {
            let a_lower = audio.name.to_lowercase();
            if a_lower.contains(&v_lower) || v_lower.contains(&a_lower) {
                let _ = log_sender.send(format!("DEBUG: Found direct match: {}", audio.name));
                return Some(audio.internal_id.clone());
            }
        }

        // 2. Aşama: Jenerik eşleşme
        if v_lower.contains("usb") || v_lower.contains("capture") {
            for audio in audio_infos {
                let a_lower = audio.name.to_lowercase();
                if a_lower.contains("digital audio")
                    || a_lower.contains("usb audio")
                    || a_lower.contains("interface")
                {
                    let _ = log_sender.send(format!(
                        "DEBUG: Found generic match for capture card: {}",
                        audio.name
                    ));
                    return Some(audio.internal_id.clone());
                }
            }
        }

        let _ = log_sender.send(format!(
            "DEBUG: No audio match found for {}, falling back to default.",
            video_name
        ));
        None
    }

    fn load_icon() -> Option<winit::window::Icon> {
        match image::open("assets/icon.ico") {
            Ok(img) => {
                let img = img.to_rgba8();
                let (width, height) = img.dimensions();
                let rgba = img.into_raw();
                winit::window::Icon::from_rgba(rgba, width, height).ok()
            }
            Err(_) => None,
        }
    }

    pub fn run(self) -> anyhow::Result<()> {
        let event_loop = EventLoop::new()?;
        let icon = Self::load_icon();
        let window = Arc::new(
            WindowBuilder::new()
                .with_title("Kaptura v1.1.0 - Video Capture Utility")
                .with_inner_size(winit::dpi::LogicalSize::new(1280.0, 720.0))
                .with_window_icon(icon)
                .with_transparent(true)
                .build(&event_loop)
                .unwrap(),
        );

        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });

        let surface = instance.create_surface(window.clone()).unwrap();

        let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        }))
        .unwrap();

        let (device, queue) = pollster::block_on(adapter.request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
            },
            None,
        ))
        .unwrap();

        let mut size = window.inner_size();
        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);

        let mut config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width.max(1),
            height: size.height.max(1),
            present_mode: wgpu::PresentMode::AutoNoVsync,
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };
        surface.configure(&device, &config);

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        multisampled: false,
                        view_dimension: wgpu::TextureViewDimension::D2,
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
            label: Some("video_bind_group_layout"),
        });

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Video Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("video_shader.wgsl").into()),
        });

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[&bind_group_layout],
                push_constant_ranges: &[],
            });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                compilation_options: Default::default(),
                buffers: &[],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                compilation_options: Default::default(),
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: None,
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });

        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        let egui_ctx = egui::Context::default();
        let viewport_id = egui_ctx.viewport_id();
        let mut egui_state = egui_winit::State::new(
            egui_ctx.clone(),
            viewport_id,
            &window,
            Some(window.scale_factor() as f32),
            None,
        );

        let mut egui_renderer = egui_wgpu::Renderer::new(&device, config.format, None, 1);

        let (video_infos_local, audio_infos_local) =
            (self.video_infos.clone(), self.audio_infos.clone());
        let mut current_pipeline = self.pipeline;

        // Initialize UI State
        let mut overlay = Overlay::new();
        overlay.video_devices = video_infos_local.iter().map(|d| d.name.clone()).collect();
        overlay.audio_devices = audio_infos_local.iter().map(|d| d.name.clone()).collect();

        // Arka planda seçilen öncelikli cihazı arayüzde de işaretle
        overlay.selected_video_idx = video_infos_local
            .iter()
            .position(|d| d.internal_id == current_pipeline.video_device_id().unwrap_or_default());
        overlay.selected_audio_idx = audio_infos_local
            .iter()
            .position(|d| d.internal_id == current_pipeline.audio_device_id().unwrap_or_default());

        // Cihaz değişince caps listesini de doldur
        if let Some(idx) = overlay.selected_video_idx {
            overlay.supported_caps = video_infos_local[idx].caps.clone();
        }

        let mut receiver = current_pipeline.receiver();

        let mut video_texture: Option<wgpu::Texture> = None;
        let mut video_bind_group: Option<wgpu::BindGroup> = None;
        let mut video_size = (1920, 1080);
        let mut frame_counter = 0; // Real capture frame counter
        let mut last_click_time = std::time::Instant::now();
        let mut last_v_idx: Option<usize> = None; // Tracks last selected video device (replaces unsafe static mut)

        event_loop.run(move |event, elwt| {
            elwt.set_control_flow(ControlFlow::Poll);

            if let Event::WindowEvent {
                event: ref window_event,
                ..
            } = event
            {
                // Handle double-click to toggle UI
                if let winit::event::WindowEvent::MouseInput {
                    state: winit::event::ElementState::Pressed,
                    button: winit::event::MouseButton::Left,
                    ..
                } = window_event
                {
                    let now = std::time::Instant::now();
                    if now.duration_since(last_click_time) < std::time::Duration::from_millis(300) {
                        overlay.show_ui = !overlay.show_ui;
                    }
                    last_click_time = now;
                }

                let res = egui_state.on_window_event(&window, window_event);
                if res.consumed {
                    return;
                }

                // Handle ESC key to restore window from Stealth Mode
                if let winit::event::WindowEvent::KeyboardInput {
                    event:
                        winit::event::KeyEvent {
                            logical_key:
                                winit::keyboard::Key::Named(winit::keyboard::NamedKey::Escape),
                            state: winit::event::ElementState::Pressed,
                            ..
                        },
                    ..
                } = window_event
                {
                    if overlay.stealth_mode {
                        overlay.stealth_mode = false;
                        window.set_outer_position(winit::dpi::PhysicalPosition::new(100, 100));
                    }
                }
            }

            // Global Hotkey Checks (Win32 specific)
        #[cfg(target_os = "windows")]
        {
            unsafe {
                #[link(name = "user32")]
                unsafe extern "system" {
                    fn GetAsyncKeyState(v_key: i32) -> i16;
                }
                const VK_ESCAPE: i32 = 0x1B;
                const VK_F10: i32 = 0x79;
                const VK_LSHIFT: i32 = 0xA0;
                const VK_RSHIFT: i32 = 0xA1;

                let esc_pressed = (GetAsyncKeyState(VK_ESCAPE) as u16 & 0x8000) != 0;
                let f10_pressed = (GetAsyncKeyState(VK_F10) as u16 & 0x8000) != 0;
                let shift_pressed = ((GetAsyncKeyState(VK_LSHIFT) as u16 & 0x8000) != 0) 
                                 || ((GetAsyncKeyState(VK_RSHIFT) as u16 & 0x8000) != 0);
                
                static mut ESC_WAS_DOWN: bool = false;
                static mut F10_WAS_DOWN: bool = false;

                // 1. UI Toggle (Works ALWAYS)
                if shift_pressed && f10_pressed && !F10_WAS_DOWN {
                    overlay.show_ui = !overlay.show_ui;
                }
                F10_WAS_DOWN = f10_pressed;

                // 2. Stealth Exit (Works only in Stealth Mode)
                if overlay.stealth_mode && shift_pressed && esc_pressed && !ESC_WAS_DOWN {
                    overlay.stealth_mode = false;
                    
                    window.set_fullscreen(None);
                    window.set_window_level(winit::window::WindowLevel::Normal);
                    window.set_outer_position(winit::dpi::PhysicalPosition::new(100, 100));
                    let _ = window.request_inner_size(winit::dpi::PhysicalSize::new(1280, 720));

                    let hwnd = get_hwnd(&window);
                    if !hwnd.is_null() {
                        unsafe extern "system" {
                            fn GetWindowLongW(hwnd: *mut std::os::raw::c_void, index: i32) -> isize;
                            fn SetWindowLongW(hwnd: *mut std::os::raw::c_void, index: i32, new_long: isize) -> isize;
                        }
                        const GWL_EXSTYLE: i32 = -20;
                        const WS_EX_LAYERED: isize = 0x00080000;
                        const WS_EX_TRANSPARENT: isize = 0x00000020;

                        let mut style = GetWindowLongW(hwnd, GWL_EXSTYLE);
                        style &= !(WS_EX_LAYERED | WS_EX_TRANSPARENT);
                        SetWindowLongW(hwnd, GWL_EXSTYLE, style);
                    }
                    window.focus_window();
                }
                ESC_WAS_DOWN = esc_pressed;
            }
        }

        match event {
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => elwt.exit(),
                Event::WindowEvent {
                    event: WindowEvent::Resized(physical_size),
                    ..
                } => {
                    if physical_size.width > 0 && physical_size.height > 0 {
                        size = physical_size;
                        config.width = physical_size.width;
                        config.height = physical_size.height;
                        surface.configure(&device, &config);
                    }
                }
                Event::WindowEvent {
                    event: WindowEvent::RedrawRequested,
                    ..
                } => {
                    // Normal redraw path
                }
                Event::AboutToWait => {
                    if overlay.stealth_mode {
                        // Keep the loop spinning at max speed when hidden to prevent suspension
                        elwt.set_control_flow(winit::event_loop::ControlFlow::Poll);
                    } else {
                        // Standard energy-efficient wait mode
                        elwt.set_control_flow(winit::event_loop::ControlFlow::Wait);
                        window.request_redraw();
                    }
                }
                _ => {}
            }

            // --- HYBRID RENDER BLOCK ---
            // This runs on EVERY loop iteration where a render is needed.
            // In Stealth Mode, it runs on every loop to keep Discord updated.
            let should_render = match event {
                Event::WindowEvent { event: WindowEvent::RedrawRequested, .. } => true,
                Event::AboutToWait if overlay.stealth_mode => true,
                _ => false,
            };

            if should_render {
                // Handle fullscreen request from UI
                if overlay.fullscreen_requested {
                    overlay.fullscreen_requested = false;
                    let is_fullscreen = window.fullscreen().is_some();
                    if is_fullscreen {
                        window.set_fullscreen(None);
                    } else {
                        window.set_fullscreen(Some(winit::window::Fullscreen::Borderless(None)));
                    }
                }

                // Handle Stealth Mode request
                if overlay.stealth_mode_requested {
                    overlay.stealth_mode_requested = false;
                    overlay.stealth_mode = !overlay.stealth_mode;

                    if overlay.stealth_mode {
                        // Win32 Ghost Mode + Fullscreen Strategy
                        window.set_fullscreen(Some(winit::window::Fullscreen::Borderless(None)));
                        window.set_window_level(winit::window::WindowLevel::AlwaysOnBottom);

                        #[cfg(target_os = "windows")]
                        {
                            use std::os::raw::{c_int, c_void};
                            let hwnd = get_hwnd(&window);
                            
                            unsafe {
                                #[link(name = "user32")]
                                unsafe extern "system" {
                                    fn GetWindowLongW(hwnd: *mut c_void, index: c_int) -> isize;
                                    fn SetWindowLongW(hwnd: *mut c_void, index: c_int, new_long: isize) -> isize;
                                    fn SetLayeredWindowAttributes(hwnd: *mut c_void, key: u32, alpha: u8, flags: u32) -> i32;
                                }
                                const GWL_EXSTYLE: c_int = -20;
                                const WS_EX_LAYERED: isize = 0x00080000;
                                const WS_EX_TRANSPARENT: isize = 0x00000020;
                                const LWA_ALPHA: u32 = 0x2;

                                let mut style = GetWindowLongW(hwnd, GWL_EXSTYLE);
                                style |= WS_EX_LAYERED | WS_EX_TRANSPARENT;
                                SetWindowLongW(hwnd, GWL_EXSTYLE, style);
                                SetLayeredWindowAttributes(hwnd, 0, 1, LWA_ALPHA); 
                            }
                        }
                    } else {
                        // Restore: Exit Fullscreen and remove Ghost attributes
                        window.set_fullscreen(None);
                        window.set_window_level(winit::window::WindowLevel::Normal);
                        window.set_outer_position(winit::dpi::PhysicalPosition::new(100, 100));
                        let _ = window.request_inner_size(winit::dpi::PhysicalSize::new(1280, 720));

                        #[cfg(target_os = "windows")]
                        {
                            use std::os::raw::{c_int, c_void};
                            let hwnd = get_hwnd(&window);
                            
                            unsafe {
                                #[link(name = "user32")]
                                unsafe extern "system" {
                                    fn GetWindowLongW(hwnd: *mut c_void, index: c_int) -> isize;
                                    fn SetWindowLongW(hwnd: *mut c_void, index: c_int, new_long: isize) -> isize;
                                }
                                const GWL_EXSTYLE: c_int = -20;
                                const WS_EX_LAYERED: isize = 0x00080000;
                                const WS_EX_TRANSPARENT: isize = 0x00000020;

                                let mut style = GetWindowLongW(hwnd, GWL_EXSTYLE);
                                style &= !(WS_EX_LAYERED | WS_EX_TRANSPARENT);
                                SetWindowLongW(hwnd, GWL_EXSTYLE, style);
                            }
                        }
                        window.focus_window();
                    }
                }

                // Handle Device Switch request
                if overlay.device_switch_requested {
                    overlay.device_switch_requested = false;

                    let v_id = overlay.selected_video_idx.and_then(|idx| {
                        video_infos_local.get(idx).map(|d| d.internal_id.clone())
                    });

                    // Detect if Video changed to trigger Auto-Match
                    let mut a_id = overlay.selected_audio_idx.and_then(|idx| {
                        audio_infos_local.get(idx).map(|d| d.internal_id.clone())
                    });

                    if last_v_idx != overlay.selected_video_idx {
                        // Video changed! Trigger Auto-Match
                        if let Some(ref v_name) = v_id {
                            if let Some(matched_id) = Self::match_audio_for_video(
                                v_name,
                                &audio_infos_local,
                                self.log_sender.clone(),
                            ) {
                                // Update overlay to reflect matched audio
                                overlay.selected_audio_idx = audio_infos_local
                                    .iter()
                                    .position(|d| d.internal_id == matched_id);
                                a_id = Some(matched_id);
                            } else {
                                overlay.selected_audio_idx = None;
                                a_id = None;
                            }
                        }
                        last_v_idx = overlay.selected_video_idx;
                    }

                    // Sync caps list when video device changes
                    if let Some(idx) = overlay.selected_video_idx {
                        let new_caps = video_infos_local[idx].caps.clone();
                        if overlay.supported_caps != new_caps {
                            overlay.supported_caps = new_caps;
                            overlay.selected_cap_idx = None; // Reset to Auto
                        }
                    }

                    // Parse selected resolution and FPS
                    let mut res = None;
                    let mut fps = None;
                    if let Some(cap_idx) = overlay.selected_cap_idx {
                        if let Some(cap_str) = overlay.supported_caps.get(cap_idx) {
                            // Format: "1920x1080 @ 60fps"
                            let parts: Vec<&str> = cap_str.split(" @ ").collect();
                            if parts.len() == 2 {
                                let dims: Vec<&str> = parts[0].split('x').collect();
                                if dims.len() == 2 {
                                    if let (Ok(w), Ok(h)) =
                                        (dims[0].parse::<i32>(), dims[1].parse::<i32>())
                                    {
                                        res = Some((w, h));
                                    }
                                }
                                let fps_val =
                                    parts[1].replace("fps", "").parse::<i32>().unwrap_or(0);
                                fps = Some(fps_val);
                            }
                        }
                    }

                    println!(
                        "Switching to Video: {:?}, Audio: {:?}, Res: {:?}, FPS: {:?}",
                        v_id, a_id, res, fps
                    );

                    let fmt_str = match overlay.selected_format_idx {
                        1 => Some("MJPG"),
                        2 => Some("YUY2"),
                        3 => Some("NV12"),
                        _ => None,
                    };

                    match PipelineController::new(
                        v_id,
                        a_id,
                        res,
                        fps,
                        fmt_str,
                        self.log_sender.clone(),
                    ) {
                        Ok(new_pipeline) => {
                            current_pipeline = new_pipeline;
                            receiver = current_pipeline.receiver();
                            overlay.add_log("Pipeline successfully restarted with new settings.");
                        }
                        Err(e) => {
                            overlay.add_log(format!("Failed to switch settings: {}", e));
                        }
                    }
                }

                // Update Volume
                let current_vol = if overlay.is_muted { 0.0 } else { overlay.volume as f64 };
                current_pipeline.update_adjustments(
                    overlay.brightness,
                    overlay.contrast,
                    overlay.saturation,
                    overlay.hue,
                    current_vol as f32,
                    overlay.is_muted,
                );

                // Drain logs from channel to UI
                while let Ok(log_msg) = self.log_receiver.try_recv() {
                    overlay.add_log(log_msg);
                }

                // Get the latest frame, discarding older ones in the channel
                let mut latest_frame = None;
                while let Ok(frame) = receiver.try_recv() {
                    latest_frame = Some(frame);
                    frame_counter += 1; // Count real received frames
                }

                // Update FPS in overlay
                if overlay.update_fps(frame_counter) {
                    frame_counter = 0;
                }

                if let Some(frame) = latest_frame {
                    let texture_size = wgpu::Extent3d {
                        width: frame.width,
                        height: frame.height,
                        depth_or_array_layers: 1,
                    };

                    video_size = (frame.width, frame.height);

                    let needs_recreate = video_texture
                        .as_ref()
                        .map_or(true, |t| t.width() != frame.width || t.height() != frame.height);

                    if needs_recreate {
                        let texture = device.create_texture(&wgpu::TextureDescriptor {
                            size: texture_size,
                            mip_level_count: 1,
                            sample_count: 1,
                            dimension: wgpu::TextureDimension::D2,
                            format: wgpu::TextureFormat::Rgba8UnormSrgb,
                            usage: wgpu::TextureUsages::TEXTURE_BINDING
                                | wgpu::TextureUsages::COPY_DST,
                            label: Some("video_texture"),
                            view_formats: &[],
                        });

                        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
                        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
                            layout: &bind_group_layout,
                            entries: &[
                                wgpu::BindGroupEntry {
                                    binding: 0,
                                    resource: wgpu::BindingResource::TextureView(&view),
                                },
                                wgpu::BindGroupEntry {
                                    binding: 1,
                                    resource: wgpu::BindingResource::Sampler(&sampler),
                                },
                            ],
                            label: Some("video_bind_group"),
                        });

                        video_texture = Some(texture);
                        video_bind_group = Some(bind_group);
                    }

                    if let Some(texture) = &video_texture {
                        queue.write_texture(
                            wgpu::ImageCopyTexture {
                                texture,
                                mip_level: 0,
                                origin: wgpu::Origin3d::ZERO,
                                aspect: wgpu::TextureAspect::All,
                            },
                            &frame.data,
                            wgpu::ImageDataLayout {
                                offset: 0,
                                bytes_per_row: Some(4 * frame.width),
                                rows_per_image: Some(frame.height),
                            },
                            texture_size,
                        );
                    }
                }

                let output = match surface.get_current_texture() {
                    Ok(texture) => texture,
                    Err(wgpu::SurfaceError::Outdated | wgpu::SurfaceError::Lost) => {
                        surface.configure(&device, &config);
                        return;
                    }
                    Err(e) => {
                        log::error!("Surface error: {:?}", e);
                        return;
                    }
                };

                let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
                let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("Render Encoder"),
                });

                if let Some(bind_group) = &video_bind_group {
                    let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                        label: Some("Video Render Pass"),
                        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                            view: &view,
                            resolve_target: None,
                            ops: wgpu::Operations {
                                load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                                store: wgpu::StoreOp::Store,
                            },
                        })],
                        depth_stencil_attachment: None,
                        timestamp_writes: None,
                        occlusion_query_set: None,
                    });

                    let window_aspect = size.width as f32 / size.height as f32;
                    let video_aspect = video_size.0 as f32 / video_size.1 as f32;

                    let mut vp_w = size.width as f32;
                    let mut vp_h = size.height as f32;
                    let mut vp_x = 0.0;
                    let mut vp_y = 0.0;

                    if window_aspect > video_aspect {
                        vp_w = vp_h * video_aspect;
                        vp_x = (size.width as f32 - vp_w) / 2.0;
                    } else {
                        vp_h = vp_w / video_aspect;
                        vp_y = (size.height as f32 - vp_h) / 2.0;
                    }

                    render_pass.set_viewport(vp_x, vp_y, vp_w, vp_h, 0.0, 1.0);
                    render_pass.set_pipeline(&render_pipeline);
                    render_pass.set_bind_group(0, bind_group, &[]);
                    render_pass.draw(0..3, 0..1);
                } else {
                    encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                        label: Some("Clear Pass"),
                        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                            view: &view,
                            resolve_target: None,
                            ops: wgpu::Operations {
                                load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                                store: wgpu::StoreOp::Store,
                            },
                        })],
                        depth_stencil_attachment: None,
                        timestamp_writes: None,
                        occlusion_query_set: None,
                    });
                }

                let raw_input = egui_state.take_egui_input(&window);
                let full_output = egui_ctx.run(raw_input, |ctx| {
                    overlay.render(ctx, video_size);
                });

                egui_state.handle_platform_output(&window, full_output.platform_output);

                let clipped_primitives =
                    egui_ctx.tessellate(full_output.shapes, full_output.pixels_per_point);
                for (id, image_delta) in &full_output.textures_delta.set {
                    egui_renderer.update_texture(&device, &queue, *id, image_delta);
                }

                let screen_descriptor = egui_wgpu::ScreenDescriptor {
                    size_in_pixels: [config.width, config.height],
                    pixels_per_point: window.scale_factor() as f32,
                };

                egui_renderer.update_buffers(
                    &device,
                    &queue,
                    &mut encoder,
                    &clipped_primitives,
                    &screen_descriptor,
                );

                {
                    let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                        label: Some("egui render pass"),
                        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                            view: &view,
                            resolve_target: None,
                            ops: wgpu::Operations {
                                load: wgpu::LoadOp::Load,
                                store: wgpu::StoreOp::Store,
                            },
                        })],
                        depth_stencil_attachment: None,
                        timestamp_writes: None,
                        occlusion_query_set: None,
                    });

                    egui_renderer.render(
                        &mut render_pass,
                        &clipped_primitives,
                        &screen_descriptor,
                    );
                }

                for id in &full_output.textures_delta.free {
                    egui_renderer.free_texture(id);
                }

                queue.submit(std::iter::once(encoder.finish()));
                output.present();
            }
        })?;
        Ok(())
    }
}
