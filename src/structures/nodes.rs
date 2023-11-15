use serde;
use serde::{Serialize, Deserialize};
use crate::errors::WledJsonApiError;
use crate::structures::none_function;


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Nodes {
    pub nodes: Vec<Node>,
}


impl TryFrom<&str> for Nodes{
    type Error = WledJsonApiError;
    fn try_from(str_in: &str) -> Result<Nodes, WledJsonApiError> {
        serde_json::from_str(str_in).map_err(|e| {WledJsonApiError::SerdeError(e)})
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Node {
    /// Not a trick question, really just the name of the corresponding node
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub name: Option<String>,

    /// 'a waste of space as we only have 5 types'
    ///     - the WLED source code
    ///
    /// #define NODE_TYPE_ID_UNDEFINED        0
    /// #define NODE_TYPE_ID_ESP8266         82 // should be 1
    /// #define NODE_TYPE_ID_ESP32           32 // should be 2
    /// #define NODE_TYPE_ID_ESP32S2         33 // etc
    /// #define NODE_TYPE_ID_ESP32S3         34
    /// #define NODE_TYPE_ID_ESP32C3         35
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    #[serde(rename = "type")]
    pub type_field: Option<u8>,

    /// Not a trick question, really just IP address
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub ip: Option<String>,

    /// We have our top 4 engineers on the case, we'll know aht this is in about 3 years
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub age: Option<u8>,

    /// We have our top 4 engineers on the case, we'll know aht this is in about 3 years
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub vid: Option<u32>,
}
