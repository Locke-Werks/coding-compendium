// Coding Compendium, an offline reference for software development in the age of coding agents.
// Copyright (C) 2026 Locke Werks
//
// This program is free software: you can redistribute it and/or modify it under
// the terms of the GNU General Public License as published by the Free Software
// Foundation, either version 3 of the License, or (at your option) any later
// version.
//
// This program is distributed in the hope that it will be useful, but WITHOUT ANY
// WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A
// PARTICULAR PURPOSE. See the GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License along with
// this program. If not, see <https://www.gnu.org/licenses/>.
//
// The reference corpus in content/ is not part of this program and is dedicated
// to the public domain under CC0 1.0. See LICENSE-CONTENT.

//! The sidecar: a narrow window that stays beside the terminal.
//!
//! # Why this exists
//!
//! Alt-tabbing to a reference tool is a habit, and beginners do not have it.
//! They get stuck, stare at the error, and stop. The global hotkey solves half
//! of that by removing the hunt for the window. The sidecar solves the other
//! half by removing the switch entirely: the answer is already on screen next to
//! the thing that broke.
//!
//! Presence beats findability for someone who does not yet know the app is
//! worth opening.
//!
//! # What makes it different from the main window
//!
//! Narrow, always on top, docked to a screen edge, and deliberately not resizable
//! into something that competes with the editor. It is a strip you read, not a
//! workspace.

use tauri::{AppHandle, Manager, WebviewUrl, WebviewWindowBuilder};

/// Width of the strip, in logical pixels.
///
/// Wide enough for a sentence at a comfortable measure, narrow enough to sit
/// beside a maximized editor without covering the code they are reading. Going
/// wider defeats the point: at that size they may as well use the main window.
const WIDTH: f64 = 400.0;

/// Show the sidecar, creating it the first time.
///
/// Docks to the right edge of whichever monitor the main window is on, because
/// that is the monitor they are looking at. Guessing the primary display puts the
/// strip on the wrong screen for anyone with two.
pub fn toggle(app: &AppHandle) -> tauri::Result<()> {
    if let Some(existing) = app.get_webview_window("sidecar") {
        // Toggle: a second press dismisses it, same as the hotkey.
        if existing.is_visible().unwrap_or(false) {
            existing.hide()?;
        } else {
            existing.show()?;
            existing.set_focus()?;
        }
        return Ok(());
    }

    // `?sidecar` is read by the frontend to render the narrow layout. A query
    // string rather than a separate HTML entry point, so both windows ship the
    // same bundle and cannot drift apart.
    let window = WebviewWindowBuilder::new(app, "sidecar", WebviewUrl::App("index.html?sidecar".into()))
        .title("Compendium")
        .inner_size(WIDTH, 900.0)
        .min_inner_size(320.0, 400.0)
        .always_on_top(true)
        .skip_taskbar(true)
        .decorations(true)
        .build()?;

    // Dock to the right edge of the monitor the main window is on.
    if let Some(main) = app.get_webview_window("main") {
        if let Ok(Some(monitor)) = main.current_monitor() {
            let size = monitor.size();
            let pos = monitor.position();
            let scale = monitor.scale_factor();

            let width_px = (WIDTH * scale) as i32;
            // A small inset from the edge so the window shadow is visible and it
            // does not read as fused to the screen border.
            let inset = (12.0 * scale) as i32;
            let x = pos.x + size.width as i32 - width_px - inset;
            let y = pos.y + inset;

            let _ = window.set_position(tauri::PhysicalPosition::new(x, y));
            let _ = window.set_size(tauri::PhysicalSize::new(
                width_px as u32,
                size.height.saturating_sub((inset * 2) as u32),
            ));
        }
    }

    Ok(())
}
