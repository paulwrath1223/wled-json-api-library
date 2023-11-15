use serde;
use serde::{Serialize, Deserialize};
use crate::errors::WledJsonApiError;
use crate::structures::none_function;



/// Not Done, broken, dont use
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Live {
    /// Not Done, broken, dont use
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub leds: Option<Vec<String>>,

    /// Not Done, broken, dont use
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub n: Option<i64>,
}

impl TryFrom<&str> for Live{
    type Error = WledJsonApiError;
    fn try_from(str_in: &str) -> Result<Live, WledJsonApiError> {
        serde_json::from_str(str_in).map_err(|e| {WledJsonApiError::SerdeError(e)})
    }
}

