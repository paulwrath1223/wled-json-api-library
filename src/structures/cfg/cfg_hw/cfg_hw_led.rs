use serde;
use serde::{Serialize, Deserialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use crate::structures::none_function;


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



///     Light capability byte 0bRCCCTTTT
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