use serde;
use serde::{Serialize, Deserialize};
use crate::errors::WledJsonApiError;


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Net {
    pub networks: Vec<Network>,
}

impl TryFrom<&str> for Net{
    type Error = WledJsonApiError;
    fn try_from(str_in: &str) -> Result<Net, WledJsonApiError> {
        serde_json::from_str(str_in).map_err(|e| {WledJsonApiError::SerdeError(e)})
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Network {
    pub ssid: String,
    pub rssi: i64,
    pub bssid: String,
    pub channel: i64,
    pub enc: i64,
}
