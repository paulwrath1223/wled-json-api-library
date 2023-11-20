use serde;
use serde::{Serialize, Deserialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use crate::errors::WledJsonApiError;
use crate::structures::none_function;


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Info {
    /// Version name.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub ver: Option<String>,

    /// Build ID (YYMMDDB, B = daily build index).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub vid: Option<u32>,

    /// Contains info about the LED setup.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub leds: Option<Leds>,

    /// sync Toggle Receive
    /// UIs which only have a single button for sync should toggle send+receive if this is true, only send otherwise
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub str: Option<bool>,

    /// Friendly name of the light. Intended for display in lists and titles.
    /// Name of module - default is WLED
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub name: Option<String>,

    /// The UDP port for realtime packets and WLED broadcast.
    /// WLED notifier default port
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub udpport: Option<u16>,

    /// If true, the software is currently receiving realtime data via UDP or E1.31.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub live: Option<bool>,

    /// main segment id if its active, -1 otherwise
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub liveseg: Option<i16>,

    /// Info about the realtime data source
    /// WLED SOURCE (as of ~wled 14.0:
    ///   switch (realtimeMode) {
    ///     case REALTIME_MODE_INACTIVE: root["lm"] = ""; break;
    ///     case REALTIME_MODE_GENERIC:  root["lm"] = ""; break;
    ///     case REALTIME_MODE_UDP:      root["lm"] = F("UDP"); break;
    ///     case REALTIME_MODE_HYPERION: root["lm"] = F("Hyperion"); break;
    ///     case REALTIME_MODE_E131:     root["lm"] = F("E1.31"); break;
    ///     case REALTIME_MODE_ADALIGHT: root["lm"] = F("USB Adalight/TPM2"); break;
    ///     case REALTIME_MODE_ARTNET:   root["lm"] = F("Art-Net"); break;
    ///     case REALTIME_MODE_TPM2NET:  root["lm"] = F("tpm2.net"); break;
    ///     case REALTIME_MODE_DDP:      root["lm"] = F("DDP"); break;
    ///   }
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub lm: Option<String>,

    /// Realtime data source IP address
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub lip: Option<String>,

    /// -1 to 8; Number of currently connected WebSockets clients. -1 indicates that WS is unsupported in this build.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub ws: Option<i8>,

    /// Number of effects included.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub fxcount: Option<u8>,

    /// Number of palettes configured.
    /// will only return built-in palette count
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub palcount: Option<u16>,

    /// custom palette count
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub cpalcount: Option<u16>,

    /// available ledmaps
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub maps: Option<Vec<Map>>,

    /// Info about wifi
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub wifi: Option<Wifi>,

    /// Info about the embedded LittleFS filesystem (since 0.11.0)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub fs: Option<Fs>,

    /// -1 to 255; Number of other WLED devices discovered on the network. -1 if Node discovery disabled. (since 0.12.0)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub ndc: Option<i16>,

    /// only present on debug builds
    /// (int) WiFi.getTxPower();
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    #[serde(rename = "txPower")]
    pub tx_power: Option<u32>,

    /// only present on debug builds
    /// (bool) WiFi.getSleep();
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub sleep: Option<bool>,

    /// Name of the platform.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub arch: Option<String>,

    /// Version of the underlying (Arduino core) SDK.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub core: Option<String>,

    /// only present on debug esp32 builds
    /// (int)rtc_get_reset_reason(0);
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    #[serde(rename = "resetReason0")]
    pub reset_reason_0: Option<u32>,

    /// only present on debug esp32 builds
    /// (int)rtc_get_reset_reason(1);
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    #[serde(rename = "resetReason1")]
    pub reset_reason_1: Option<u32>,

    /// only present on debug esp8266 builds
    /// (int)rtc_get_reset_reason(0);
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    #[serde(rename = "resetReason")]
    pub reset_reason: Option<u32>,

    /// 0-2; Version of LwIP. 1 or 2 on ESP8266, 0 (does not apply) on ESP32. Deprecated, removal in 0.14.0
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub lwip: Option<u8>,

    /// Bytes of heap memory (RAM) currently available. Problematic if <10k.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub freeheap: Option<u32>,

    /// ESP.getFreePsram(); only present when hardware supports psram
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub psram: Option<u64>,

    /// Time since the last boot/reset in seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub uptime: Option<u32>,

    /// The current time in human readable format
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub time: Option<String>,

    /// Used for debugging purposes only. bit map of build info
    ///     #ifdef WLED_DEBUG_HOST
    ///         os |= 0x0100;
    ///         if (!netDebugEnabled) os &= ~0x0080;
    ///     #endif
    /// 0x80: debug enabled
    /// 0x40: disable alexa
    /// 0x20: Depreceated, used to be Blynk support, may be repurposed
    /// 0x10: usermod Chronixie
    /// 0x08: disable filesystem build tag
    /// 0x04: disable hue sync build tag
    /// 0x02: enable AdaLight build tag
    /// 0x01: disable OTA build tag
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub opt: Option<u16>,

    /// The producer/vendor of the light. Always WLED for standard installations.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub brand: Option<String>,

    /// The product name. Always FOSS for standard installations.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub product: Option<String>,

    /// The origin of the build.
    /// src if a release version is compiled from source,
    /// bin for an official release image,
    /// dev for a development build (regardless of src/bin origin)
    /// and exp for experimental versions.
    /// ogn if the image is flashed to hardware by the vendor.
    /// Removed as of v0.10
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub btype: Option<String>,

    /// The hexadecimal hardware MAC address of the light, lowercase and without colons.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub mac: Option<String>,

    /// The IP address of this instance. Empty string if not connected. (since 0.13.0)
    /// format: sprintf(s, "%d.%d.%d.%d", localIP[0], localIP[1], localIP[2], localIP[3]);
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub ip: Option<String>,
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
    /// will include virtual/nonexistent pixels in matrix
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub count: Option<u16>,

    /// 0 to 65000; Current LED power usage in milliamps as determined by the ABL.
    /// (0 if ABL is disabled.)<-maybe
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub pwr: Option<u16>,

    /// Returns the refresh rate of the LED strip.
    /// Useful for finding out whether a given setup is fast enough.
    /// Only updates on show() or is set to 0 fps if last show is more than 2 secs ago,
    /// so accuracy varies
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub fps: Option<u16>,

    /// 0 to 65000; Maximum power budget in milliamps for the ABL. 0 if ABL is disabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub maxpwr: Option<u16>,

    /// Maximum number of segments supported by this version.
    /// returns maximum number of supported segments (fixed value)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub maxseg: Option<u8>,

    /// Dimensions of matrix; not included in all builds
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub matrix: Option<MatrixDims>,

    /// Per-segment virtual light capabilities
    ///
    /// !!! this is a bitmap using the masks found in ```SegmentLightCapability```
    ///
    /// !!! Not ```LightCapability```
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub seglc: Option<Vec<u8>>,

    /// Logical AND of all active segment's virtual light capabilities
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub lc: Option<u8>,

    /// true if LEDs are 4-channel (RGB + White). (deprecated, use info.leds.lc)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub rgbw: Option<bool>,

    /// WLED WIKI SAYS BOOL??? true if a white channel slider should be displayed. (available since 0.10.0, deprecated, use info.leds.lc)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub wv: Option<u8>,

    /// WLED WIKI SAYS BOOL??? true if the light supports color temperature control (available since 0.13.0, deprecated, use info.leds.lc)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub cct: Option<u8>,

    /// LED strip pin(s). Always one element. Removed as of v0.13
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub pin: Option<Vec<i8>>,

    /// [i2c_sda pin, i2c_scl pin];
    /// only in debug builds
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub i2c: Option<[i8; 2]>,

    /// [spi_mosi pin, spi_sclk pin, spi_miso pin];
    /// only in debug builds
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub spi: Option<[i8; 3]>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Map {
    /// index of an avilable led map
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub id: Option<u8>,

    /// Led map name. only included in builds for hardware with sufficient processing power
    /// (currently just everything but the esp8266)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub n: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Wifi {
    /// Return the current bssid / mac associated with the network if configured
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub bssid: Option<String>,

    /// undocumented??? heres the WLED source as of WLED ~14.0:
    /// int qrssi = WiFi.RSSI(); <- thats an i8
    ///   wifi_info[F("rssi")] = qrssi; <- but thats a u32
    ///   wifi_info[F("signal")] = getSignalQuality(qrssi);
    /// I'm going to make RSSI an i64 to include both in case this gets fixed in the future.
    /// blame me for 4 bytes
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub rssi: Option<i64>,

    /// 0-100; Relative signal quality of the current connection.
    /// use this over RSSI unless for some reason you feel cooler doing the conversion in your head
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub signal: Option<u8>,

    /// 1 to 14; The current WiFi channel. WLED source uses i32 so i will too
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub channel: Option<i32>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Fs {
    /// Estimate of used filesystem space in kilobytes
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub u: Option<u32>,

    /// Total filesystem size in kilobytes
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub t: Option<u32>,

    /// Unix timestamp for the last modification to the presets.json file.
    /// Not accurate after boot or after using /edit
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub pmt: Option<u64>,
}

///these define matrix width & height (max. segment dimensions)
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MatrixDims {
    /// max width
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub w: Option<u16>,

    /// max height
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub h: Option<u16>,
}


///     NOT TO BE CONFUSED WITH LightCapability.
///     this is a bitmap of 3 basic capabilities
///
///     I know its confusing. Believe me.
#[allow(non_camel_case_types)]
#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug, Clone)]
#[repr(u8)]
pub enum SegmentLightCapability {
    SEG_CAPABILITY_RGB = 0x01,
    SEG_CAPABILITY_W = 0x02,
    SEG_CAPABILITY_CCT = 0x04,
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
