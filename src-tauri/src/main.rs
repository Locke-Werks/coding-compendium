// Stops a console window appearing behind the app on Windows in release builds.
// Debug builds keep it, because that is where println! and panic messages go.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    compendium_lib::run()
}
