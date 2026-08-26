//! Summoning the app from anywhere in Windows.
//!
//! This is the feature the whole "faster than opening a browser tab" claim rests
//! on. Without it, looking something up means finding the window first, and
//! alt-tabbing to a reference tool is a habit you have to build. Beginners do not
//! build it: they get stuck and stop.
//!
//! # Toggle, not summon
//!
//! Pressing the key with the app already in front hides it. That makes one key
//! the whole interaction: press to ask, press to dismiss, without moving to the
//! mouse or hunting for a close button. A summon-only binding leaves a window
//! they then have to get rid of.
//!
//! # It selects rather than clears
//!
//! Summoning selects whatever is in the box instead of emptying it. Typing
//! replaces it, which is what they want nine times in ten, and the tenth time
//! the previous question is still there to press End and edit. Clearing outright
//! would throw away a query they may have spent effort phrasing.

use tauri::{AppHandle, Emitter, Manager};
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState};

/// The binding.
///
/// Ctrl+Shift+Space is deliberately awkward to press by accident and is not
/// claimed by Windows, Visual Studio Code, or a browser. Ctrl+Space alone is
/// taken by IntelliSense in every editor they are likely to have open, and
/// Alt+Space opens the window menu on Windows.
/// A function rather than a const, because `Shortcut::new` is not const.
fn toggle() -> Shortcut {
    Shortcut::new(Some(Modifiers::CONTROL.union(Modifiers::SHIFT)), Code::Space)
}

/// Human-readable form, for the footer hint.
pub const TOGGLE_LABEL: &str = "Ctrl+Shift+Space";

/// Docks or hides the narrow always-on-top strip.
///
/// A binding as well as a button, because the strip is something they will toggle
/// while their hands are already on the keyboard and mid-task. Reaching for the
/// mouse to dismiss a reference window is exactly the friction it exists to
/// remove.
fn dock() -> Shortcut {
    Shortcut::new(Some(Modifiers::CONTROL.union(Modifiers::SHIFT)), Code::KeyD)
}

pub const DOCK_LABEL: &str = "Ctrl+Shift+D";

/// Register the global shortcut.
///
/// Returns an error only if the binding is already claimed by another program.
/// The caller treats that as a degraded feature rather than a failed startup:
/// the app is perfectly usable by clicking on it, and refusing to launch because
/// a key combination was taken would be a wildly disproportionate response.
pub fn register(app: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    app.global_shortcut().on_shortcut(toggle(), move |app, _shortcut, event| {
        // Fire on press only. Without this the toggle runs twice per keypress,
        // once down and once up, which lands back exactly where it started and
        // looks like the shortcut does nothing at all.
        if event.state() != ShortcutState::Pressed {
            return;
        }

        let Some(window) = app.get_webview_window("main") else { return };

        // Hide only when it is both visible AND focused. A visible-but-buried
        // window should come forward, not disappear: they pressed the key because
        // they could not see it.
        let visible = window.is_visible().unwrap_or(false);
        let focused = window.is_focused().unwrap_or(false);

        if visible && focused {
            let _ = window.hide();
            return;
        }

        let _ = window.show();
        let _ = window.unminimize();
        let _ = window.set_focus();
        // The frontend puts the caret in the search box and selects what is
        // there. Doing it here would need the webview to be focused already,
        // which it is not until this returns.
        let _ = app.emit("summoned", ());
    })?;

    app.global_shortcut().on_shortcut(dock(), move |app, _shortcut, event| {
        if event.state() != ShortcutState::Pressed {
            return;
        }
        // A failure here costs the strip, not the app. The main window is
        // untouched and they can still search in it.
        let _ = crate::sidecar::toggle(app);
    })?;

    Ok(())
}
