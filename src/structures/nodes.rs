use serde;
use serde::{Serialize, Deserialize};
use crate::errors::WledJsonApiError;



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
    pub name: String,
    #[serde(rename = "type")]
    pub type_field: i64,
    pub ip: String,
    pub age: i64,
    pub vid: i64,
}
