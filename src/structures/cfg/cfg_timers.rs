use serde;
use serde::{Serialize, Deserialize};
use crate::structures::none_function;
use serde_aux_ext::field_attributes::deserialize_option_bool_from_anything;


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Timers {

    /// Countdown
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub cntdwn: Option<Cntdwn>,

    /// Countdown mode; Clock will count down towards date
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub ins: Option<Vec<Ins>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cntdwn {

    /// The time that the countdown is counting down towards.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub goal: Option<Goal>,

    /// Macro (presumably this is executed on when countdown reaches goal
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    #[serde(rename = "macro")]
    pub macro_field: Option<u8>,
}

/// You can figure this one out, its a date time
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Goal {
    #[serde(rename = "0")]
    pub year: u8,
    #[serde(rename = "1")]
    pub month: u8,
    #[serde(rename = "2")]
    pub day: u8,
    #[serde(rename = "3")]
    pub hour: u8,
    #[serde(rename = "4")]
    pub minute: u8,
    #[serde(rename = "5")]
    pub second: u8,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ins {
    /// enabled?
    #[serde(deserialize_with = "deserialize_option_bool_from_anything")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub en: Option<bool>,

    /// hour, I don't think this one needs much explaining
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub hour: Option<u8>,

    /// minute, I don't think this one needs much explaining
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub min: Option<i8>,

    /// Macro to execute when triggered
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    #[serde(rename = "macro")]
    pub macro_field: Option<u8>,

    /// Day and month that this timer turns on every year
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub start: Option<MonthDay>,

    /// Day and month that this timer turns off every year
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub end: Option<MonthDay>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MonthDay {

    /// the month (actually 4 bits, but cope)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub mon: Option<u8>,

    /// Day and month that this timer turns off every year
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub day: Option<u8>,
}
