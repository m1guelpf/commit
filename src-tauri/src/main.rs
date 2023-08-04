#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::path::PathBuf;

use git2::Repository;
use tauri::{
	generate_context, ActivationPolicy, AppHandle, Builder as Tauri, CustomMenuItem,
	GlobalShortcutManager, Manager, RunEvent, SystemTray, SystemTrayEvent, SystemTrayMenu,
	SystemTrayMenuItem, WindowEvent,
};
use window_vibrancy::{apply_vibrancy, NSVisualEffectMaterial, NSVisualEffectState};

mod utils;

enum TrayMenu {
	Quit,
	#[cfg(debug_assertions)]
	DevTools,
}

impl From<TrayMenu> for String {
	fn from(value: TrayMenu) -> Self {
		match value {
			TrayMenu::Quit => "quit".to_string(),
			#[cfg(debug_assertions)]
			TrayMenu::DevTools => "devtools".to_string(),
		}
	}
}

impl From<String> for TrayMenu {
	fn from(value: String) -> Self {
		match value.as_str() {
			"quit" => TrayMenu::Quit,
			#[cfg(debug_assertions)]
			"devtools" => TrayMenu::DevTools,
			_ => unreachable!(),
		}
	}
}

#[tauri::command]
fn commit(
	app: AppHandle,
	path: PathBuf,
	title: String,
	description: Option<String>,
) -> Result<(), String> {
	let repo = Repository::open(path).map_err(|e| e.to_string())?;

	utils::commit(
		&repo,
		&format!(
			"{title}{}",
			description.map(|m| format!("\n\n{m}")).unwrap_or_default()
		),
	)
	.map_err(|e| e.to_string())?;

	handle_window_toggle(&app).unwrap();

	Ok(())
}

fn main() {
	let tray_menu = SystemTrayMenu::new();

	#[cfg(debug_assertions)]
	let tray_menu = tray_menu
		.add_item(
			CustomMenuItem::new(TrayMenu::DevTools, "Open DevTools").accelerator("Cmd+Shift+I"),
		)
		.add_native_item(SystemTrayMenuItem::Separator);

	let tray_menu =
		tray_menu.add_item(CustomMenuItem::new(TrayMenu::Quit, "Quit").accelerator("Cmd+Q"));

	let app = Tauri::default()
		.setup(|app| {
			app.set_activation_policy(ActivationPolicy::Accessory);
			let mut shortcuts = app.global_shortcut_manager();

			let app_handle = app.handle();
			shortcuts
				.register("CmdOrControl+Alt+Shift+C", move || {
					handle_window_toggle(&app_handle).unwrap();
				})
				.unwrap();

			let window = app.get_window("main").unwrap();
			apply_vibrancy(
				&window,
				NSVisualEffectMaterial::HudWindow,
				Some(NSVisualEffectState::Active),
				Some(10.0),
			)
			.expect("Unsupported platform! 'apply_vibrancy' is only supported on macOS");

			Ok(())
		})
		.system_tray(SystemTray::new().with_menu(tray_menu))
		.on_system_tray_event(|app, event| match event {
			SystemTrayEvent::LeftClick { .. } => handle_window_toggle(app).unwrap(),
			SystemTrayEvent::MenuItemClick { id, .. } => match id.into() {
				TrayMenu::Quit => std::process::exit(0),
				#[cfg(debug_assertions)]
				TrayMenu::DevTools => app.get_window("main").unwrap().open_devtools(),
			},
			_ => {},
		})
		.on_window_event(|event| {
			if let WindowEvent::Focused(is_focused) = event.event() {
				if !is_focused && event.window().is_visible().unwrap() {
					handle_window_toggle(&event.window().app_handle()).unwrap();
				}
			}
		})
		.invoke_handler(tauri::generate_handler![commit])
		.build(generate_context!())
		.expect("error while running tauri application");

	app.run(|_, event| {
		if let RunEvent::ExitRequested { api, .. } = event {
			api.prevent_exit();
		}
	});
}

fn handle_window_toggle(app: &AppHandle) -> Option<()> {
	let window = app.get_window("main")?;

	if window.is_visible().unwrap() {
		window.emit("reset", true).unwrap();
		window.hide().unwrap();
	} else {
		let app_handle = window.app_handle();
		tauri::async_runtime::spawn(async move {
			let window = app_handle.get_window("main")?;

			let Some(repo_path) = utils::find_latest_repo(&["/Users/m1guelpf/Code".into()]).ok()?
			else {
				window.emit("current_dir", Option::<String>::None).unwrap();

				return None;
			};

			window.emit("current_dir", &repo_path).unwrap();

			let repo = Repository::open(&repo_path).ok()?;

			window
				.emit("current_branch", utils::get_branch_name(&repo))
				.unwrap();

			window
				.emit(
					"current_repo",
					utils::get_repo_name(&repo).or_else(|| {
						repo_path
							.file_name()
							.and_then(|s| s.to_str())
							.map(ToString::to_string)
					}),
				)
				.unwrap();

			window.emit("current_diff", utils::get_diff(&repo)).unwrap();

			Some(())
		});

		window.show().unwrap();
		window.set_focus().unwrap();
	}

	Some(())
}
