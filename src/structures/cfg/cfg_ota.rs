use serde;
use serde::{Serialize, Deserialize};
use crate::structures::none_function;



#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ota {
    /// prevents OTA firmware updates without password. ALWAYS enable if system exposed to any public networks
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub lock: Option<bool>,

    /// prevents access to WiFi settings when OTA lock is enabled
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    #[serde(rename = "lock-wifi")]
    pub lock_wifi: Option<bool>,

    /// length of OTA password
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub pskl: Option<u8>,

    /// ArduinoOTA allows easy updates directly from the IDE. Careful, it does not auto-disable when OTA lock is on
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub aota: Option<bool>,
}