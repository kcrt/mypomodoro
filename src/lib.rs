#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui::{self, epaint::{PathShape, Shape, Stroke, PathStroke}, Pos2, Vec2, Color32, Align2, FontId}; 
use chrono::{DateTime, Utc};
use std::f32::consts::TAU;

// Constants for magic numbers
const RADIUS_SCALE: f32 = 0.8;
const STROKE_WIDTH_RATIO: f32 = 0.25;
const REPAINT_INTERVAL_MS: u64 = 100;
const BELL_VOLUME: f32 = 0.1;
const FONT_SIZE_RATIO: f32 = 0.5;
const QUARTER_TURN: f32 = TAU / 4.0;
const MS_PER_MINUTE: f32 = 60_000.0;
const SECONDS_PER_MINUTE: f32 = 60.0;
const CIRCLE_POINTS: i32 = 100;
const ICON_SIZE: u32 = 64;
const TIMER_DISPLAY_SIZE: f32 = 250.0;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TimerState {
    Running,
    Paused,
    Stopped,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TimerPhase {
    Pomodoro,
    ShortBreak,
    LongBreak,
}

pub struct MyApp {
    pomodoro_min: f32,
    short_break_min: f32,
    cycles: i32,
    long_break_min: f32,
    play_sound: bool,

    timer_state: TimerState,
    current_phase: TimerPhase,
    current_cycle: i32,
    phase_start_time: Option<DateTime<Utc>>,
    pause_delta_min: f32, // Time spent in pause
    pause_start_time: Option<DateTime<Utc>>,
    last_remaining_time_seconds: i32,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            pomodoro_min: 25.0,
            short_break_min: 5.0,
            cycles: 4,
            long_break_min: 15.0,
            play_sound: true,
            timer_state: TimerState::Stopped,
            current_phase: TimerPhase::Pomodoro,
            current_cycle: 0,
            phase_start_time: None,
            pause_delta_min: 0.0,
            pause_start_time: None,
            last_remaining_time_seconds: 0,
        }
    }
}

impl MyApp {
    pub fn timer_state(&self) -> TimerState {
        self.timer_state
    }
    
    pub fn current_phase(&self) -> TimerPhase {
        self.current_phase
    }
    
    pub fn current_cycle(&self) -> i32 {
        self.current_cycle
    }
    
    pub fn pomodoro_min(&self) -> f32 {
        self.pomodoro_min
    }
    
    pub fn short_break_min(&self) -> f32 {
        self.short_break_min
    }
    
    pub fn long_break_min(&self) -> f32 {
        self.long_break_min
    }
    
    pub fn cycles(&self) -> i32 {
        self.cycles
    }
    
    pub fn play_sound(&self) -> bool {
        self.play_sound
    }
    
    pub fn phase_start_time(&self) -> Option<DateTime<Utc>> {
        self.phase_start_time
    }
    
    pub fn pause_start_time(&self) -> Option<DateTime<Utc>> {
        self.pause_start_time
    }
    
    pub fn pause_delta_min(&self) -> f32 {
        self.pause_delta_min
    }

    pub fn get_current_phase_duration_minutes(&self) -> f32 {
        let duration = self.get_phase_duration_minutes(self.current_phase);
        assert!(duration != 0.0, "Phase duration cannot be zero");
        duration
    }

    pub fn get_phase_duration_minutes(&self, phase: TimerPhase) -> f32 {
        match phase {
            TimerPhase::Pomodoro => self.pomodoro_min,
            TimerPhase::ShortBreak => self.short_break_min,
            TimerPhase::LongBreak => self.long_break_min,
        }
    }

    /// Returns the time spent in the current phase in minutes, ignoring any pauses.
    pub fn get_spent_time_minutes(&self) -> f32 {
        if let Some(phase_start_time) = self.phase_start_time {
            if self.timer_state == TimerState::Paused {
                if let Some(pause_start_time) = self.pause_start_time {
                    let elapsed_duration = pause_start_time.signed_duration_since(phase_start_time).num_milliseconds() as f32 / MS_PER_MINUTE;
                    return elapsed_duration - self.pause_delta_min;
                } else {
                    return 0.0;
                }
            } else {
                let elapsed_total = Utc::now().signed_duration_since(phase_start_time).num_milliseconds() as f32 / MS_PER_MINUTE;
                return elapsed_total - self.pause_delta_min;
            }
        }
        0.0
    }
    
    /// Returns the remaining time in the current phase in minutes.
    pub fn get_remaining_time_minutes(&self) -> f32 {
        let spent_time = self.get_spent_time_minutes();
        let total_duration = self.get_current_phase_duration_minutes();
        total_duration - spent_time
    }

