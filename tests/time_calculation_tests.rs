use std::time::Duration;

use mypomodoro::MyApp;

#[test]
fn test_get_spent_time_minutes_when_stopped() {
    let app = MyApp::default();
    
    assert_eq!(app.get_spent_time_minutes(), 0.0);
}

#[test]
fn test_get_remaining_time_minutes_when_stopped() {
    let app = MyApp::default();
    
    assert_eq!(app.get_remaining_time_minutes(), app.pomodoro_min());
}


#[test]
fn test_get_spent_time_minutes_increases_over_time() {
    let mut app = MyApp::default();
    
    app.begin_timer();
    
    let initial_spent = app.get_spent_time_minutes();
    
    std::thread::sleep(Duration::from_millis(100));
    
    let later_spent = app.get_spent_time_minutes();
    
    assert!(later_spent > initial_spent);
}

#[test]
fn test_get_remaining_time_minutes_decreases_over_time() {
    let mut app = MyApp::default();
    
    app.begin_timer();
    
    let initial_remaining = app.get_remaining_time_minutes();
    
    std::thread::sleep(Duration::from_millis(100));
    
    let later_remaining = app.get_remaining_time_minutes();
    
    assert!(later_remaining < initial_remaining);
}

#[test]
fn test_pause_stops_time_calculation() {
    let mut app = MyApp::default();
    
    app.begin_timer();
    
    std::thread::sleep(Duration::from_millis(100));
    
    app.pause_timer();
    
    let spent_at_pause = app.get_spent_time_minutes();
    
    std::thread::sleep(Duration::from_millis(100));
    
    let spent_after_wait = app.get_spent_time_minutes();
    
    assert_eq!(spent_at_pause, spent_after_wait);
}

#[test]
fn test_resume_continues_time_calculation() {
    let mut app = MyApp::default();
    
    app.begin_timer();
    
    app.pause_timer();
    
    let spent_at_pause = app.get_spent_time_minutes();
    
    app.resume_timer();
    
    std::thread::sleep(Duration::from_millis(100));
    
    let spent_after_resume = app.get_spent_time_minutes();
    
    assert!(spent_after_resume > spent_at_pause);
}
