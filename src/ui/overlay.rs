use egui::{Color32, RichText, Rounding, Stroke, Vec2};
use std::time::{Duration, Instant};

pub struct Overlay {
    last_frame_time: Instant,
    pub fps: f32,
    pub show_ui: bool,
    pub fullscreen_requested: bool,
    pub stealth_mode: bool,
    pub stealth_mode_requested: bool,
    pub volume: f32,
    pub brightness: f32,
    pub contrast: f32,
    pub saturation: f32,
    pub hue: f32,
    pub is_muted: bool,
    pub video_devices: Vec<String>,
    pub audio_devices: Vec<String>,
    pub supported_caps: Vec<String>,
    pub selected_video_idx: Option<usize>,
    pub selected_audio_idx: Option<usize>,
    pub selected_cap_idx: Option<usize>,
    pub selected_format_idx: usize, // 0: Auto, 1: MJPG, 2: YUY2
    pub device_switch_requested: bool,
    pub logs: Vec<String>,
    pub show_logs: bool,
}

impl Overlay {
    pub fn new() -> Self {
        Self {
            last_frame_time: Instant::now(),
            fps: 0.0,
            show_ui: true,
            fullscreen_requested: false,
            stealth_mode: false,
            stealth_mode_requested: false,
            volume: 1.0,
            brightness: 0.0,
            contrast: 1.0,
            saturation: 1.0,
            hue: 0.0,
            is_muted: false,
            video_devices: Vec::new(),
            audio_devices: Vec::new(),
            supported_caps: Vec::new(),
            selected_video_idx: None,
            selected_audio_idx: None,
            selected_cap_idx: None,
            selected_format_idx: 0,
            device_switch_requested: false,
            logs: vec!["Welcome to Kaptura v1.0".to_string()],
            show_logs: false,
        }
    }

    pub fn update_fps(&mut self, frame_count: u32) -> bool {
        let now = Instant::now();
        let elapsed = now.duration_since(self.last_frame_time);
        if elapsed >= Duration::from_secs(1) {
            self.fps = frame_count as f32 / elapsed.as_secs_f32();
            self.last_frame_time = now;
            return true;
        }
        false
    }

    pub fn add_log(&mut self, msg: impl Into<String>) {
        let msg_str = msg.into();
        self.logs.push(msg_str);
        if self.logs.len() > 100 {
            self.logs.remove(0);
        }
    }

