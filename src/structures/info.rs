use serde;
use serde::{Serialize, Deserialize};
use crate::errors::WledJsonApiError;


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Info {
    /// Version name.
    pub ver: String,

    /// Build ID (YYMMDDB, B = daily build index).
    pub vid: u32,

    /// Contains info about the LED setup.
    pub leds: Leds,

    /// If true, an UI with only a single button for toggling sync should toggle receive+send, otherwise send only
    pub str: bool,

    /// Friendly name of the light. Intended for display in lists and titles.
    pub name: String,

    /// The UDP port for realtime packets and WLED broadcast.
    pub udpport: u16,

    /// If true, the software is currently receiving realtime data via UDP or E1.31.
    pub live: bool,

    /// undocumented??? TODO
    pub liveseg: i64,

    /// Info about the realtime data source
    pub lm: String,

    /// Realtime data source IP address
    pub lip: String,

    /// -1 to 8; Number of currently connected WebSockets clients. -1 indicates that WS is unsupported in this build.
    pub ws: i8,

    /// Number of effects included.
    pub fxcount: u8,

    /// Number of palettes configured.
    pub palcount: u16,

    /// undocumented??? TODO
    pub cpalcount: i64,

    /// undocumented??? TODO
    pub maps: Vec<Map>,

    /// Info about current signal strength
    pub wifi: Wifi,

    /// Info about the embedded LittleFS filesystem (since 0.11.0)
    pub fs: Fs,

    /// -1 to 255; Number of other WLED devices discovered on the network. -1 if Node discovery disabled. (since 0.12.0)
    pub ndc: i16,

    /// Name of the platform.
    pub arch: String,

    /// Version of the underlying (Arduino core) SDK.
    pub core: String,

    /// 0-2; Version of LwIP. 1 or 2 on ESP8266, 0 (does not apply) on ESP32. Deprecated, removal in 0.14.0
    pub lwip: u8,

    /// Bytes of heap memory (RAM) currently available. Problematic if <10k.
    pub freeheap: u32,

    /// Time since the last boot/reset in seconds.
    pub uptime: u32,

    /// undocumented??? TODO
    pub time: String,

    /// Used for debugging purposes only.
    pub opt: u16,

    /// The producer/vendor of the light. Always WLED for standard installations.
    pub brand: String,

    /// The product name. Always FOSS for standard installations.
    pub product: String,

    /// The origin of the build. src if a release version is compiled from source, bin for an official release image, dev for a development build (regardless of src/bin origin) and exp for experimental versions. ogn if the image is flashed to hardware by the vendor. Removed as of v0.10
    #[serde(default)]
    pub btype: String,

    /// The hexadecimal hardware MAC address of the light, lowercase and without colons.
    pub mac: String,

    /// The IP address of this instance. Empty string if not connected. (since 0.13.0)
    pub ip: String,
}

impl TryFrom<&str> for Info{
    type Error = WledJsonApiError;
    fn try_from(str_in: &str) -> Result<Info, WledJsonApiError> {
        serde_json::from_str(str_in).map_err(|e| {WledJsonApiError::SerdeError(e)})
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Leds {
    /// 1 to 1200; Total LED count.
    pub count: u16,

    /// 0 to 65000; Current LED power usage in milliamps as determined by the ABL. 0 if ABL is disabled.
    pub pwr: u16,

    /// Current frames per second. (available since 0.12.0)
    pub fps: u8,

    /// 0 to 65000; Maximum power budget in milliamps for the ABL. 0 if ABL is disabled.
    pub maxpwr: i64,

    /// Maximum number of segments supported by this version.
    pub maxseg: u8,

    /// Per-segment virtual light capabilities
    pub seglc: Vec<u8>,

    /// Logical AND of all active segment's virtual light capabilities
    pub lc: u8,

    /// true if LEDs are 4-channel (RGB + White). (deprecated, use info.leds.lc)
    pub rgbw: bool,

    /// WLED WIKI SAYS BOOL??? true if a white channel slider should be displayed. (available since 0.10.0, deprecated, use info.leds.lc)
    pub wv: i64,

    /// WLED WIKI SAYS BOOL??? true if the light supports color temperature control (available since 0.13.0, deprecated, use info.leds.lc)
    pub cct: i64,

    /// LED strip pin(s). Always one element. Removed as of v0.13
    #[serde(default)]
    pub pin: Vec<u8>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Map {
    /// undocumented??? TODO
    pub id: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Wifi {
    /// The BSSID of the currently connected network.
    pub bssid: String,

    /// undocumented??? (rssi is an oversimplified measure of signal strength, just use signal)TODO
    pub rssi: i64,

    /// 0-100; Relative signal quality of the current connection.
    pub signal: u8,

    /// 1 to 14; The current WiFi channel.
    pub channel: u8,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Fs {
    /// Estimate of used filesystem space in kilobytes
    pub u: u32,

    /// Total filesystem size in kilobytes
    pub t: u32,

    /// Unix timestamp for the last modification to the presets.json file. Not accurate after boot or after using /edit
    pub pmt: u32,
}

#[cfg(test)]
mod tests {
    use crate::structures::info::Info;

    #[test]
    fn it_works() {
        let a: Info = Info::try_from(r#"{"ver":"0.14.0","vid":2310130,"leds":{"count":6,"pwr":0,"fps":5,"maxpwr":0,"maxseg":32,"seglc":[1],"lc":1,"rgbw":false,"wv":0,"cct":0},"str":false,"name":"WLED","udpport":21324,"live":false,"liveseg":-1,"lm":"","lip":"","ws":0,"fxcount":187,"palcount":71,"cpalcount":0,"maps":[{"id":0}],"wifi":{"bssid":"FC:EC:DA:A4:C4:77","rssi":-60,"signal":80,"channel":1},"fs":{"u":12,"t":983,"pmt":0},"ndc":0,"arch":"esp32","core":"v3.3.6-16-gcc5440f6a2","lwip":0,"freeheap":200300,"uptime":6,"time":"1970-1-1, 00:00:06","opt":79,"brand":"WLED","product":"FOSS","mac":"a842e38d9828","ip":"192.168.1.40"}"#).unwrap();
        println!("{:?}", a);
    }
}
