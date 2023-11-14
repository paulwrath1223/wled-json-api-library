use serde;
use serde::{Serialize, Deserialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use crate::errors::WledJsonApiError;
use crate::structures::none_function;


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ol {
    pub clock: i64,
    pub cntdwn: bool,
    pub min: i64,
    pub max: i64,
    pub o12pix: i64,
    pub o5m: bool,
    pub osec: bool,
}
