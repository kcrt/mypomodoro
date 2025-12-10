//! Platform-specific application icon setting.
//!
//! This code is derived from eframe's internal `app_icon.rs` module.
//! Source: https://github.com/emilk/egui/blob/main/crates/eframe/src/native/app_icon.rs
//!
//! The original implementation only sets the icon once at startup through `AppTitleIconSetter`.
//! This module provides a way to dynamically update the application icon at runtime,
//! which is necessary for showing timer progress in the dock (macOS) or taskbar (Windows).

use egui::IconData;
use image::ImageEncoder;

/// Set the application icon using platform-specific APIs.
/// This ensures the icon appears in dock/taskbar, not just the window title bar.
pub fn set_app_icon(icon_data: &IconData) {
    #[cfg(target_os = "macos")]
    set_app_icon_macos(icon_data);

    #[cfg(target_os = "windows")]
    set_app_icon_windows(icon_data);
}

/// Set the macOS dock icon using NSApplication API.
/// Derived from eframe's set_title_and_icon_mac function.
#[cfg(target_os = "macos")]
fn set_app_icon_macos(icon_data: &IconData) {
    // Convert icon data to PNG
    let mut png_data = Vec::new();
    if let Err(e) = image::codecs::png::PngEncoder::new(&mut png_data).write_image(
        &icon_data.rgba,
        icon_data.width,
        icon_data.height,
        image::ExtendedColorType::Rgba8,
    ) {
        log::warn!("Failed to encode icon as PNG: {}", e);
        return;
    }

    // Use Objective-C to set the dock icon
    unsafe {
        use objc2::ClassType;
        use objc2_app_kit::{NSApplication, NSImage};
        use objc2_foundation::NSData;

        #[allow(improper_ctypes)]
        unsafe extern "C" {
            static NSApp: Option<&'static NSApplication>;
        }

        if let Some(app) = NSApp {
            let data = NSData::with_bytes(&png_data);
            let app_icon = NSImage::initWithData(NSImage::alloc(), &data);
            app.setApplicationIconImage(app_icon.as_deref());
            log::debug!("Successfully updated macOS dock icon");
        } else {
            log::warn!("NSApp is null");
        }
    }
}

/// Set the Windows taskbar icon using Windows API.
/// Derived from eframe's set_app_icon_windows function.
#[cfg(target_os = "windows")]
fn set_app_icon_windows(icon_data: &IconData) {
    // TODO: Implement Windows icon setting
    // The eframe implementation uses winapi to set both ICON_SMALL and ICON_BIG
    // for proper taskbar icon display.
    log::debug!("Windows dynamic icon update not yet implemented");
}
