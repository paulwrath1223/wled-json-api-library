use serde;
use serde::{Serialize, Deserialize};
use crate::errors::WledJsonApiError;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cfg {
    pub rev: Vec<i64>,
    pub vid: i64,
    pub id: Id,
    pub nw: Nw,
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
    pub mdns: String,
    pub name: String,
    pub inv: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Nw {
    pub ins: Vec<In>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct In {
    pub ssid: String,
    pub pskl: i64,
    pub ip: Vec<i64>,
    pub gw: Vec<i64>,
    pub sn: Vec<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ap {
    pub ssid: String,
    pub pskl: i64,
    pub chan: i64,
    pub hide: i64,
    pub behav: i64,
    pub ip: Vec<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Wifi {
    pub sleep: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Hw {
    pub led: Led,
    pub com: Vec<ColorOrderMap>,
    pub btn: Btn,
    pub ir: Ir,
    pub relay: Relay,
    pub baud: i64,
    #[serde(rename = "if")]
    pub if_field: If,
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


