#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui::{self, epaint::{PathShape, Shape, Stroke, PathStroke}, Pos2, Vec2, Color32, Align2, FontId}; // Removed PathStrokeKind, ColorMode
use chrono::{DateTime, Utc};
use std::f32::consts::TAU;

fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([400.0, 400.0]),
        ..Default::default()
    };
    eframe::run_native(
        "My pomodoro",
        options,
        Box::new(|_cc| {
            Ok(Box::<MyApp>::default())
        }),
    )
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum TimerState {
    Running,
    Paused,
    Stopped,
}
#[derive(Debug, PartialEq, Clone, Copy)]
enum TimerPhase {
    Pomodoro,
    ShortBreak,
    LongBreak,
}
struct MyApp {
    pomodoro_min: f32,
    short_break_min: f32,
    cycles: i32,
    long_break_min: f32,
    play_sound: bool,

    // Timer state
    timer_state: TimerState,
    current_phase: TimerPhase,
    current_cycle: i32,
    phase_start_time: Option<DateTime<Utc>>,
    pause_delta_min: f32, // Time spent in pause
    pause_start_time: Option<DateTime<Utc>>,
}


impl Default for MyApp {
    fn default() -> Self {
        let pomodoro_duration_minutes = 25.0;
        Self {
            pomodoro_min: pomodoro_duration_minutes,
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
        }
    }
}

impl MyApp {
    fn get_current_phase_duration_minutes(&self) -> f32 {
        self.get_phase_duration_minutes(self.current_phase)
    }

    fn get_phase_duration_minutes(&self, phase: TimerPhase) -> f32 {
        match phase {
            TimerPhase::Pomodoro => self.pomodoro_min,
            TimerPhase::ShortBreak => self.short_break_min,
            TimerPhase::LongBreak => self.long_break_min,
        }
    }

    fn get_spent_time_minutes(&self) -> f32 {
        if let Some(phase_start_time_actual) = self.phase_start_time {
            if self.timer_state == TimerState::Paused {
                // When paused, display the time spent up until the pause began.
                // self.pause_start_time is Some when Paused.
                // self.pause_delta_min contains sum of durations of *previously completed* pauses.
                if let Some(current_pause_start_time) = self.pause_start_time {
                    let elapsed_duration_until_current_pause = current_pause_start_time.signed_duration_since(phase_start_time_actual).num_milliseconds() as f32 / 60_000.0;
                    return elapsed_duration_until_current_pause - self.pause_delta_min;
                } else {
                    return 0.0;
                }
            } else {
                // Timer is Running or Stopped (but phase_start_time is Some, so it was running before stop or is currently running)
                // Calculate current elapsed time, accounting for all completed pauses.
                let elapsed_total = Utc::now().signed_duration_since(phase_start_time_actual).num_milliseconds() as f32 / 60_000.0;
                return elapsed_total - self.pause_delta_min;
            }
        }
        0.0
    }
    fn get_remaining_time_minutes(&self) -> f32 {
        let spent_time = self.get_spent_time_minutes();
        let total_duration = self.get_current_phase_duration_minutes();
        total_duration - spent_time // could be negative if time is up
    }

    fn begin_timer(&mut self) {
        self.timer_state = TimerState::Running;
        self.begin_phase(TimerPhase::Pomodoro);
    }
    fn begin_phase(&mut self, phase: TimerPhase) {
        self.current_phase = phase;
        self.phase_start_time = Some(Utc::now());
        self.pause_delta_min = 0.0;
    }
    fn pause_timer(&mut self) {
        assert!(self.timer_state == TimerState::Running, "Cannot pause a timer that is not running.");
        self.pause_start_time = Some(Utc::now());
        self.timer_state = TimerState::Paused;
    }
    fn resume_timer(&mut self) {
        assert!(self.timer_state == TimerState::Paused, "Cannot resume a timer that is not paused.");
        if let Some(start_time) = self.pause_start_time {
            let pause_duration_min = Utc::now().signed_duration_since(start_time).num_milliseconds() as f32 / 60_000.0;
            self.pause_delta_min += pause_duration_min;
            self.pause_start_time = None;
        }
        self.timer_state = TimerState::Running;
    }
    fn reset_timer(&mut self) {
        self.timer_state = TimerState::Stopped;
        self.phase_start_time = None;
        self.pause_start_time = None;
        self.pause_delta_min = 0.0;
        self.current_cycle = 0;
        self.current_phase = TimerPhase::Pomodoro;
    }

