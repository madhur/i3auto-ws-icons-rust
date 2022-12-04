use std::collections::HashMap;

use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
#[derive(Default)]
pub struct Config {
    #[serde(default = "default_icon")]
    pub default_icon: Option<String>,
    #[serde(default = "default_remove_duplicates")]
    pub remove_duplicates: Option<bool>,
    #[serde(default = "default_enable_rename")]
    pub enable_rename: Option<bool>,
    pub icons: Option<Icon>,
}

#[derive(Deserialize, Debug)]
pub struct Icon {
    pub icons: HashMap<String, String>,
}

fn default_icon() -> Option<String> {
    Some("*".to_string())
}
fn default_remove_duplicates() -> Option<bool> {
    Some(false)
}

fn default_enable_rename() -> Option<bool> {
    Some(true)
}