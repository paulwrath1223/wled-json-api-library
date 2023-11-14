#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Id {
    ///mDNS address (*.local, replaced by wledXXXXXX if default is used)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub mdns: Option<String>,

    /// Server Description; Name of module - use default
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub name: Option<String>,

    /// Alexa invocation name; speech control name of device. Choose something voice-to-text can understand
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub inv: Option<String>,

    /// Simplified UI;
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub sui: Option<bool>,
}