    fn play_bell_sound(&self) {
        if self.play_sound {
            let bell_data = include_bytes!("../resources/bell.mp3");
            
            use std::io::Cursor;
            use rodio::{Decoder, OutputStream, Sink};
            
            if let Ok((_stream, stream_handle)) = OutputStream::try_default() {
                if let Ok(sink) = Sink::try_new(&stream_handle) {
                    if let Ok(source) = Decoder::new(Cursor::new(bell_data)) {
                        sink.append(source);
                        sink.set_volume(0.1); // Set volume to 10%
                        sink.sleep_until_end();
                    }
                }
            }
        }
    }
    fn next_phase(&mut self) {
        let next_phase;
        match self.current_phase {
            TimerPhase::Pomodoro => {
                self.current_cycle += 1;
                if self.current_cycle >= self.cycles {
                    next_phase = TimerPhase::LongBreak;
                    self.current_cycle = 0; // Reset for next set of cycles
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

    fn draw_doughnut_timer(&self, ui: &mut egui::Ui, rect: egui::Rect) {
        let painter = ui.painter_at(rect);
        let center = rect.center();
        let radius = rect.width().min(rect.height()) / 2.0 * 0.8;
        let stroke_width = radius * 0.25; // Thickness of the doughnut ring

        let total_duration_sec = self.get_current_phase_duration_minutes() * 60.0;
        let remaining_duration_sec = self.get_remaining_time_minutes().max(0.0) * 60.0;
        let progress_ratio = (total_duration_sec - remaining_duration_sec) / total_duration_sec;
        let spent_angle = progress_ratio * TAU;
        let start_angle_offset = -TAU / 4.0; // Start from the top (12 o'clock)

        // Color for the remaining time based on phase
        let mut remaining_color = match self.current_phase {
            TimerPhase::Pomodoro => Color32::from_rgb(255, 70, 70), // Reddish
            TimerPhase::ShortBreak => Color32::from_rgb(70, 200, 70), // Greenish
            TimerPhase::LongBreak => Color32::from_rgb(70, 130, 255), // Bluish
        };

        if self.timer_state == TimerState::Paused {
            remaining_color = Color32::from_gray(150); // Diminished color when paused
        }

        let spent_color = Color32::from_gray(80); // Darker gray for spent time

        // 1. Draw the spent time track (full circle with spent_color)
        painter.circle_stroke(center, radius - stroke_width / 2.0, Stroke::new(stroke_width, spent_color));

        // 2. Draw the "remaining time" arc (on top of the background, starting where spent ended)
        if progress_ratio < 1.0 { // Only draw if there's time remaining
            let points_remaining: Vec<Pos2> = (0..=100)
                .map(|i| {
                    // Start angle for remaining is where spent ended
                    let angle = start_angle_offset + spent_angle + (i as f32 / 100.0) * (TAU - spent_angle);
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

        // Draw the time text in the center
        let minutes = (remaining_duration_sec / 60.0).floor();
        let mut seconds = (remaining_duration_sec % 60.0).floor();
        if remaining_duration_sec > 0.0 && minutes == 0.0 && seconds == 0.0 {
            // Avoid drawing "00:00" when timer is running
            seconds = 1.0;
        }
        let time_text = format!("{:02}:{:02}", minutes, seconds);
        
        painter.text(
            center,
            Align2::CENTER_CENTER,
            time_text,
            FontId::new(radius * 0.5, egui::FontFamily::Monospace), // Adjust font size relative to radius
            ui.style().visuals.strong_text_color(),
        );
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.timer_state == TimerState::Running {
            let remaining_time = self.get_remaining_time_minutes();
            if remaining_time <= 0.0 {
                // It's time!
                self.play_bell_sound();
                self.next_phase();
            }
        }
        ctx.request_repaint_after(std::time::Duration::from_millis(100));

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My Pomodoro");
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
            ui.separator();
            ui.horizontal(|ui| {
                if ui.add_enabled(self.timer_state == TimerState::Stopped, egui::Button::new("Start")).clicked() {
                    self.begin_timer();
                }
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

            ui.label(format!("Current Phase: {:?}", self.current_phase));
            ui.label(format!("Current Cycle: {} / {}", self.current_cycle, self.cycles));

            let desired_size = Vec2::splat(250.0);
            let (response_val, _rect) = ui.allocate_exact_size(desired_size, egui::Sense::hover());
            self.draw_doughnut_timer(ui, response_val);
            
        });
    }
}
