use egui::{Color32, RichText, Rounding, Stroke, Vec2};
use std::time::{Duration, Instant};

pub struct Overlay {
    last_frame_time: Instant,
    pub fps: f32,
    pub show_ui: bool,
    pub fullscreen_requested: bool,
    pub stealth_mode: bool,
    pub stealth_mode_requested: bool,
    pub audio_only: bool,
    pub volume: f32,
    pub brightness: f32,
    pub contrast: f32,
    pub saturation: f32,
    pub hue: f32,
    pub is_muted: bool,
    pub sharpening_amount: f32,
    pub use_sharpen: bool,
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
            audio_only: false,
            volume: 1.0,
            brightness: 0.0,
            contrast: 1.0,
            saturation: 1.0,
            hue: 0.0,
            is_muted: false,
            sharpening_amount: 0.0,
            use_sharpen: false,
            video_devices: Vec::new(),
            audio_devices: Vec::new(),
            supported_caps: Vec::new(),
            selected_video_idx: None,
            selected_audio_idx: None,
            selected_cap_idx: None,
            selected_format_idx: 0,
            device_switch_requested: false,
            logs: vec!["Welcome to Kaptura v1.1.0".to_string()],
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

    pub fn render(&mut self, ctx: &egui::Context, _video_size: (u32, u32)) {
        if !self.show_ui {
            return;
        }

        // Apply a modern dark theme with glassmorphism touches
        let mut visuals = egui::Visuals::dark();
        visuals.window_fill = Color32::from_rgba_premultiplied(15, 15, 20, 230); // Darker, sleeker glass
        visuals.window_stroke = Stroke::new(1.0, Color32::from_rgba_premultiplied(80, 80, 90, 100));
        visuals.window_rounding = Rounding::same(16.0); // Smoother corners

        let mut style = egui::Style::default();
        style.visuals = visuals;
        style.spacing.item_spacing = Vec2::new(10.0, 10.0);
        style.spacing.window_margin = egui::Margin::same(16.0);
        ctx.set_style(style);

        // Bento Box 1: Status Overlay (Top Left)
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
                ui.label(RichText::new("v1.2.0 - Video Capture").color(Color32::GRAY).size(10.0));
                
                ui.add_space(12.0);

                // Video Device Selection
                ui.label(RichText::new("VIDEO SOURCE").color(Color32::GRAY).size(10.0));
                let prev_v = self.selected_video_idx;
                egui::ComboBox::from_id_source("v_dev")
                    .width(220.0)
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
                    .width(220.0)
                    .selected_text(self.selected_audio_idx.and_then(|idx| self.audio_devices.get(idx)).unwrap_or(&"No Audio Output".to_string()))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.selected_audio_idx, None, "None");
                        for (i, dev) in self.audio_devices.iter().enumerate() {
                            ui.selectable_value(&mut self.selected_audio_idx, Some(i), dev);
                        }
                    });
                if prev_a != self.selected_audio_idx { self.device_switch_requested = true; }

                ui.add_space(8.0);

                // NEW: Restore Resolution / FPS Selection
                ui.label(RichText::new("RESOLUTION / FPS").color(Color32::GRAY).size(10.0));
                let prev_c = self.selected_cap_idx;
                egui::ComboBox::from_id_source("cap_dev")
                    .width(220.0)
                    .selected_text(self.selected_cap_idx.and_then(|idx| self.supported_caps.get(idx)).unwrap_or(&"Auto".to_string()))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.selected_cap_idx, None, "Auto");
                        for (i, cap) in self.supported_caps.iter().enumerate() {
                            ui.selectable_value(&mut self.selected_cap_idx, Some(i), cap);
                        }
                    });
                if prev_c != self.selected_cap_idx { self.device_switch_requested = true; }

                ui.add_space(8.0);

                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        ui.label(RichText::new("PIXEL FORMAT").color(Color32::GRAY).size(10.0));
                        let prev_f = self.selected_format_idx;
                        egui::ComboBox::from_id_source("fmt_dev")
                            .width(100.0)
                            .selected_text(match self.selected_format_idx {
                                1 => "MJPG", 2 => "YUY2", 3 => "NV12", _ => "Auto",
                            })
                            .show_ui(ui, |ui| {
                                ui.selectable_value(&mut self.selected_format_idx, 0, "Auto");
                                ui.selectable_value(&mut self.selected_format_idx, 1, "MJPG");
                                ui.selectable_value(&mut self.selected_format_idx, 2, "YUY2");
                                ui.selectable_value(&mut self.selected_format_idx, 3, "NV12");
                            });
                        if prev_f != self.selected_format_idx { self.device_switch_requested = true; }
                    });

                    ui.add_space(12.0);

                    ui.vertical(|ui| {
                        ui.label(RichText::new("CURRENT FPS").color(Color32::GRAY).size(10.0));
                        ui.add_space(4.0);
                        ui.label(RichText::new(format!("{:.1}", self.fps)).strong().color(Color32::from_rgb(100, 255, 100)));
                    });
                });

                ui.add_space(8.0);
            });

        // Bento Box 2: Adjustments & Modes (Bottom Right)
        egui::Window::new("Adjustments")
            .title_bar(false)
            .resizable(false)
            .collapsible(false)
            .anchor(egui::Align2::RIGHT_BOTTOM, Vec2::new(-20.0, -20.0))
            .show(ctx, |ui| {
                ui.add_space(8.0);
                
                // Volume
                ui.horizontal(|ui| {
                    ui.set_min_width(280.0);
                    let icon = if self.is_muted || self.volume == 0.0 { "🔇" } else if self.volume < 0.5 { "🔉" } else { "🔊" };
                    if ui.button(RichText::new(icon).size(16.0)).clicked() {
                        self.is_muted = !self.is_muted;
                    }
                    ui.add_space(8.0);
                    ui.add(egui::Slider::new(&mut self.volume, 0.0..=1.0).show_value(false).trailing_fill(true));
                    ui.add_space(8.0);
                    ui.label(RichText::new(format!("{:.0}%", self.volume * 100.0)).monospace().color(Color32::GRAY));
                });

                ui.add_space(12.0);
                ui.separator();
                ui.add_space(8.0);

                // --- ACTION GRID with Improved Contrast ---
                ui.columns(3, |cols| {
                    // Logs: Bright Gray vs Dark Gray
                    let log_color = if self.show_logs { Color32::WHITE } else { Color32::from_rgb(160, 160, 170) };
                    if cols[0].add_sized([cols[0].available_width(), 30.0], egui::Button::new(RichText::new("LOGS").size(11.0).strong().color(log_color))).clicked() {
                        self.show_logs = !self.show_logs;
                    }

                    // Audio Only: Neon Purple vs Muted Purple
                    let audio_color = if self.audio_only { Color32::from_rgb(220, 180, 255) } else { Color32::from_rgb(140, 120, 160) };
                    if cols[1].add_sized([cols[1].available_width(), 30.0], egui::Button::new(RichText::new("AUDIO").size(11.0).strong().color(audio_color))).clicked() {
                        self.audio_only = !self.audio_only;
                    }

                    // Stealth: Neon Cyan vs Muted Cyan
                    let stealth_color = if self.stealth_mode { Color32::from_rgb(255, 120, 120) } else { Color32::from_rgb(100, 220, 255) };
                    if cols[2].add_sized([cols[2].available_width(), 30.0], egui::Button::new(RichText::new("STEALTH").size(11.0).strong().color(stealth_color))).clicked() {
                        self.stealth_mode_requested = true;
                    }
                });

                ui.add_space(12.0);
                ui.separator();
                ui.add_space(8.0);

                let add_control = |ui: &mut egui::Ui, label: &str, value: &mut f32, range: std::ops::RangeInclusive<f32>, default: f32| {
                    ui.horizontal(|ui| {
                        ui.set_min_width(280.0);
                        ui.label(RichText::new(label).color(Color32::LIGHT_GRAY).size(12.0));
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if ui.button("⟲").on_hover_text("Reset").clicked() {
                                *value = default;
                            }
                            ui.add_space(8.0);
                            
                            // Interactive DragValue for manual input
                            ui.add(egui::DragValue::new(value)
                                .speed(0.01)
                                .range(range.clone())
                                .fixed_decimals(1)
                            );
                            
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
                ui.horizontal(|ui| {
                    ui.checkbox(&mut self.use_sharpen, RichText::new("Use Sharpening").color(Color32::WHITE).size(12.0));
                    if self.use_sharpen {
                        ui.add(egui::Slider::new(&mut self.sharpening_amount, 0.0..=1.0).show_value(false).trailing_fill(true));
                        ui.add_space(8.0);
                        // Interactive DragValue for Sharpening
                        ui.add(egui::DragValue::new(&mut self.sharpening_amount)
                            .speed(0.01)
                            .range(0.0..=1.0)
                            .fixed_decimals(2)
                        );
                    }
                });

                ui.add_space(16.0);
                ui.separator();
                ui.add_space(8.0);

                ui.heading(RichText::new("SHORTCUTS").strong().color(Color32::WHITE).size(14.0));
                ui.add_space(6.0);
                
                let add_hotkey = |ui: &mut egui::Ui, key: &str, action: &str| {
                    ui.horizontal(|ui| {
                        ui.label(RichText::new(key).strong().color(Color32::from_rgb(0, 200, 255)).size(11.0));
                        ui.label(RichText::new(action).color(Color32::GRAY).size(11.0));
                    });
                };
                
                add_hotkey(ui, "Shift + Esc", "- Exit Stealth Mode");
                add_hotkey(ui, "Shift + F10", "- Toggle UI (Global)");
                add_hotkey(ui, "Double Click", "- Toggle UI (Normal)");

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
