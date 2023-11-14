
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Nw {
    /// honestly no idea why this is a vector, WLED source only uses one element
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub ins: Option<Vec<In>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct In {
    /// SSID of the network to connect to
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub ssid: Option<String>,

    /// Length of the wifi password
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub pskl: Option<usize>,

    /// static IP of ESP
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub ip: Option<[u8; 4]>,

    /// gateway (router) IP
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub gw: Option<[u8; 4]>,

    /// most common subnet in home networks is 255:255:255:0
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub sn: Option<[u8; 4]>,
}