    pub fn begin_timer(&mut self) {
        self.begin_timer_with_phase(TimerPhase::Pomodoro);
    }
    
    pub fn begin_timer_with_phase(&mut self, phase: TimerPhase) {
        self.timer_state = TimerState::Running;
        self.begin_phase(phase);
    }
    
    /// Starts a new timer phase, resetting pause state and setting start time.
    pub fn begin_phase(&mut self, phase: TimerPhase) {
        self.current_phase = phase;
        self.phase_start_time = Some(Utc::now());
        self.pause_delta_min = 0.0;
    }
    
    pub fn pause_timer(&mut self) {
        assert!(self.timer_state == TimerState::Running, "Cannot pause a timer that is not running.");
        self.pause_start_time = Some(Utc::now());
        self.timer_state = TimerState::Paused;
    }
    
    pub fn resume_timer(&mut self) {
        assert!(self.timer_state == TimerState::Paused, "Cannot resume a timer that is not paused.");
        if let Some(start_time) = self.pause_start_time {
            let pause_duration_min = Utc::now().signed_duration_since(start_time).num_milliseconds() as f32 / MS_PER_MINUTE;
            self.pause_delta_min += pause_duration_min;
            self.pause_start_time = None;
        }
        self.timer_state = TimerState::Running;
    }
    
    pub fn reset_timer(&mut self) {
        self.timer_state = TimerState::Stopped;
        self.phase_start_time = None;
        self.pause_start_time = None;
        self.pause_delta_min = 0.0;
        self.current_cycle = 0;
        self.current_phase = TimerPhase::Pomodoro;
    }

    /// Plays the bell sound notification in a separate thread to avoid blocking the UI.
    /// Only plays if `play_sound` is enabled.
    pub fn play_bell_sound(&self) {
        if self.play_sound {
            let bell_data = include_bytes!("../resources/bell.mp3");
            
            std::thread::spawn(move || {
                use std::io::Cursor;
                use rodio::{Decoder, OutputStream, Sink};
                
                if let Ok((_stream, stream_handle)) = OutputStream::try_default() {
                    if let Ok(sink) = Sink::try_new(&stream_handle) {
                        if let Ok(source) = Decoder::new(Cursor::new(bell_data)) {
                            sink.append(source);
                            sink.set_volume(BELL_VOLUME); // Set smaller volume
                            sink.sleep_until_end();
                        }
                    }
                }
            });
        }
    }
    
    /// Advances to the next phase in the pomodoro cycle.
    /// Handles transitions between pomodoro, short break, and long break phases.
    pub fn next_phase(&mut self) {
        let next_phase;
        match self.current_phase {
            TimerPhase::Pomodoro => {
                self.current_cycle += 1;
                if self.current_cycle >= self.cycles {
                    next_phase = TimerPhase::LongBreak;
                    self.current_cycle = 0;
                } else {
                    next_phase = TimerPhase::ShortBreak;
                }
            }
            TimerPhase::ShortBreak | TimerPhase::LongBreak => {
                next_phase = TimerPhase::Pomodoro;
            }
        }
        self.begin_phase(next_phase);
    }

