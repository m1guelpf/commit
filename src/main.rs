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

	window::main_window::create(&app.handle())?;
	let settings_window = window::settings::create(&app.handle())?;

	let config = app.user_config();
	let config_r = config
		.read()
		.map_err(|_| anyhow!("Failed to read config"))?;

	if config_r.shortcut != shortcuts::DEFAULT_SHORTCUT {
		shortcuts::update_default(
			&app.handle(),
			shortcuts::DEFAULT_SHORTCUT,
			&config_r.shortcut,
		)?;
	}

	if config_r.autostart {
		let autolaunch = app.autolaunch();

		autolaunch
			.enable()
			.map_err(|_| anyhow!("Failed to enable autostart"))?;
	}

	if config_r.first_run {
		drop(config_r); // Prevent deadlock saving config

		settings_window.show()?;
		settings_window.set_focus()?;

		{
			let mut config_rw = config
				.write()
				.map_err(|_| anyhow!("Failed to write config"))?;
			config_rw.first_run = false;
			config_rw.save()?;
		}
	}

	Ok(())
}
