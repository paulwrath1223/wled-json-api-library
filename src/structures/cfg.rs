use serde;
use serde::{Serialize, Deserialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use crate::errors::WledJsonApiError;
use crate::structures::none_function;


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cfg {
    /// Version of WLED ("1.0.2", for example is [1, 0, 2])
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub rev: Option<Vec<u32>>,

    /// Version ID; version code in format yymmddb (b = daily build) (macro called "VERSION" in wled source)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub vid: Option<u64>,

    /// identifying information
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub id: Option<Id>,

    pub nw: Nw,
    pub eth: Eth,
    pub ap: Ap,
    pub wifi: Wifi,
    pub hw: Hw,
    pub light: Light,
    pub def: Def,
    #[serde(rename = "if")]
    pub if_field: If2,
    pub remote: Remote,
    pub ol: Ol,
    pub timers: Timers,
    pub ota: Ota,
    pub um: Um,
}


impl TryFrom<&str> for Cfg{
    type Error = WledJsonApiError;
    fn try_from(str_in: &str) -> Result<Cfg, WledJsonApiError> {
        serde_json::from_str(str_in).map_err(|e| {WledJsonApiError::SerdeError(e)})
    }
}


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Id {
    ///mDNS address (*.local, replaced by wledXXXXXX if default is used)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub mdns: Option<String>,

    /// Server Description; Name of module - use default
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub name: Option<String>,

    /// Alexa invocation name; speech control name of device. Choose something voice-to-text can understand
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub inv: Option<String>,

    /// Simplified UI;
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub sui: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Nw {
    /// honestly no idea why this is a vector, WLED source only uses one element
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub ins: Option<Vec<In>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct In {
    /// SSID of the network to connect to
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub ssid: Option<String>,

    /// Length of the wifi password
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub pskl: Option<usize>,

    /// static IP of ESP
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub ip: Option<[u8; 4]>,

    /// gateway (router) IP
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub gw: Option<[u8; 4]>,

    /// most common subnet in home networks is 255:255:255:0
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub sn: Option<[u8; 4]>,
}

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

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Wifi {

    /// IDFK you're on your own
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub sleep: Option<bool>,
}

/// Hardware info
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Hw {

    /// Led info
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub led: Option<Led>,

    /// List of color order maps
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub com: Option<Vec<ColorOrderMap>>,

    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub btn: Option<Btn>,

    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub ir: Option<Ir>,

    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub relay: Option<Relay>,

    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub baud: Option<i64>,

    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    #[serde(rename = "if")]
    pub if_field: Option<If>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Led {
    /// get total legth; no longer read as of WLED 14.0, but provided for compatibility on downgrade
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub total: Option<u16>,

    /// max milliamps
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub maxpwr: Option<u16>,

    /// milliamps per LED
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub ledma: Option<u8>,

    /// CCT color correction of RGB color
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub cct: Option<bool>,

    /// CCT is calculated from RGB instead of using seg.cct
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub cr: Option<bool>,

    /// CCT Blending
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub cb: Option<u8>,

    /// Target FPS
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub fps: Option<u8>,

    /// global auto white mode override; 255 = Global auto white mode override disabled. Per-bus setting is used
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub rgbwm: Option<u8>,

    /// use Global Led Buffer
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub ld: Option<bool>,

    /// if using 2d WLED
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub matrix: Option<Matrix>,

    /// WLED busses
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub ins: Option<Vec<In2>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct In2 {

    /// start ig?
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub start: Option<u16>,

    /// length ig?
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub len: Option<u16>,

    /// list of pins for some fucking reason
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub pin: Option<Vec<u8>>,

    /// Color order
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub order: Option<u8>,

    /// is reversed?
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub rev: Option<bool>,

    /// skipped LEDs??
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub skip: Option<u8>,

    /// Light type. see "LightCapability" enum documentation
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    #[serde(rename = "type")]
    pub type_field: Option<LightCapability>,

    /// is Off Refresh Required
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    #[serde(rename = "ref")]
    pub ref_field: Option<bool>,

    /// get Auto White Mode
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub rgbwm: Option<u8>,

    /// frequency?? IG?
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub freq: Option<u16>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Btn {

    /// just information about max number of buttons (not actually used)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub max: Option<u8>,

    /// idfk
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub pull: Option<bool>,

    /// balls, testicles
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub ins: Option<Vec<In3>>,

    /// Touch threshold
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub tt: Option<u8>,

    /// button Publish Mqtt
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub mqtt: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct In3 {

    /// Button type
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    #[serde(rename = "type")]
    pub type_field: Option<ButtonType>,

    /// Button pin
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub pin: Option<Vec<i8>>,

    /// frequency?? IG?
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub macros: Option<Vec<i64>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ir {
    pub pin: i64,
    #[serde(rename = "type")]
    pub type_field: i64,
    pub sel: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Relay {
    pub pin: i64,
    pub rev: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct If {
    #[serde(rename = "i2c-pin")]
    pub i2c_pin: Vec<i64>,
    #[serde(rename = "spi-pin")]
    pub spi_pin: Vec<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Light {
    #[serde(rename = "scale-bri")]
    pub scale_bri: i64,
    #[serde(rename = "pal-mode")]
    pub pal_mode: i64,
    pub aseg: bool,
    pub gc: Gc,
    pub tr: Tr,
    pub nl: Nl,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Gc {
    pub bri: i64,
    pub col: f64,
    pub val: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tr {
    pub mode: bool,
    pub dur: i64,
    pub pal: i64,
    pub rpc: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Nl {
    pub mode: i64,
    pub dur: i64,
    pub tbri: i64,
    #[serde(rename = "macro")]
    pub macro_field: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Def {
    pub ps: i64,
    pub on: bool,
    pub bri: i64,
}

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
    pub port0: i64,
    pub port1: i64,
    pub recv: Recv,
    pub send: Send,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Recv {
    pub bri: bool,
    pub col: bool,
    pub fx: bool,
    pub grp: i64,
    pub seg: bool,
    pub sb: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Send {
    pub dir: bool,
    pub btn: bool,
    pub va: bool,
    pub hue: bool,
    #[serde(rename = "macro")]
    pub macro_field: bool,
    pub grp: i64,
    pub ret: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Nodes {
    pub list: bool,
    pub bcast: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Live {
    pub en: bool,
    pub mso: bool,
    pub port: i64,
    pub mc: bool,
    pub dmx: Dmx,
    pub timeout: i64,
    pub maxbri: bool,
    #[serde(rename = "no-gc")]
    pub no_gc: bool,
    pub offset: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Dmx {
    pub uni: i64,
    pub seqskip: bool,
    pub e131prio: i64,
    pub addr: i64,
    pub dss: i64,
    pub mode: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Va {
    pub alexa: bool,
    pub macros: Vec<i64>,
    pub p: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mqtt {
    pub en: bool,
    pub broker: String,
    pub port: i64,
    pub user: String,
    pub pskl: i64,
    pub cid: String,
    pub rtn: bool,
    pub topics: Topics,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Topics {
    pub device: String,
    pub group: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Hue {
    pub en: bool,
    pub id: i64,
    pub iv: i64,
    pub recv: Recv2,
    pub ip: Vec<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Recv2 {
    pub on: bool,
    pub bri: bool,
    pub col: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ntp {
    pub en: bool,
    pub host: String,
    pub tz: i64,
    pub offset: i64,
    pub ampm: bool,
    pub ln: i64,
    pub lt: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Remote {
    #[serde(rename = "remote_enabled")]
    pub remote_enabled: bool,
    #[serde(rename = "linked_remote")]
    pub linked_remote: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ol {
    pub clock: i64,
    pub cntdwn: bool,
    pub min: i64,
    pub max: i64,
    pub o12pix: i64,
    pub o5m: bool,
    pub osec: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Timers {
    pub cntdwn: Cntdwn,
    pub ins: Vec<Ins>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cntdwn {
    pub goal: Goal,
    #[serde(rename = "macro")]
    pub macro_field: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ota {
    pub lock: bool,
    #[serde(rename = "lock-wifi")]
    pub lock_wifi: bool,
    pub pskl: i64,
    pub aota: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Um {
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ColorOrderMap {

    /// Probably the start index?
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub start: Option<u16>,

    /// stop index? probably?, could not find in latest source, but its in the WLED 14.0 binary
    #[serde(skip_serializing)]
    #[serde(default = "none_function")]
    pub stop: Option<u16>,

    /// length ig?
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub len: Option<u16>,

    /// Whatever the fuck 'order' is
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub order: Option<u8>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Goal {
    #[serde(rename = "0")]
    pub year: u8,
    #[serde(rename = "1")]
    pub month: u8,
    #[serde(rename = "2")]
    pub day: u8,
    #[serde(rename = "3")]
    pub hour: u8,
    #[serde(rename = "4")]
    pub minute: u8,
    #[serde(rename = "5")]
    pub second: u8,
}


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ins {
    pub en: bool,
    pub hour: u8,
    pub min: i8,
    #[serde(rename = "macro")]
    pub macro_field: u8,
    pub start: MonthDay,
    pub end: MonthDay,
}


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MonthDay {
    pub mon: u8,
    pub day: u8,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Eth {
    /// type of ethernet
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub relay: Option<EthType>,

    /// pins used??? idk how this one works
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub pin: Option<Vec<u32>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Matrix {
    /// fuck if I know (strip.panels in WLED source)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub mpc: Option<u8>,

    /// panels
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub panels: Option<Vec<Panel>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Panel {
    /// starts at bottom?
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub b: Option<bool>,

    /// starts on right?
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub r: Option<bool>,

    /// is vertical?
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub v: Option<bool>,

    /// is serpentine?
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub s: Option<bool>,

    /// x offset relative to the top left of matrix in LEDs
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub x: Option<u16>,

    /// y offset relative to the top left of matrix in LEDs
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub y: Option<u16>,

    /// height of the panel
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub h: Option<u8>,

    /// width of the panel
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub w: Option<u8>,

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

/// Various types
#[allow(non_camel_case_types)]
#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug, Clone)]
#[repr(u8)]
pub enum ButtonType {
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

/// Constants defined for Ethernet types
#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug, Clone)]
#[repr(u8)]
pub enum EthType {
    WledEthNone,
    WledEthWt32Eth01,
    WledEthEsp32Poe,
    WledEthWesp32,
    WledEthQuinled,
    WledEthTwilightlord,
    WledEthEsp32deux,
    WledEthEsp32ethkitve,
    WledEthQuinledOcta,
    WledEthAbcwledv43eth,
    WledEthSerg74,
    /// Reserved to keep some semblance of backwards compatibility when new WLED versions come out with more ethernet types
    RSVD1,
    /// Reserved to keep some semblance of backwards compatibility when new WLED versions come out with more ethernet types
    RSVD2,
    /// Reserved to keep some semblance of backwards compatibility when new WLED versions come out with more ethernet types
    RSVD3,
    /// Reserved to keep some semblance of backwards compatibility when new WLED versions come out with more ethernet types
    RSVD4,
    /// Reserved to keep some semblance of backwards compatibility when new WLED versions come out with more ethernet types
    RSVD5,
    /// Reserved to keep some semblance of backwards compatibility when new WLED versions come out with more ethernet types
    RSVD6,
    /// Reserved to keep some semblance of backwards compatibility when new WLED versions come out with more ethernet types
    RSVD7,
    /// Reserved to keep some semblance of backwards compatibility when new WLED versions come out with more ethernet types
    RSVD8,

}

///     Light capability byte (unused) 0bRCCCTTTT
///     bits 0/1/2/3: specifies a type of LED driver. A single "driver" may have different chip models but must have the same protocol/behavior
///     bits 4/5/6: specifies the class of LED driver - 0b000 (dec. 0-15)  unconfigured/reserved
///                                                   - 0b001 (dec. 16-31) digital (data pin only)
///                                                   - 0b010 (dec. 32-47) analog (PWM)
///                                                   - 0b011 (dec. 48-63) digital (data + clock / SPI)
///                                                   - 0b100 (dec. 64-79) unused/reserved
///                                                   - 0b101 (dec. 80-95) virtual network busses
///                                                   - 0b110 (dec. 96-111) unused/reserved
///                                                   - 0b111 (dec. 112-127) unused/reserved
///     bit 7 is reserved and set to 0
#[allow(non_camel_case_types)]
#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug, Clone)]
#[repr(u8)]
pub enum LightCapability {
    /// light is not configured
    TYPE_NONE = 0,
    ///unused. Might indicate a "virtual"
    TYPE_RESERVED = 1,

    //Digital types (data pin only) (16-31)

    ///white-only chips (1 channel per IC) (unused)
    TYPE_WS2812_1CH = 18,
    ///white-only chips (3 channels per IC)
    TYPE_WS2812_1CH_X3 = 19,
    ///CCT chips (1st IC controls WW + CW of 1st zone and CW of 2nd zone, 2nd IC controls WW of 2nd zone and WW + CW of 3rd zone)
    TYPE_WS2812_2CH_X3 = 20,
    ///amber + warm + cold white
    TYPE_WS2812_WWA = 21,

    TYPE_WS2812_RGB = 22,
    ///same driver as WS2812, but will require signal 2x per second (else displays test pattern)
    TYPE_GS8608 = 23,
    ///half-speed WS2812 protocol, used by very old WS2811 units
    TYPE_WS2811_400KHZ = 24,

    TYPE_TM1829 = 25,

    TYPE_UCS8903 = 26,

    TYPE_UCS8904 = 29,

    TYPE_SK6812_RGBW = 30,

    TYPE_TM1814 = 31,

    //"Analog" types (PWM) (32-47)

    ///binary output (relays etc.)
    TYPE_ONOFF = 40,
    ///single channel PWM. Uses value of brightest RGBW channel
    TYPE_ANALOG_1CH = 41,
    ///analog WW + CW
    TYPE_ANALOG_2CH = 42,
    ///analog RGB
    TYPE_ANALOG_3CH = 43,
    ///analog RGBW
    TYPE_ANALOG_4CH = 44,
    ///analog RGB + WW
    TYPE_ANALOG_5CH = 45,

    // Digital types (data + clock / SPI) (48-63)


    TYPE_WS2801 = 50,

    TYPE_APA102 = 51,

    TYPE_LPD8806 = 52,

    TYPE_P9813 = 53,

    TYPE_LPD6803 = 54,

    //Network types (master broadcast) (80-95)

    ///network DDP RGB bus (master broadcast bus)
    TYPE_NET_DDP_RGB = 80,
    ///network E131 RGB bus (master broadcast bus, unused)
    TYPE_NET_E131_RGB = 81,
    ///network ArtNet RGB bus (master broadcast bus, unused)
    TYPE_NET_ARTNET_RGB = 82,
    ///network DDP RGBW bus (master broadcast bus)
    TYPE_NET_DDP_RGBW = 88,

    /// Reserved to keep some semblance of backwards compatibility when new WLED versions come out with more ethernet types
    RSVD1,
    /// Reserved to keep some semblance of backwards compatibility when new WLED versions come out with more ethernet types
    RSVD2,
    /// Reserved to keep some semblance of backwards compatibility when new WLED versions come out with more ethernet types
    RSVD3,
    /// Reserved to keep some semblance of backwards compatibility when new WLED versions come out with more ethernet types
    RSVD4,
    /// Reserved to keep some semblance of backwards compatibility when new WLED versions come out with more ethernet types
    RSVD5,
    /// Reserved to keep some semblance of backwards compatibility when new WLED versions come out with more ethernet types
    RSVD6,
    /// Reserved to keep some semblance of backwards compatibility when new WLED versions come out with more ethernet types
    RSVD7,
    /// Reserved to keep some semblance of backwards compatibility when new WLED versions come out with more ethernet types
    RSVD8,

}