    /// Draws a doughnut-style timer visualization showing progress of the current phase.
    /// The color changes based on the current phase and timer state.
    pub fn draw_doughnut_timer(&self, ui: &mut egui::Ui, rect: egui::Rect) {
        let painter = ui.painter_at(rect);
        let center = rect.center();
        let radius = rect.width().min(rect.height()) / 2.0 * RADIUS_SCALE;
        let stroke_width = radius * STROKE_WIDTH_RATIO; // Thickness of the doughnut ring

        let total_duration_sec = self.get_current_phase_duration_minutes() * SECONDS_PER_MINUTE;
        let remaining_duration_sec = self.get_remaining_time_minutes().max(0.0) * SECONDS_PER_MINUTE;
        let progress_ratio = (total_duration_sec - remaining_duration_sec) / total_duration_sec;
        let spent_angle = progress_ratio * TAU;
        let start_angle_offset = -QUARTER_TURN; // Start from the top (12 o'clock)

        let mut remaining_color = match self.current_phase {
            TimerPhase::Pomodoro => Color32::from_rgb(255, 70, 70), // Reddish
            TimerPhase::ShortBreak => Color32::from_rgb(70, 200, 70), // Greenish
            TimerPhase::LongBreak => Color32::from_rgb(70, 130, 255), // Bluish
        };

        if self.timer_state == TimerState::Paused {
            remaining_color = Color32::from_gray(150); // Diminished color when paused
        }

        let spent_color = Color32::from_gray(80); // Darker gray for spent time

        painter.circle_stroke(center, radius - stroke_width / 2.0, Stroke::new(stroke_width, spent_color));

        if progress_ratio < 1.0 { // Only draw if there's time remaining
            let points_remaining: Vec<Pos2> = (0..=CIRCLE_POINTS)
                .map(|i| {
                    let angle = start_angle_offset + spent_angle + (i as f32 / CIRCLE_POINTS as f32) * (TAU - spent_angle);
                    center + Vec2::new(angle.cos() * radius, angle.sin() * radius)
                })
                .collect();
            if points_remaining.len() > 1 {
                painter.add(Shape::Path(PathShape {
                    points: points_remaining,
                    closed: false,
                    fill: Color32::TRANSPARENT,
                    stroke: PathStroke { width: stroke_width, color: eframe::epaint::ColorMode::Solid(remaining_color), kind: egui::StrokeKind::Middle }, // Reverted to simpler PathStroke
                }));
            }
        }

        let minutes = (remaining_duration_sec / SECONDS_PER_MINUTE).floor();
        let mut seconds = (remaining_duration_sec % SECONDS_PER_MINUTE).floor();
        if remaining_duration_sec > 0.0 && minutes == 0.0 && seconds == 0.0 {
            seconds = 1.0;
        }
        let time_text = format!("{:02}:{:02}", minutes, seconds);
        
        painter.text(
            center,
            Align2::CENTER_CENTER,
            time_text,
            FontId::new(radius * FONT_SIZE_RATIO, egui::FontFamily::Monospace), // Adjust font size relative to radius
            ui.style().visuals.strong_text_color(),
        );
    }

    /// Renders the timer progress as an icon for the window titlebar.
    /// Returns icon data showing the current phase progress as a doughnut shape.
    pub fn render_icon_data(&self, size: u32) -> egui::IconData {
        let mut pixels = vec![Color32::TRANSPARENT; (size * size) as usize];
        let center_f = size as f32 / 2.0;
        
        let path_radius = center_f * RADIUS_SCALE; // Radius of the center-line of the doughnut ring
        let stroke_w = path_radius * STROKE_WIDTH_RATIO; // Thickness of the doughnut ring

        let total_duration_sec = self.get_current_phase_duration_minutes() * SECONDS_PER_MINUTE;
        let remaining_duration_sec = self.get_remaining_time_minutes().max(0.0) * SECONDS_PER_MINUTE;
        
        let progress_ratio = if total_duration_sec > 0.0 {
            (total_duration_sec - remaining_duration_sec) / total_duration_sec
        } else {
            0.0 // Avoid division by zero if duration is zero; show as 0% spent
        };
        let spent_angle_end = progress_ratio * TAU; // Angle covered by spent time
        let start_angle_offset = -QUARTER_TURN; // Start from the top (12 o'clock)

        let mut remaining_color = match self.current_phase {
            TimerPhase::Pomodoro => Color32::from_rgb(255, 70, 70),
            TimerPhase::ShortBreak => Color32::from_rgb(70, 200, 70),
            TimerPhase::LongBreak => Color32::from_rgb(70, 130, 255),
        };

        if self.timer_state == TimerState::Paused {
            remaining_color = Color32::from_gray(150); // Dimmed color when paused
        }
        let spent_color = Color32::from_gray(80); // Darker gray for spent time

        let outer_ring_radius_sq = (path_radius + stroke_w / 2.0).powi(2);
        let inner_ring_radius_sq = (path_radius - stroke_w / 2.0).powi(2);

        for y_idx in 0..size {
            for x_idx in 0..size {
                let (xf, yf) = (x_idx as f32 + 0.5, y_idx as f32 + 0.5); // Use pixel center
                let dist_sq = (xf - center_f).powi(2) + (yf - center_f).powi(2);

                if dist_sq <= outer_ring_radius_sq && dist_sq >= inner_ring_radius_sq {
                    let mut angle = (yf - center_f).atan2(xf - center_f) - start_angle_offset;
                    if angle < 0.0 {
                        angle += TAU; // Normalize angle to 0..TAU relative to start_angle_offset
                    }

                    if angle < spent_angle_end {
                        pixels[(y_idx * size + x_idx) as usize] = spent_color;
                    } else {
                        pixels[(y_idx * size + x_idx) as usize] = remaining_color;
                    }
                }
            }
        }

        let rgba_data: Vec<u8> = pixels
            .into_iter()
            .flat_map(|c| [c.r(), c.g(), c.b(), c.a()])
            .collect();

        egui::IconData {
            rgba: rgba_data,
            width: size,
            height: size,
        }
    }
}

