
use serde::{Deserialize, Serialize};
use crate::*;

const CONFIG_PATH: &str = "sd:/Twitch_Integration_Config.toml";


pub static mut CONFIG: Option<Box<Config>> = None;

#[derive(Deserialize, Serialize, Clone)]
pub struct Config {
    pub oauth: String,
    pub channel: String,
    pub mode: GameModes,
    pub voting_interval: u32,
}
impl Config {
    fn new() -> Self {
        Config {
            oauth: "oauth:".to_string(), // get it here https://twitchapps.com/tmi/
            channel: "faultypine".to_string(),
            mode: GameModes::ChooseEffect,
            voting_interval: 45,
        }
    }
}

pub fn init_config() {
    let mut config = Config::new();
    let config_path = std::path::Path::new(CONFIG_PATH);
    if !config_path.exists() {
        let _ = std::fs::File::create(config_path);
        let config_str = toml::to_string(&config).unwrap();
        let _ = std::fs::write(config_path, config_str);
        println!("[Twitch Integration] Created new config file at {}", CONFIG_PATH);
    }
    else {
        let config_contents_raw = std::fs::read(config_path).unwrap();
        let config_contents = std::str::from_utf8(&config_contents_raw).unwrap();
        config = match toml::from_str(config_contents) {
            Ok(s) => s,
            Err(e) => {
                println!("[Twitch Integration] Failed to deserialize config file! Your config file may be malformed. {}", e);
                config
            }
        };
    }
    unsafe { CONFIG = Some(Box::new(config)); }
}