    pub fn render(&mut self, ctx: &egui::Context, video_size: (u32, u32)) {
        if !self.show_ui {
            return;
        }

        // Apply a dark theme with glassmorphism touches
        let mut visuals = egui::Visuals::dark();
        visuals.window_fill = Color32::from_rgba_premultiplied(20, 20, 25, 220);
        visuals.window_stroke = Stroke::new(1.0, Color32::from_rgba_premultiplied(100, 100, 110, 80));
        visuals.window_rounding = Rounding::same(12.0);
        ctx.set_visuals(visuals);

        // Bento Box 1: Status Overlay
        egui::Window::new("Status")
            .title_bar(false)
            .resizable(false)
            .collapsible(false)
            .anchor(egui::Align2::LEFT_TOP, Vec2::new(20.0, 20.0))
            .show(ctx, |ui| {
                ui.add_space(8.0);
                ui.horizontal(|ui| {
                    ui.heading(RichText::new("KAPTURA").strong().color(Color32::from_rgb(0, 200, 255)).size(20.0));
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button(RichText::new("⛶").size(16.0)).on_hover_text("Toggle Fullscreen").clicked() {
                            self.fullscreen_requested = true;
                        }
                    });
                });

                // Video Device Selection
                ui.label(RichText::new("VIDEO SOURCE").color(Color32::GRAY).size(10.0));
                let prev_v = self.selected_video_idx;
                egui::ComboBox::from_id_source("v_dev")
                    .width(180.0)
                    .selected_text(self.selected_video_idx.and_then(|idx| self.video_devices.get(idx)).unwrap_or(&"Select Device".to_string()))
                    .show_ui(ui, |ui| {
                        for (i, dev) in self.video_devices.iter().enumerate() {
                            ui.selectable_value(&mut self.selected_video_idx, Some(i), dev);
                        }
                    });
                if prev_v != self.selected_video_idx { self.device_switch_requested = true; }

                ui.add_space(8.0);

                // Audio Device Selection
                ui.label(RichText::new("AUDIO SOURCE").color(Color32::GRAY).size(10.0));
                let prev_a = self.selected_audio_idx;
                egui::ComboBox::from_id_source("a_dev")
                    .width(180.0)
                    .selected_text(self.selected_audio_idx.and_then(|idx| self.audio_devices.get(idx)).unwrap_or(&"No Audio Output".to_string()))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.selected_audio_idx, None, "None");
                        for (i, dev) in self.audio_devices.iter().enumerate() {
                            ui.selectable_value(&mut self.selected_audio_idx, Some(i), dev);
                        }
                    });
                if prev_a != self.selected_audio_idx { self.device_switch_requested = true; }

                ui.add_space(8.0);

                // Resolution / FPS Selection
                ui.label(RichText::new("RESOLUTION / FPS").color(Color32::GRAY).size(10.0));
                let prev_c = self.selected_cap_idx;
                egui::ComboBox::from_id_source("cap_dev")
                    .width(180.0)
                    .selected_text(self.selected_cap_idx.and_then(|idx| self.supported_caps.get(idx)).unwrap_or(&"Auto".to_string()))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.selected_cap_idx, None, "Auto");
                        for (i, cap) in self.supported_caps.iter().enumerate() {
                            ui.selectable_value(&mut self.selected_cap_idx, Some(i), cap);
                        }
                    });
                if prev_c != self.selected_cap_idx { self.device_switch_requested = true; }

                ui.add_space(8.0);

                // Pixel Format Selection
                ui.label(RichText::new("PIXEL FORMAT").color(Color32::GRAY).size(10.0));
                let prev_f = self.selected_format_idx;
                egui::ComboBox::from_id_source("fmt_dev")
                    .width(180.0)
                    .selected_text(match self.selected_format_idx {
                        1 => "MJPG",
                        2 => "YUY2",
                        _ => "Auto",
                    })
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.selected_format_idx, 0, "Auto");
                        ui.selectable_value(&mut self.selected_format_idx, 1, "MJPG");
                        ui.selectable_value(&mut self.selected_format_idx, 2, "YUY2");
                    });
                if prev_f != self.selected_format_idx { self.device_switch_requested = true; }

                ui.add_space(12.0);
                
                ui.horizontal(|ui| {
                    ui.label(RichText::new("RESOLUTION").color(Color32::GRAY).size(10.0));
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.label(RichText::new(format!("{}x{}", video_size.0, video_size.1)).strong().color(Color32::WHITE));
                    });
                });
                
                ui.horizontal(|ui| {
                    ui.label(RichText::new("CAPTURE FPS").color(Color32::GRAY).size(10.0));
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.label(RichText::new(format!("{:.1}", self.fps)).strong().color(Color32::from_rgb(100, 255, 100)));
                    });
                });

                ui.add_space(8.0);
            });

        // Bento Box 2: Controls Overlay
        egui::Window::new("Controls")
            .title_bar(false)
            .resizable(false)
            .collapsible(false)
            .anchor(egui::Align2::RIGHT_BOTTOM, Vec2::new(-20.0, -20.0))
            .show(ctx, |ui| {
                ui.add_space(8.0);
                ui.heading(RichText::new("AUDIO").strong().color(Color32::WHITE).size(14.0));
                ui.add_space(8.0);
                ui.horizontal(|ui| {
                    ui.set_min_width(200.0);
                    let icon = if self.is_muted || self.volume == 0.0 { "🔇" } else if self.volume < 0.5 { "🔉" } else { "🔊" };
                    if ui.button(RichText::new(icon).size(16.0)).clicked() {
                        self.is_muted = !self.is_muted;
                    }
                    ui.add_space(8.0);
                    ui.add(egui::Slider::new(&mut self.volume, 0.0..=1.0).show_value(false).trailing_fill(true));
                    ui.add_space(8.0);
                    ui.label(RichText::new(format!("{:.0}%", self.volume * 100.0)).monospace().color(Color32::GRAY));
                });

                ui.add_space(16.0);
                ui.separator();
                ui.add_space(8.0);

                ui.horizontal(|ui| {
                    ui.heading(RichText::new("ADJUSTMENTS").strong().color(Color32::WHITE).size(14.0));
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        let text = if self.stealth_mode { "Stealth Active" } else { "Stealth Mode" };
                        let color = if self.stealth_mode { Color32::from_rgb(255, 100, 100) } else { Color32::from_rgb(0, 200, 255) };
                        
                        if ui.button(RichText::new(text).color(color).strong()).clicked() {
                            self.stealth_mode_requested = true;
                        }

                        ui.add_space(8.0);

                        let log_btn_text = if self.show_logs { "Hide Logs" } else { "Show Logs" };
                        if ui.button(RichText::new(log_btn_text).color(Color32::from_rgb(150, 150, 160))).clicked() {
                            self.show_logs = !self.show_logs;
                        }
                    });
                });
                ui.add_space(12.0);

                let add_control = |ui: &mut egui::Ui, label: &str, value: &mut f32, range: std::ops::RangeInclusive<f32>, default: f32| {
                    ui.horizontal(|ui| {
                        ui.set_min_width(200.0);
                        ui.label(RichText::new(label).color(Color32::LIGHT_GRAY).size(12.0));
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if ui.button("⟲").on_hover_text("Reset").clicked() {
                                *value = default;
                            }
                            ui.add_space(8.0);
                            ui.label(RichText::new(format!("{:.1}", *value)).monospace().color(Color32::GRAY));
                            ui.add_space(8.0);
                            ui.add(egui::Slider::new(value, range).show_value(false).trailing_fill(true));
                        });
                    });
                    ui.add_space(6.0);
                };

                add_control(ui, "Brightness", &mut self.brightness, -1.0..=1.0, 0.0);
                add_control(ui, "Contrast", &mut self.contrast, 0.0..=2.0, 1.0);
                add_control(ui, "Saturation", &mut self.saturation, 0.0..=2.0, 1.0);
                add_control(ui, "Hue", &mut self.hue, -1.0..=1.0, 0.0);
                
                ui.add_space(8.0);
            });

        // Log Window
        if self.show_logs {
            egui::Window::new("System Logs")
                .collapsible(false)
                .resizable(true)
                .default_size([400.0, 300.0])
                .show(ctx, |ui| {
                    egui::ScrollArea::vertical()
                        .stick_to_bottom(true)
                        .show(ui, |ui| {
                            for log in &self.logs {
                                ui.label(RichText::new(log).monospace().size(11.0).color(Color32::from_rgb(180, 180, 200)));
                            }
                        });
                });
        }
    }
}
