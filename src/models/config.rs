use std::collections::HashMap;

use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub default_icon: String,
    pub remove_duplicates: Option<bool>,
    pub icons: Icon,
}

#[derive(Deserialize, Debug)]
pub struct Icon {
    pub icons: HashMap<String, String>,
}
