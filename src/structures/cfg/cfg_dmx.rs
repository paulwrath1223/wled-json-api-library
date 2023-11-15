use serde;
use serde::{Serialize, Deserialize};
use crate::structures::none_function;


/// only included in some WLED builds
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Dmx {
    /// number of channels per fixture
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub chan: Option<u8>,

    /// assigns the different channels to different functions. See wled21_dmx.ino for more information.
    /// gap between the fixtures. makes addressing easier because you don't have to memorize odd numbers when climbing up onto a rig.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub gap: Option<u16>,

    /// start address of the first fixture
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub start: Option<u16>,

    /// LED from which DMX fixtures start
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    #[serde(rename = "start-led")]
    pub start_led: Option<u16>,

    /// fixture map
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub fixmap: Option<[u8; 16]>,

    /// output this E1.31 (sACN) / ArtNet universe via MAX485 (0 = disabled)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub e131proxy: Option<u16>,
}