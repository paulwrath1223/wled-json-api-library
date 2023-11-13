use serde;
use serde::{Serialize, Deserialize};
use crate::errors::WledJsonApiError;


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Live {
    pub leds: Vec<String>,
    pub n: i64,
}

impl TryFrom<&str> for Live{
    type Error = WledJsonApiError;
    fn try_from(str_in: &str) -> Result<Live, WledJsonApiError> {
        serde_json::from_str(str_in).map_err(|e| {WledJsonApiError::SerdeError(e)})
    }
}

