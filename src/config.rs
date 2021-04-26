use crate::Location;

use serde::Deserialize;

#[derive(Deserialize, Default, Debug)]
pub struct Config {
    pub location: Location,
    pub commands: Vec<String>,
}

impl Config {
    pub fn load<P: AsRef<std::path::Path>>(path: P) -> Config {
        std::fs::File::open(path.as_ref())
            .ok()
            .and_then(|file| serde_yaml::from_reader(file).ok())
            .unwrap_or_default()
    }
}
