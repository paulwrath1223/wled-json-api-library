use serde;
use serde::{Serialize, Deserialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use crate::structures::none_function;





#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Light {
    /// % of brightness to set (to limit power, if you set it to 50 and set bri to 255, actual brightness will be 127)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    #[serde(rename = "scale-bri")]
    pub scale_bri: Option<u8>,

    /// paletteBlend: 0 - wrap when moving, 1 - always wrap, 2 - never wrap, 3 - none (undefined)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    #[serde(rename = "pal-mode")]
    pub pal_mode: Option<u8>,

    /// If true, a segment per bus will be created on boot and LED settings save.
    /// If false, only one segment spanning the total LEDs is created,
    /// but not on LED settings save if there is more than one segment currently
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub aseg: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub gc: Option<Gc>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub tr: Option<Tr>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub nl: Option<Nl>,
}

/// gamma correction shit, tired of documenting speghetti
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Gc {
    /// see struct documentation
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub bri: Option<f64>,

    /// see struct documentation
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub col: Option<f64>,

    /// see struct documentation
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub val: Option<f64>,
}

/// transition stuff, tired of documenting speghetti
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tr {
    /// see struct documentation
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub mode: Option<bool>,

    /// default transition time / 100
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub dur: Option<u16>,

    /// see struct documentation
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub pal: Option<u8>,

    /// amount of time [s] between random palette changes (min: 1s, max: 255s)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub rpc: Option<u8>,
}

/// night light stuff, tired of documenting speghetti
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Nl {

    /// see struct documentation
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub mode: Option<NightLightMode>,

    /// see struct documentation
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub dur: Option<u8>,

    /// brightness after nightlight is over
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub tbri: Option<u8>,

    /// after nightlight delay over
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    #[serde(rename = "macro")]
    pub macro_field: Option<u8>,
}




/// Modes for night light
#[allow(non_camel_case_types)]
#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug, Clone)]
#[repr(u8)]
pub enum NightLightMode {
    /// After nightlight time elapsed, set to target brightness
    NL_MODE_SET,
    /// Fade to target brightness gradually
    NL_MODE_FADE,
    /// Fade to target brightness and secondary color gradually
    NL_MODE_COLORFADE,
    /// Sunrise/sunset. Target brightness is set immediately, then Sunrise effect is started. Max 60 min.
    NL_MODE_SUN,
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
