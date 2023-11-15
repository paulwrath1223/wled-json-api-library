use serde;
use serde::{Serialize, Deserialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use crate::structures::cfg::cfg_hw::cfg_hw_led::Led;
use crate::structures::none_function;


pub mod cfg_hw_led;


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

    /// Info about connected buttons
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub btn: Option<Btn>,

    /// info about IR receiver
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub ir: Option<Ir>,

    /// info about relay
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
pub struct If {
    /// [i2c_sda pin, i2c_scl pin]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    #[serde(rename = "i2c-pin")]
    pub i2c_pin: Option<[i8; 2]>,

    /// [spi_mosi pin, spi_sclk pin, spi_miso pin]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    #[serde(rename = "spi-pin")]
    pub spi_pin: Option<[i8; 3]>,
}


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct In3 {

    /// Button type
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    #[serde(rename = "type")]
    pub type_field: Option<ButtonType>,

    /// Button pin
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub pin: Option<Vec<i8>>,

    /// Button macros
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub macros: Option<ButtonMacros>,
}



/// Various types
#[allow(non_camel_case_types)]
#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug, Clone)]
#[repr(u8)]
pub enum ButtonType {
    /// None
    BTN_TYPE_NONE,
    /// Reserved
    BTN_TYPE_RESERVED,
    /// Push button that is considered "pushed" when at logic low
    BTN_TYPE_PUSH,
    /// Push button that is considered "pushed" when at logic high
    BTN_TYPE_PUSH_ACT_HIGH,
    /// A switch with no defined on or off
    BTN_TYPE_SWITCH,
    /// PIR sensor
    BTN_TYPE_PIR_SENSOR,
    /// Touch sensor (presumably using the built in touch sensor of the ESP32)
    BTN_TYPE_TOUCH,
    /// Not really a button, but there you go
    BTN_TYPE_ANALOG,
    /// BTN_TYPE_ANALOG, but inverted
    BTN_TYPE_ANALOG_INVERTED,
    /// Reserved to keep some semblance of backwards compatibility when new WLED versions come out with more AP behaviours
    RSVD1,
    /// Reserved to keep some semblance of backwards compatibility when new WLED versions come out with more AP behaviours
    RSVD2,
    /// Reserved to keep some semblance of backwards compatibility when new WLED versions come out with more AP behaviours
    RSVD3,
    /// Reserved to keep some semblance of backwards compatibility when new WLED versions come out with more AP behaviours
    RSVD4,
    /// Reserved to keep some semblance of backwards compatibility when new WLED versions come out with more AP behaviours
    RSVD5,
    /// Reserved to keep some semblance of backwards compatibility when new WLED versions come out with more AP behaviours
    RSVD6,

}



