use std::{fs::OpenOptions, io::Read, path::PathBuf};

use gpui::*;
use serde::Deserialize;

use crate::app;

#[derive(Default, Deserialize)]
pub struct Config {
    pub proxy: Option<String>,
}

impl Config {
    fn config_path() -> PathBuf {
        let username = whoami::username();
        let user_dir = PathBuf::from("/Users").join(username);

        user_dir.join(".config").join(app::NAME).join("config.toml")
    }

    fn load_config() -> Self {
        let config_path = Self::config_path();

        let mut config = String::new();
        OpenOptions::new()
            .read(true)
            .open(config_path)
            .and_then(|mut file| file.read_to_string(&mut config))
            .ok();

        toml::from_str::<Self>(&config).unwrap_or_default()
    }

    pub fn reload(cx: &mut WindowContext) {
        cx.update_global::<Self, _>(|config, _| {
            *config = Self::load_config();
        });
    }

    pub fn init(cx: &mut AppContext) {
        let config = Self::load_config();

        cx.set_global(config)
    }
}

impl Global for Config {}
