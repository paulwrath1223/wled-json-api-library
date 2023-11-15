use serde;
use serde::{Serialize, Deserialize};
use crate::errors::WledJsonApiError;
use crate::structures::cfg::cfg_ap::Ap;
use crate::structures::cfg::cfg_def::Def;
use crate::structures::cfg::cfg_dmx::Dmx;
use crate::structures::cfg::cfg_eth::Eth;
use crate::structures::cfg::cfg_hw::Hw;
use crate::structures::cfg::cfg_id::Id;
use crate::structures::cfg::cfg_if2::If2;
use crate::structures::cfg::cfg_light::Light;
use crate::structures::cfg::cfg_nw::Nw;
use crate::structures::cfg::cfg_ol::Ol;
use crate::structures::cfg::cfg_ota::Ota;
use crate::structures::cfg::cfg_remote::Remote;
use crate::structures::cfg::cfg_timers::Timers;
use crate::structures::cfg::cfg_wifi::Wifi;
use crate::structures::none_function;




pub mod cfg_id;
pub mod cfg_nw;
pub mod cfg_ap;
pub mod cfg_eth;
pub mod cfg_wifi;
pub mod cfg_hw;
pub mod cfg_light;
pub mod cfg_def;
pub mod cfg_if2;
pub mod cfg_remote;
pub mod cfg_ol;
pub mod cfg_timers;
pub mod cfg_ota;
pub mod cfg_dmx;


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cfg {
    /// Version of WLED ("1.0.2", for example is [1, 0, 2]) but WLED source only uses 2 indices :/
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

    /// client mode network info
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub nw: Option<Nw>,

    /// ethernet info, not included in builds with use Ethernet build flag
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub eth: Option<Eth>,

    /// Access point info
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub ap: Option<Ap>,

    /// literally just "sleep" whatever the fuck it meansa
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub wifi: Option<Wifi>,

    /// hardware info
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub hw: Option<Hw>,

    /// light info
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub light: Option<Light>,

    /// defaults
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub def: Option<Def>,

    /// Iterface info
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    #[serde(rename = "if")]
    pub if_field: Option<If2>,

    /// Remote info
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub remote: Option<Remote>,

    /// Analog clock stuff
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub ol: Option<Ol>,

    /// timer settings
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub timers: Option<Timers>,

    /// Over the air update settings
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub ota: Option<Ota>,

    /// Dmx setting. build-dependant
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub dmx: Option<Dmx>,

    /// usermod settings. this depends on the mod, so I wont even touch this.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub um: Option<serde_json::Value>,
}


impl TryFrom<&str> for Cfg{
    type Error = WledJsonApiError;
    fn try_from(str_in: &str) -> Result<Cfg, WledJsonApiError> {
        serde_json::from_str(str_in).map_err(|e| {WledJsonApiError::SerdeError(e)})
    }
}

impl TryInto<String> for &Cfg{
    type Error = WledJsonApiError;
    fn try_into(self) -> Result<String, WledJsonApiError> {
        serde_json::to_string(self).map_err(|e| {WledJsonApiError::SerdeError(e)})
    }
}