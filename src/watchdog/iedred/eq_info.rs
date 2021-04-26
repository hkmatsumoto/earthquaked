use serde::Deserialize;

#[derive(Deserialize, PartialEq, Eq, Hash, Debug)]
#[serde(untagged)]
pub enum IedredEqInfo {
    Warning {
        #[serde(rename = "ParseStatus")]
        parse_status: String,
        #[serde(rename = "Title")]
        title: Title,
        #[serde(rename = "AnnouncedTime")]
        announced_time: AnnouncedTime,
        #[serde(rename = "Warn")]
        warn: bool,
        #[serde(rename = "WarnForecast")]
        warn_forecast: Vec<WarnForecast>,
    },
    Forecast {
        #[serde(rename = "ParseStatus")]
        parse_status: String,
        #[serde(rename = "Title")]
        title: Title,
        #[serde(rename = "AnnouncedTime")]
        announced_time: AnnouncedTime,
        #[serde(rename = "Warn")]
        warn: bool,
    },
}

impl IedredEqInfo {
    pub fn announced_time(&self) -> i64 {
        match self {
            IedredEqInfo::Warning { announced_time, .. }
            | IedredEqInfo::Forecast { announced_time, .. } => announced_time.unix_time,
        }
    }
}

#[derive(Deserialize, PartialEq, Eq, Hash, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct WarnForecast {
    district: Vec<String>,
    local_areas: Vec<String>,
    regions: Vec<String>,
}

#[derive(Deserialize, PartialEq, Eq, Hash, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Title {
    code: u8,
    string: String,
    detail: String,
}

#[derive(Deserialize, PartialEq, Eq, Hash, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct AnnouncedTime {
    unix_time: i64,
}
