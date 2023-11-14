use serde;
use serde::{Serialize, Deserialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use crate::errors::WledJsonApiError;
use crate::structures::none_function;




#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Eth {
    /// type of ethernet
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub relay: Option<EthType>,

    /// pins used??? idk how this one works
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub pin: Option<Vec<u32>>,
}



/// Constants defined for Ethernet types
#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug, Clone)]
#[repr(u8)]
pub enum EthType {
    WledEthNone,
    WledEthWt32Eth01,
    WledEthEsp32Poe,
    WledEthWesp32,
    WledEthQuinled,
    WledEthTwilightlord,
    WledEthEsp32deux,
    WledEthEsp32ethkitve,
    WledEthQuinledOcta,
    WledEthAbcwledv43eth,
    WledEthSerg74,
    /// Reserved to keep some semblance of backwards compatibility when new WLED versions come out with more ethernet types
    RSVD1,
    /// Reserved to keep some semblance of backwards compatibility when new WLED versions come out with more ethernet types
    RSVD2,
    /// Reserved to keep some semblance of backwards compatibility when new WLED versions come out with more ethernet types
    RSVD3,
    /// Reserved to keep some semblance of backwards compatibility when new WLED versions come out with more ethernet types
    RSVD4,
    /// Reserved to keep some semblance of backwards compatibility when new WLED versions come out with more ethernet types
    RSVD5,
    /// Reserved to keep some semblance of backwards compatibility when new WLED versions come out with more ethernet types
    RSVD6,
    /// Reserved to keep some semblance of backwards compatibility when new WLED versions come out with more ethernet types
    RSVD7,
    /// Reserved to keep some semblance of backwards compatibility when new WLED versions come out with more ethernet types
    RSVD8,

}