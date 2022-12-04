use serde_derive::Deserialize;
use rust_embed::RustEmbed;
#[derive(Deserialize, Debug)]
pub struct FAConfig {
    pub solid: Vec<Solid>,
    pub brands: Vec<Brand>,
}

#[derive(Deserialize, Debug)]
pub struct Solid {
    pub name: String,
    pub unicode: String,
}
#[derive(Deserialize, Debug)]
pub struct Brand {
    pub name: String,
    pub unicode: String,
}


#[derive(RustEmbed)]
#[folder = "src/assets/"]
pub struct Asset;

#[derive(RustEmbed)]
#[folder = "examples/"]

pub struct DefaultConfig;