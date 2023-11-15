use serde;
use serde::{Serialize, Deserialize};
use crate::structures::none_function;


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Remote {
    /// is remote enabled
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    #[serde(rename = "remote_enabled")]
    pub remote_enabled: Option<bool>,

    /// linked remote? idk
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    #[serde(rename = "linked_remote")]
    pub linked_remote: Option<String>,
}
