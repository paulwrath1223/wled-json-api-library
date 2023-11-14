
/// Hardware info
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Hw {

    /// Led info
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub led: Option<Led>,

    /// List of color order maps
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub com: Option<Vec<ColorOrderMap>>,

    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub btn: Option<Btn>,

    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub ir: Option<Ir>,

    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub relay: Option<Relay>,

    /// baud rate/100
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub baud: Option<u16>,

    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    #[serde(rename = "if")]
    pub if_field: Option<If>,
}


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Led {
    /// get total legth; no longer read as of WLED 14.0, but provided for compatibility on downgrade
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub total: Option<u16>,

    /// max milliamps
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub maxpwr: Option<u16>,

    /// milliamps per LED
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub ledma: Option<u8>,

    /// CCT color correction of RGB color
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub cct: Option<bool>,

    /// CCT is calculated from RGB instead of using seg.cct
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub cr: Option<bool>,

    /// CCT Blending
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub cb: Option<u8>,

    /// Target FPS
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub fps: Option<u8>,

    /// global auto white mode override; 255 = Global auto white mode override disabled. Per-bus setting is used
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub rgbwm: Option<u8>,

    /// use Global Led Buffer
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub ld: Option<bool>,

    /// if using 2d WLED
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub matrix: Option<Matrix>,

    /// WLED busses
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub ins: Option<Vec<In2>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Btn {

    /// just information about max number of buttons (not actually used)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub max: Option<u8>,

    /// idfk
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub pull: Option<bool>,

    /// balls, testicles
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub ins: Option<Vec<In3>>,

    /// Touch threshold
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub tt: Option<u8>,

    /// button Publish Mqtt
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub mqtt: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Relay {
    /// pin for the relay
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub pin: Option<i8>,

    /// negate output
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub rev: Option<bool>,
}


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ColorOrderMap {

    /// Probably the start index?
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub start: Option<u16>,

    /// stop index? probably?, could not find in latest source, but its in the WLED 14.0 binary
    #[serde(skip_serializing)]
    #[serde(default = "none_function")]
    pub stop: Option<u16>,

    /// length ig?
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub len: Option<u16>,

    /// Whatever the fuck 'order' is
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub order: Option<u8>,
}


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ButtonMacros {
    /// preset to apply when button is pressed once, or default behaviour if 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    #[serde(rename = "0")]
    pub macro_button: Option<u8>,

    /// preset to apply when button is pressed for a long time, or default behaviour if 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    #[serde(rename = "1")]
    pub macro_long_press: Option<u8>,

    /// preset to apply when button is pressed twice, or default behaviour if 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    #[serde(rename = "2")]
    pub macro_double_press: Option<u8>,

}


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ir {

    /// Pin for this sensor. presumably fucking explodes if negative
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub pin: Option<i8>,

    /// IR type 0 is disabled, go scour the undocumented WLED source code for the rest
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    #[serde(rename = "type")]
    pub type_field: Option<u8>,

    /// apply IR to all selected segments; who knows what happens when false
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub sel: Option<bool>,
}


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct In2 {

    /// start ig?
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub start: Option<u16>,

    /// length ig?
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub len: Option<u16>,

    /// list of pins for some fucking reason
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub pin: Option<Vec<u8>>,

    /// Color order
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub order: Option<u8>,

    /// is reversed?
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub rev: Option<bool>,

    /// skipped LEDs??
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub skip: Option<u8>,

    /// Light type. see "LightCapability" enum documentation
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    #[serde(rename = "type")]
    pub type_field: Option<LightCapability>,

    /// is Off Refresh Required
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    #[serde(rename = "ref")]
    pub ref_field: Option<bool>,

    /// get Auto White Mode
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub rgbwm: Option<u8>,

    /// frequency?? IG?
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub freq: Option<u16>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Matrix {
    /// fuck if I know (strip.panels in WLED source)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub mpc: Option<u8>,

    /// panels
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub panels: Option<Vec<Panel>>,
}