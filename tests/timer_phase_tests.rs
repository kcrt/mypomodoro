use chrono::{DateTime, Utc};

use mypomodoro::{MyApp, TimerState, TimerPhase};

#[test]
fn test_next_phase_from_pomodoro() {
    let mut app = MyApp::default();
    
    app.begin_timer();
    
    app.next_phase();
    assert_eq!(app.current_phase(), TimerPhase::ShortBreak);
    assert_eq!(app.current_cycle(), 1);
}

#[test]
fn test_next_phase_from_short_break() {
    let mut app = MyApp::default();
    
    app.begin_timer();
    app.next_phase();
    
    app.next_phase();
    assert_eq!(app.current_phase(), TimerPhase::Pomodoro);
    assert_eq!(app.current_cycle(), 1);
}

#[test]
fn test_long_break_after_cycles() {
    let mut app = MyApp::default();
    
    app.begin_timer();
    
    // Complete cycles-1 full cycles (Pomodoro -> ShortBreak -> Pomodoro)
    for _ in 0..app.cycles()-1 {
        // Pomodoro -> ShortBreak
        app.next_phase();
        // ShortBreak -> Pomodoro
        app.next_phase();
    }
    
    // Now we're at the last Pomodoro of the set
    // The cycle counter should be at cycles-1
    assert_eq!(app.current_cycle(), app.cycles()-1);
    
    // This should trigger the long break
    app.next_phase();
    assert_eq!(app.current_phase(), TimerPhase::LongBreak);
    assert_eq!(app.current_cycle(), 0); // Cycle counter should reset
}

#[test]
fn test_get_phase_duration_minutes() {
    let app = MyApp::default();
    
    assert_eq!(app.get_phase_duration_minutes(TimerPhase::Pomodoro), 25.0);
    assert_eq!(app.get_phase_duration_minutes(TimerPhase::ShortBreak), 5.0);
    assert_eq!(app.get_phase_duration_minutes(TimerPhase::LongBreak), 15.0);
}

#[test]
fn test_get_current_phase_duration_minutes() {
    let mut app = MyApp::default();
    
    assert_eq!(app.get_current_phase_duration_minutes(), 25.0);
    
    app.begin_timer();
    app.next_phase();
    assert_eq!(app.get_current_phase_duration_minutes(), 5.0);
    
    app.begin_phase(TimerPhase::LongBreak);
    assert_eq!(app.get_current_phase_duration_minutes(), 15.0);
}
