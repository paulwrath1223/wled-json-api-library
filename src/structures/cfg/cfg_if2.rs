use serde;
use serde::{Serialize, Deserialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use crate::errors::WledJsonApiError;
use crate::structures::none_function;



#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct If2 {
    pub sync: Sync,
    pub nodes: Nodes,
    pub live: Live,
    pub va: Va,
    pub mqtt: Mqtt,
    pub hue: Hue,
    pub ntp: Ntp,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sync {
    /// WLED notifier default port
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub port0: Option<u16>,

    /// WLED notifier supplemental port
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub port1: Option<u16>,

    /// WLED notifier receive info
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub recv: Option<Recv>,

    /// WLED notifier send info
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub send: Option<Send>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Recv {

    /// apply brightness from incoming notifications
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub bri: Option<bool>,

    /// apply color
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub col: Option<bool>,

    /// apply effects setup
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub fx: Option<bool>,

    /// sync receive groups this instance belongs to (bit mapped)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub grp: Option<u8>,

    /// apply segment options
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub seg: Option<bool>,

    /// apply segment bounds (start, stop, offset)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub sb: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Send {
    /// send notification if change via UI or HTTP API
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub dir: Option<bool>,

    /// send if updated by button or infrared remote
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub btn: Option<bool>,

    /// send notification if updated via Alexa
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub va: Option<bool>,

    /// send notification if Hue light changes
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub hue: Option<bool>,

    /// send notification for macro
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    #[serde(rename = "macro")]
    pub macro_field: Option<bool>,

    /// sync groups this instance syncs (bit mapped)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub grp: Option<u8>,

    /// Number of times a UDP sync message is retransmitted. Increase to increase reliability
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub ret: Option<u8>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Nodes {
    /// node List Enabled
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub list: Option<bool>,

    /// node Broadcast Enabled
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub bcast: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Live {
    /// receive UDP realtime
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub en: Option<bool>,

    /// use Main Segment Only
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub mso: Option<bool>,

    /// DMX in port. E1.31 default is 5568, Art-Net is 6454
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub port: Option<u16>,

    /// multicast or unicast; who the fuck knows which one is which
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub mc: Option<bool>,

    /// DMX info
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub dmx: Option<Dmx>,

    /// (ms timeout of realtime mode before returning to normal mode) / 100
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub timeout: Option<u16>,

    /// enable to force max brightness if source has very dark colors that would be black
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub maxbri: Option<bool>,

    /// Disable gamma correction; activate if gamma correction is handled by the source
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    #[serde(rename = "no-gc")]
    pub no_gc: Option<bool>,

    /// realtime LED offset
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub offset: Option<u32>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Dmx {
    /// settings for E1.31 (sACN) protocol (only DMX_MODE_MULTIPLE_* can span over consequtive universes)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub uni: Option<u16>,

    /// freeze instead of flickering
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub seqskip: Option<bool>,

    /// E1.31 port priority (if != 0 priority handling is active)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub e131prio: Option<u8>,

    /// DMX start address of fixture, a.k.a. first Channel [for E1.31 (sACN) protocol]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub addr: Option<u16>,

    /// DMX channel spacing; Number of void/unused channels between each segments DMX channels
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub dss: Option<u16>,

    /// DMX mode
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub mode: Option<DmxMode>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Va {
    /// enable device discovery by Amazon Echo
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub alexa: Option<bool>,

    /// [macroAlexaOn, macroAlexaOff]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub macros: Option<[u8; 2]>,

    /// number of presets to expose to Alexa, starting from preset 1, up to 9
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub p: Option<u8>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mqtt {
    /// MQTT enabled?
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub en: Option<bool>,

    /// both domains and IPs should work (no SSL)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub broker: Option<String>,

    /// MQTT port
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub port: Option<u16>,

    /// optional: username for MQTT auth
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub user: Option<String>,

    /// optional: length of password for MQTT auth
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub pskl: Option<u8>,

    /// override the client ID
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub cid: Option<String>,

    /// retain brightness and color
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub rtn: Option<bool>,

    /// mqtt topics
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub topics: Option<Topics>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Topics {
    /// main MQTT topic (individual per device, default is wled/mac)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub device: Option<String>,

    /// second MQTT topic (for example to group devices)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub group: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Hue {
    /// poll hue bridge for light state
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub en: Option<bool>,

    /// ID of hue lamp to sync to. Find the ID in the hue app ("about" section)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub id: Option<u8>,

    /// second MQTT topic (for example to group devices)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub iv: i64,

    /// second MQTT topic (for example to group devices)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub recv: Recv2,

    /// second MQTT topic (for example to group devices)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub ip: Vec<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Recv2 {
    /// second MQTT topic (for example to group devices)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub on: bool,

    /// second MQTT topic (for example to group devices)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub bri: bool,

    /// second MQTT topic (for example to group devices)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub col: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ntp {
    /// second MQTT topic (for example to group devices)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub en: bool,

    /// second MQTT topic (for example to group devices)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub host: String,

    /// second MQTT topic (for example to group devices)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub tz: i64,

    /// second MQTT topic (for example to group devices)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub offset: i64,

    /// second MQTT topic (for example to group devices)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub ampm: bool,

    /// second MQTT topic (for example to group devices)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub ln: i64,

    /// second MQTT topic (for example to group devices)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub lt: i64,
}


/// DMX modes
#[allow(non_camel_case_types)]
#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug, Clone)]
#[repr(u8)]
pub enum DmxMode {
    /// Not used
    DMX_MODE_DISABLED = 0,
    /// all LEDs same RGB color (3 channels)
    DMX_MODE_SINGLE_RGB = 1,
    /// all LEDs same RGB color and master dimmer (4 channels)
    DMX_MODE_SINGLE_DRGB = 2,
    /// trigger standalone effects of WLED (15 channels)
    DMX_MODE_EFFECT = 3,
    /// trigger standalone effects of WLED (18 channels)
    DMX_MODE_EFFECT_W = 7,
    /// every LED is addressed with its own RGB (ledCount * 3 channels)
    DMX_MODE_MULTIPLE_RGB = 4,
    /// every LED is addressed with its own RGB and share a master dimmer (ledCount * 3 + 1 channels)
    DMX_MODE_MULTIPLE_DRGB = 5,
    /// every LED is addressed with its own RGBW (ledCount * 4 channels)
    DMX_MODE_MULTIPLE_RGBW = 6,
    /// trigger standalone effects of WLED (15 channels per segment)
    DMX_MODE_EFFECT_SEGMENT = 8,
    /// trigger standalone effects of WLED (18 channels per segment)
    DMX_MODE_EFFECT_SEGMENT_W = 9,
    /// apply presets (1 channel)
    DMX_MODE_PRESET = 10,
    /// Reserved to keep some semblance of backwards compatibility when new WLED versions come out with more Night Light modes
    RSVD1,
    /// Reserved to keep some semblance of backwards compatibility when new WLED versions come out with more Night Light modes
    RSVD2,
    /// Reserved to keep some semblance of backwards compatibility when new WLED versions come out with more Night Light modes
    RSVD3,
    /// Reserved to keep some semblance of backwards compatibility when new WLED versions come out with more Night Light modes
    RSVD4,
    /// Reserved to keep some semblance of backwards compatibility when new WLED versions come out with more Night Light modes
    RSVD5,
    /// Reserved to keep some semblance of backwards compatibility when new WLED versions come out with more Night Light modes
    RSVD6,

}