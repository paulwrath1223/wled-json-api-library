use serde;
use serde::{Serialize, Deserialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use crate::errors::WledJsonApiError;
use crate::structures::none_function;



#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Net {
    /// list of networks found by most recent scan
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub networks: Option<Vec<Network>>,
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
    /// SSID
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub ssid: Option<String>,

    /// RSSI
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub rssi: Option<i32>,

    /// String MAC / BSSID of scanned wifi
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub bssid: Option<String>,

    /// Wifi channel. normally these are 1-14 but WLED source uses i32 at time of writing
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub channel: Option<i32>,

    /// encryption type (enum wl_enc_type) of the specified item on the networks scanned list
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub enc: Option<ApBehaviourEnum>,
}


/// words do not begin to explain how hard this was to find.
/// its buried in the esp core arduino framework.
#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug, Clone)]
#[allow(non_camel_case_types)]
#[repr(u8)]
pub enum ApBehaviourEnum {
    /** authenticate mode : open */
    WIFI_AUTH_OPEN,
    /** authenticate mode : WEP */
    WIFI_AUTH_WEP,
    /** authenticate mode : WPA_PSK */
    WIFI_AUTH_WPA_PSK,
    /** authenticate mode : WPA2_PSK */
    WIFI_AUTH_WPA2_PSK,
    /** authenticate mode : WPA_WPA2_PSK */
    WIFI_AUTH_WPA_WPA2_PSK,
    /** authenticate mode : WPA2_ENTERPRISE */
    WIFI_AUTH_WPA2_ENTERPRISE,
    /// probaly you found a CIA AP. Run
    WIFI_AUTH_MAX,
    /// Reserved to keep some semblance of backwards compatibility when new versions of whatever the fuck this came from come out
    RSVD1,
    /// Reserved to keep some semblance of backwards compatibility when new versions of whatever the fuck this came from come out
    RSVD2,
    /// Reserved to keep some semblance of backwards compatibility when new versions of whatever the fuck this came from come out
    RSVD3,
    /// Reserved to keep some semblance of backwards compatibility when new versions of whatever the fuck this came from come out
    RSVD4,
    /// Reserved to keep some semblance of backwards compatibility when new versions of whatever the fuck this came from come out
    RSVD5,
    /// Reserved to keep some semblance of backwards compatibility when new versions of whatever the fuck this came from come out
    RSVD6,
    /// Reserved to keep some semblance of backwards compatibility when new versions of whatever the fuck this came from come out
    RSVD7,
    /// Reserved to keep some semblance of backwards compatibility when new versions of whatever the fuck this came from come out
    RSVD8,
    /// Reserved to keep some semblance of backwards compatibility when new versions of whatever the fuck this came from come out
    RSVD9,
}