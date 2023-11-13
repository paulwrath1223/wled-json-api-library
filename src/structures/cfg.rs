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
    pub eth: Eth, //TODO
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

    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub led: Option<Led>,

    ///
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
    pub total: i64,
    pub maxpwr: i64,
    pub ledma: i64,
    pub cct: bool,
    pub cr: bool,
    pub cb: i64,
    pub fps: i64,
    pub rgbwm: i64,
    pub ld: bool,
    pub ins: Vec<In2>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct In2 {
    pub start: i64,
    pub len: i64,
    pub pin: Vec<i64>,
    pub order: i64,
    pub rev: bool,
    pub skip: i64,
    #[serde(rename = "type")]
    pub type_field: i64,
    #[serde(rename = "ref")]
    pub ref_field: bool,
    pub rgbwm: i64,
    pub freq: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Btn {
    pub max: i64,
    pub pull: bool,
    pub ins: Vec<In3>,
    pub tt: i64,
    pub mqtt: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct In3 {
    #[serde(rename = "type")]
    pub type_field: i64,
    pub pin: Vec<i64>,
    pub macros: Vec<i64>,
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
    pub start: u16,
    pub stop: u16,
    pub order: u8,

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
    // TODO; wled source below


    /*
  JsonObject ethernet = doc.createNestedObject("eth");
  ethernet["type"] = ethernetType;
  if (ethernetType != WLED_ETH_NONE && ethernetType < WLED_NUM_ETH_TYPES) {
    JsonArray pins = ethernet.createNestedArray("pin");
    for (uint8_t p=0; p<WLED_ETH_RSVD_PINS_COUNT; p++) pins.add(esp32_nonconfigurable_ethernet_pins[p].pin);
    if (ethernetBoards[ethernetType].eth_power>=0)     pins.add(ethernetBoards[ethernetType].eth_power);
    if (ethernetBoards[ethernetType].eth_mdc>=0)       pins.add(ethernetBoards[ethernetType].eth_mdc);
    if (ethernetBoards[ethernetType].eth_mdio>=0)      pins.add(ethernetBoards[ethernetType].eth_mdio);
    switch (ethernetBoards[ethernetType].eth_clk_mode) {
      case ETH_CLOCK_GPIO0_IN:
      case ETH_CLOCK_GPIO0_OUT:
        pins.add(0);
        break;
      case ETH_CLOCK_GPIO16_OUT:
        pins.add(16);
        break;
      case ETH_CLOCK_GPIO17_OUT:
        pins.add(17);
        break;
    }
  }
    */
}


#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug, Clone)]
#[repr(u8)]
pub enum ApBehaviourEnum {
    /// Open AP when no connection after boot
    ApBehaviorBootNoConn = 0,
    /// Open when no connection (either after boot or if connection is lost)
    ApBehaviorNoConn = 1,
    /// Always open
    ApBehaviorAlways = 2,
    /// Only when button pressed for 6 sec
    ApBehaviorButtonOnly = 3,
}

