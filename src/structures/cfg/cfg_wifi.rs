use serde;
use serde::{Serialize, Deserialize};
use crate::structures::none_function;




#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Wifi {

    /// IDFK you're on your own
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub sleep: Option<bool>,
}