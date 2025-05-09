use chrono::{DateTime, Utc};
use std::time::Duration;

use mypomodoro::{MyApp, TimerState, TimerPhase};

#[test]
fn test_default_state() {
    let app = MyApp::default();
    
    assert_eq!(app.timer_state(), TimerState::Stopped);
    assert_eq!(app.current_phase(), TimerPhase::Pomodoro);
    assert_eq!(app.current_cycle(), 0);
    assert_eq!(app.pomodoro_min(), 25.0);
    assert_eq!(app.short_break_min(), 5.0);
    assert_eq!(app.long_break_min(), 15.0);
    assert_eq!(app.cycles(), 4);
    assert!(app.play_sound());
}

#[test]
fn test_begin_timer() {
    let mut app = MyApp::default();
    
    app.begin_timer();
    
    assert_eq!(app.timer_state(), TimerState::Running);
    assert_eq!(app.current_phase(), TimerPhase::Pomodoro);
    assert!(app.phase_start_time().is_some());
}

#[test]
fn test_pause_resume_timer() {
    let mut app = MyApp::default();
    
    app.begin_timer();
    
    app.pause_timer();
    assert_eq!(app.timer_state(), TimerState::Paused);
    assert!(app.pause_start_time().is_some());
    
    app.resume_timer();
    assert_eq!(app.timer_state(), TimerState::Running);
    assert!(app.pause_start_time().is_none());
}

#[test]
fn test_reset_timer() {
    let mut app = MyApp::default();
    
    app.begin_timer();
    app.reset_timer();
    
    assert_eq!(app.timer_state(), TimerState::Stopped);
    assert_eq!(app.current_phase(), TimerPhase::Pomodoro);
    assert_eq!(app.current_cycle(), 0);
    assert!(app.phase_start_time().is_none());
    assert!(app.pause_start_time().is_none());
    assert_eq!(app.pause_delta_min(), 0.0);
}
