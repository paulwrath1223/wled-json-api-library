use serde;
use serde::{Serialize, Deserialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use crate::structures::none_function;




/// Information about the access point that the ESP hosts when enabled, or when connecting to other AP fails
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ap {
    /// SSID of local AP
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub ssid: Option<String>,

    /// Length of AP password (password is wled1234 by default if I remember correctly)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub pskl: Option<usize>,

    /// 2.4GHz WiFi AP channel (1-13)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub chan: Option<u8>,

    /// hidden AP SSID, no idea why this is a byte but it is
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub hide: Option<u8>,

    /// access point opens when no connection after boot by default
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub behav: Option<ApBehaviourEnum>,

    /// IP to host the website when on AP (default 4.3.2.1)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub ip: Option<[u8; 4]>,
}



#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug, Clone)]
#[repr(u8)]
pub enum ApBehaviourEnum {
    /// Open AP when no connection after boot
    ApBehaviorBootNoConn,
    /// Open when no connection (either after boot or if connection is lost)
    ApBehaviorNoConn,
    /// Always open
    ApBehaviorAlways,
    /// Only when button pressed for 6 sec
    ApBehaviorButtonOnly,
    /// Reserved to keep some semblance of backwards compatibility when new WLED versions come out with more AP behaviours
    RSVD1,
    /// Reserved to keep some semblance of backwards compatibility when new WLED versions come out with more AP behaviours
    RSVD2,
    /// Reserved to keep some semblance of backwards compatibility when new WLED versions come out with more AP behaviours
    RSVD3,
    /// Reserved to keep some semblance of backwards compatibility when new WLED versions come out with more AP behaviours
    RSVD4,
    /// Reserved to keep some semblance of backwards compatibility when new WLED versions come out with more AP behaviours
    RSVD5,
    /// Reserved to keep some semblance of backwards compatibility when new WLED versions come out with more AP behaviours
    RSVD6,

}