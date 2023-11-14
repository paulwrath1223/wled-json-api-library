use serde;
use serde::{Serialize, Deserialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use crate::errors::WledJsonApiError;
use crate::structures::none_function;



pub mod cfg_id;
pub mod cfg_nw;
pub mod cfg_ap;
pub mod cfg_eth;
pub mod cfg_wifi;
pub mod cfg_hw;


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cfg {
    /// Version of WLED ("1.0.2", for example is [1, 0, 2])
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub rev: Option<Vec<u32>>,

    /// Version ID; version code in format yymmddb (b = daily build) (macro called "VERSION" in wled source)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub vid: Option<u64>,

    /// identifying information
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub id: Option<Id>,

    pub nw: Nw,
    pub eth: Eth,
    pub ap: Ap,
    pub wifi: Wifi,
    pub hw: Hw,
    pub light: Light,
    pub def: Def,
    #[serde(rename = "if")]
    pub if_field: If2,
    pub remote: Remote,
    pub ol: Ol,
    pub timers: Timers,
    pub ota: Ota,
    pub um: Um,
}


impl TryFrom<&str> for Cfg{
    type Error = WledJsonApiError;
    fn try_from(str_in: &str) -> Result<Cfg, WledJsonApiError> {
        serde_json::from_str(str_in).map_err(|e| {WledJsonApiError::SerdeError(e)})
    }
}