impl MyApp {
    /// Handles timer completion and phase transitions
    fn handle_timer_completion(&mut self) {
        let remaining_time = self.get_remaining_time_minutes();
        if self.timer_state == TimerState::Running && remaining_time <= 0.0 {
            self.play_bell_sound();
            self.next_phase();
        }
    }
    
    /// Updates the window icon if the remaining time has changed
    fn update_window_icon(&mut self, ctx: &egui::Context) {
        let remaining_time = self.get_remaining_time_minutes();
        let remaining_time_seconds: i32 = (remaining_time * SECONDS_PER_MINUTE).round() as i32;
        
        if remaining_time_seconds != self.last_remaining_time_seconds {
            self.last_remaining_time_seconds = remaining_time_seconds;
            let new_icon_data = self.render_icon_data(ICON_SIZE);
            let icon_arc = std::sync::Arc::new(new_icon_data);
            ctx.send_viewport_cmd(egui::ViewportCommand::Icon(Some(icon_arc)));
        }
    }
    
    /// Renders the settings UI panel
    fn render_settings_ui(&mut self, ui: &mut egui::Ui) {
        ui.collapsing("Settings", |ui| {
            let settings_enabled = self.timer_state == TimerState::Stopped;
            ui.horizontal(|ui| {
                ui.label("Pomodoro");
                ui.add_enabled(settings_enabled, egui::Slider::new(&mut self.pomodoro_min, 1.0..=60.0).text("minutes"));
            });
            ui.horizontal(|ui| {
                ui.label("Short Break");
                ui.add_enabled(settings_enabled, egui::Slider::new(&mut self.short_break_min, 1.0..=60.0).text("minutes"));
            });
            ui.horizontal(|ui| {
                ui.label("Cycles");
                ui.add_enabled(settings_enabled, egui::Slider::new(&mut self.cycles, 1..=10).text("cycles"));
            });
            ui.horizontal(|ui| {
                ui.label("Long Break");
                ui.add_enabled(settings_enabled, egui::Slider::new(&mut self.long_break_min, 1.0..=60.0).text("minutes"));
            });
            ui.horizontal(|ui| {
                ui.label("Play Sound");
                ui.add_enabled(settings_enabled, egui::Checkbox::new(&mut self.play_sound, "Play Sound"));
            });
        });
    }
    
    /// Renders the timer control buttons
    fn render_control_buttons(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            let start_button_response = ui.add_enabled(self.timer_state == TimerState::Stopped, egui::Button::new("Start"));
            
            // Handle left click - start with default Pomodoro phase
            if start_button_response.clicked() {
                self.begin_timer();
            }
            
            // Handle right click - show context menu for phase selection
            start_button_response.context_menu(|ui| {
                ui.label("Start with:");
                ui.separator();
                
                if ui.button("🍅 Pomodoro [Default]").clicked() {
                    self.begin_timer_with_phase(TimerPhase::Pomodoro);
                    ui.close_menu();
                } else if ui.button("☕ Short Break").clicked() {
                    self.begin_timer_with_phase(TimerPhase::ShortBreak);
                    ui.close_menu();
                } else if ui.button("🍭 Long Break").clicked() {
                    self.begin_timer_with_phase(TimerPhase::LongBreak);
                    ui.close_menu();
                }
            });
            
            let pause_resume_text = if self.timer_state == TimerState::Paused { "Resume" } else { "Pause" };
            if ui.add_enabled(self.timer_state != TimerState::Stopped, egui::Button::new(pause_resume_text)).clicked() {
                if self.timer_state == TimerState::Running {
                    self.pause_timer();
                } else {
                    self.resume_timer();
                }
            }
            if ui.button("Reset").clicked() {
                self.reset_timer();
            }
        });
    }
    
    /// Renders the status information and timer display
    fn render_status_and_timer(&self, ui: &mut egui::Ui) {
        ui.label(format!("Current Phase: {:?}", self.current_phase));
        ui.label(format!("Current Cycle: {} / {}", self.current_cycle, self.cycles));

        let desired_size = Vec2::splat(TIMER_DISPLAY_SIZE);
        let (response_val, _rect) = ui.allocate_exact_size(desired_size, egui::Sense::hover());
        self.draw_doughnut_timer(ui, response_val);
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.handle_timer_completion();
        ctx.request_repaint_after(std::time::Duration::from_millis(REPAINT_INTERVAL_MS));
        self.update_window_icon(ctx);

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My Pomodoro");
            self.render_settings_ui(ui);
            ui.separator();
            self.render_control_buttons(ui);
            self.render_status_and_timer(ui);
        });
    }
}
