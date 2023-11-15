use serde;
use serde::{Serialize, Deserialize};
use crate::structures::none_function;

/// defaults
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Def {
    /// save preset to load after power-up
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub ps: Option<u8>,

    /// turn on LEDs at power-up
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub on: Option<bool>,

    /// default brightness
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub bri: Option<u8>,
}