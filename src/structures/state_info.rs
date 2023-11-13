use serde;
use serde::{Serialize, Deserialize};
use crate::errors::WledJsonApiError;
use crate::structures::state::State;
use crate::structures::info::Info;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StateInfo {
    pub state: State,
    pub info: Info,
}

impl TryFrom<&str> for StateInfo{
    type Error = WledJsonApiError;
    fn try_from(str_in: &str) -> Result<StateInfo, WledJsonApiError> {
        serde_json::from_str(str_in).map_err(|e| {WledJsonApiError::SerdeError(e)})
    }
}