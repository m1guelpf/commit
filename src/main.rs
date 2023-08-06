#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use anyhow::anyhow;
use config::{Config, ConfigExt};
use std::error::Error;
use tauri::{generate_context, ActivationPolicy, Builder as Tauri};
use tauri_autostart::ManagerExt;
use tauri_plugin_autostart::{self as tauri_autostart};

mod commands;
mod config;
mod repo;
mod shortcuts;
mod tray;
mod window;

fn main() {
	let config = Config::load().unwrap();

	let app = Tauri::new()
		.setup(setup_tauri)
		.plugin(tray::autostart())
		.system_tray(tray::build())
		.plugin(window::spotlight())
		.manage(config.manage())
		.on_window_event(window::handler)
		.invoke_handler(commands::handler())
		.on_system_tray_event(tray::handle)
		.build(generate_context!())
		.expect("error while running tauri application");

	app.run(window::prevent_exit);
}

fn setup_tauri(app: &mut tauri::App) -> Result<(), Box<(dyn Error + 'static)>> {
	app.set_activation_policy(ActivationPolicy::Accessory);

	window::settings::create(&app.handle())?;
	window::main_window::create(&app.handle())?;

	let config = app.user_config();
	let config = config
		.read()
		.map_err(|_| anyhow!("Failed to read config"))?;

	if config.shortcut != shortcuts::DEFAULT_SHORTCUT {
		shortcuts::update_default(&app.handle(), shortcuts::DEFAULT_SHORTCUT, &config.shortcut)?;
	}

	if config.autostart {
		let autolaunch = app.autolaunch();

		autolaunch
			.enable()
			.map_err(|_| anyhow!("Failed to enable autostart"))?;
	}

	Ok(())
}
