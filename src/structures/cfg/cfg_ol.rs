use serde;
use serde::{Serialize, Deserialize};
use crate::structures::none_function;


/// anolog clock stuff?
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ol {
    /// 0: no overlay 1: analog clock 2: was single-digit clock 3: was cronixie
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub clock: Option<u8>,

    /// Countdown mode; Clock will count down towards date
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub cntdwn: Option<bool>,

    /// Overlay min; boundaries of overlay mode
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub min: Option<u8>,

    /// Overlay max; boundaries of overlay mode
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub max: Option<u8>,

    /// The pixel in your strip where "midnight" would be
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub o12pix: Option<u8>,

    /// analog Clock 5 Minute Marks; Light pixels at every 5-minute position
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub o5m: Option<bool>,

    /// Display seconds as trail of LEDs instead of a single pixel
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub osec: Option<bool>,
}
