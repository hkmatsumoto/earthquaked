pub mod config;
pub mod watchdog;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum Location {
    Locale {
        region: Region,
        area: Option<String>,
    },
    Horizontal(Horizontal),
}

impl Default for Location {
    fn default() -> Self {
        Location::Locale {
            region: Region::Japan,
            area: Some("東京".to_owned()),
        }
    }
}

#[derive(Deserialize, Debug)]
pub enum Region {
    #[serde(rename = "jp")]
    Japan,
}

#[derive(Deserialize, Debug)]
pub struct Horizontal {
    pub longitude: f32,
    pub latitude: f32,
}
