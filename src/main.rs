#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use anyhow::anyhow;
use config::{Config, ConfigExt};
use std::error::Error;
use tauri::{generate_context, ActivationPolicy, Builder as Tauri, Manager};
use tauri_plugin_autostart::{self as tauri_autostart, MacosLauncher};
use tauri_plugin_spotlight::WindowConfig;

mod commands;
mod config;
mod shortcuts;
mod tray;
mod utils;
mod window;

fn main() {
	let config = Config::load().unwrap();

	let app = Tauri::new()
		.setup(setup_tauri)
		.system_tray(tray::build())
		.manage(config.manage())
		.on_window_event(window::handler)
		.invoke_handler(commands::handler())
		.on_system_tray_event(tray::handle)
		.plugin(tauri_autostart::init(MacosLauncher::LaunchAgent, None))
		.plugin(tauri_plugin_spotlight::init(Some(
			tauri_plugin_spotlight::PluginConfig {
				windows: Some(vec![WindowConfig {
					macos_window_level: Some(20),
					label: String::from(window::NAME),
					shortcut: String::from(shortcuts::DEFAULT_SHORTCUT),
				}]),
				global_close_shortcut: None,
			},
		)))
		.build(generate_context!())
		.expect("error while running tauri application");

	app.run(window::prevent_exit);
}

fn setup_tauri(app: &mut tauri::App) -> Result<(), Box<(dyn Error + 'static)>> {
	app.set_activation_policy(ActivationPolicy::Accessory);

	let window = app
		.get_window(window::NAME)
		.ok_or_else(|| anyhow!("Window not found"))?;

	window::make_transparent(&window).map_err(|_| {
		anyhow!("Unsupported platform! 'apply_vibrancy' is only supported on macOS")
	})?;

	let config = app.user_config();
	let config = config
		.read()
		.map_err(|_| anyhow!("Failed to read config"))?;

	if config.shortcut != shortcuts::DEFAULT_SHORTCUT {
		shortcuts::update_default(window, shortcuts::DEFAULT_SHORTCUT, &config.shortcut)?;
	}

	Ok(())
}
