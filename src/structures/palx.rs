use serde;
use serde::{Serialize, Deserialize};
use crate::errors::WledJsonApiError;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PalX {
    pub m: i64,
    pub p: P,
}

impl TryFrom<&str> for PalX{
    type Error = WledJsonApiError;
    fn try_from(str_in: &str) -> Result<PalX, WledJsonApiError> {
        serde_json::from_str(str_in).map_err(|e| {WledJsonApiError::SerdeError(e)})
    }
}


impl TryInto<String> for &PalX{
    type Error = WledJsonApiError;
    fn try_into(self) -> Result<String, WledJsonApiError> {
        serde_json::to_string(self).map_err(|e| {WledJsonApiError::SerdeError(e)})
    }
}



#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct P {
    #[serde(rename = "0")]
    pub n0: Vec<Vec<i64>>,
    #[serde(rename = "1")]
    pub n1: Vec<String>,
    #[serde(rename = "2")]
    pub n2: Vec<String>,
    #[serde(rename = "3")]
    pub n3: Vec<String>,
    #[serde(rename = "4")]
    pub n4: Vec<String>,
    #[serde(rename = "5")]
    pub n5: Vec<String>,
    #[serde(rename = "6")]
    pub n6: Vec<Vec<i64>>,
    #[serde(rename = "7")]
    pub n7: Vec<Vec<i64>>,
}
