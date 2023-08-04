#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use anyhow::anyhow;
use std::error::Error;
use tauri::{generate_context, ActivationPolicy, Builder as Tauri, GlobalShortcutManager, Manager};

mod commands;
mod tray;
mod utils;
mod window;

fn main() {
	let app = Tauri::new()
		.setup(setup_tauri)
		.system_tray(tray::build())
		.on_window_event(window::handler)
		.invoke_handler(commands::handler())
		.on_system_tray_event(tray::handle)
		.build(generate_context!())
		.expect("error while running tauri application");

	app.run(window::prevent_exit);
}

fn setup_tauri(app: &mut tauri::App) -> Result<(), Box<(dyn Error + 'static)>> {
	app.set_activation_policy(ActivationPolicy::Accessory);

	let window = app
		.get_window("main")
		.ok_or_else(|| anyhow!("Window not found"))?;

	window::make_transparent(&window).map_err(|_| {
		anyhow!("Unsupported platform! 'apply_vibrancy' is only supported on macOS")
	})?;

	app.global_shortcut_manager()
		.register("CmdOrControl+Alt+Shift+C", move || {
			window::toggle(&window).unwrap();
		})?;

	Ok(())
}
