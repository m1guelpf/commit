use std::{fs, io, path::PathBuf, sync::RwLock};

use anyhow::Result;
use directories::ProjectDirs;

use crate::shortcuts;

pub type GetConfig = RwLock<Config>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
	#[error(transparent)]
	Io(#[from] io::Error),
	#[error(transparent)]
	Encoding(#[from] toml::ser::Error),
	#[error(transparent)]
	Decoding(#[from] toml::de::Error),
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Config {
	pub autostart: bool,
	#[serde(default)]
	pub first_run: bool,
	pub shortcut: String,
	pub should_push: bool,
	pub repo_paths: Vec<PathBuf>,
}

impl Config {
	pub fn load() -> Result<Self, Error> {
		let config_path = Self::config_path();

		if config_path.exists() {
			let config = toml::from_str(&fs::read_to_string(config_path)?)?;

			return Ok(config);
		}

		let config = Self::default();
		config.save()?;

		Ok(config)
	}

	pub fn save(&self) -> Result<(), Error> {
		let path = Self::config_path();
		fs::create_dir_all(path.parent().unwrap())?;
		fs::write(path, toml::to_string_pretty(self)?)?;

		Ok(())
	}

	pub fn update(&mut self, new_config: Config) -> Result<(), Error> {
		self.shortcut = new_config.shortcut;
		self.autostart = new_config.autostart;
		self.repo_paths = new_config.repo_paths;
		self.should_push = new_config.should_push;

		self.save()
	}

	pub fn manage(self) -> RwLock<Self> {
		RwLock::new(self)
	}

	pub fn config_path() -> PathBuf {
		let project_dirs = ProjectDirs::from("", "Miguel Piedrafita", "Commit").unwrap();

		project_dirs.config_dir().join("Settings.toml")
	}
}

impl Default for Config {
	fn default() -> Self {
		Self {
			first_run: true,
			autostart: false,
			should_push: true,
			repo_paths: vec![],
			shortcut: shortcuts::DEFAULT_SHORTCUT.to_string(),
		}
	}
}

pub trait ConfigExt<R: tauri::Runtime> {
	fn user_config(&self) -> tauri::State<'_, GetConfig>;
}

impl<R: tauri::Runtime, T: tauri::Manager<R>> ConfigExt<R> for T {
	fn user_config(&self) -> tauri::State<'_, GetConfig> {
		self.state::<GetConfig>()
	}
}
