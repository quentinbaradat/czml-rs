// czml-rs/src/lib.rs
//
// Copyright © 2018 Quentin Baradat
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate chrono;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

use chrono::prelude::*;
use serde::ser::{Serialize, Serializer, SerializeSeq};
use std::collections::HashMap;

pub struct Czml {
    pub packets: Vec<Packet>
}

impl Czml {
    pub fn new() -> Self {
        Czml { packets: vec![] }
    }

    pub fn push(&mut self, packet: Packet) {
        self.packets.push(packet);
    }
}

impl Serialize for Czml {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.packets.len()))?;
        for e in &self.packets {
            seq.serialize_element(&e)?;
        }
        seq.end()
    }
}

#[derive(Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Packet {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<CzmlString>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clock: Option<Clock>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability: Option<Vec<TimeInterval>>,
    // TODO : Remove String value and impl Properties
    // See https://github.com/AnalyticalGraphicsInc/czml-writer/wiki/CustomProperty
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position : Option<Position>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orientation: Option<Orientation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub view_from: Option<ViewFrom>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billboard: Option<Billboard>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub polyline: Option<Polyline>
}

#[derive(Serialize)]
pub enum CzmlString {
    StringValue { string: String },
    ReferenceValue { reference: String },
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Clock {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<TimeInterval>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_time: Option<Time>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiplier: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range: Option<ClockRange>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub step: Option<ClockStep>,
}

impl Default for Clock {
    fn default() -> Clock {
        Clock {
            interval: None,
            current_time: None,
            multiplier: Some(1.0),
            range: Some(ClockRange::LoopStop),
            step: Some(ClockStep::SystemClockMultiplier)
        }
    }
}

pub type Time = DateTime<Utc>;

pub struct TimeInterval {
    pub start: Time,
    pub stop: Time,
}

impl Serialize for TimeInterval {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut interval = String::new();
        interval += &format!("{:?}", self.start);
        interval += "/";
        interval += &format!("{:?}", self.stop);
        serializer.serialize_str(&interval)
    }
}

#[derive(Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ClockRange {
    Unbounded,
    Clamped,
    LoopStop,
}

#[derive(Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ClockStep {
    TickDependent,
    SystemClockMultiplier,
    SystemClock,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Position {
    #[serde(flatten)]
    pub interpolatable_property: Option<InterpolatableProperty>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_frame: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cartesian: Option<Sequence<Cartesian3Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cartographic_radians: Option<Sequence<CartographicValue>>,
}

impl Default for Position {
    fn default() -> Position {
        Position {
            reference_frame: Some(String::from("FIXED")),
            interpolatable_property: Some(InterpolatableProperty::default()),
            cartesian: None,
            cartographic_radians: None
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InterpolatableProperty {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub epoch: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interpolation_algorithm: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interpolation_degree: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forward_extrapolation_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forward_extrapolation_duration: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backward_extrapolation_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backward_extrapolation_duration: Option<f64>,
}

impl Default for InterpolatableProperty {
    fn default() -> InterpolatableProperty {
        InterpolatableProperty {
            epoch: None,
            interpolation_algorithm: Some(String::from("LINEAR")),
            interpolation_degree: Some(1.0),
            forward_extrapolation_type: Some(String::from("NONE")),
            forward_extrapolation_duration: Some(1.0),
            backward_extrapolation_type: Some(String::from("NONE")),
            backward_extrapolation_duration: Some(1.0)
        }
    }
}

// TODO(Quentin) : More abstraction on TimeTaggedValue to have the possibility
// to add a generic tag.
pub enum Sequence<T> {
    Single(T),
    Array(Vec<TimeTaggedValue<T>>),
}

impl<T> Serialize for Sequence<T>
where 
    T: Sequenceable
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Sequence::Single(value) => {
                let mut seq = serializer.serialize_seq(None)?;
                value.seq(&mut seq)?;
                seq.end()
            },
            Sequence::Array(values) => {
                let mut seq = serializer.serialize_seq(None)?;
                for v in values {
                    seq.serialize_element(&v.time)?;
                    v.value.seq(&mut seq)?;
                }
                seq.end()
            }
        }
    }
}

// TODO(Quentin): Need an other trick to fit values of the position in one array.
pub trait Sequenceable {
    fn seq<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: SerializeSeq;
}

pub struct TimeTaggedValue<T> {
    pub time: f64,
    pub value: T,
}

#[derive(Serialize)]
pub struct Cartesian3Value {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Sequenceable for Cartesian3Value {
    fn seq<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: SerializeSeq
    {
        serializer.serialize_element(&self.x)?;
        serializer.serialize_element(&self.y)?;
        serializer.serialize_element(&self.z)?;
        Ok(())
    }
}

#[derive(Serialize)]
pub struct CartographicValue {
    pub longitude: f64,
    pub latitude: f64,
    pub height: f64,
}

impl Sequenceable for CartographicValue {
    fn seq<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: SerializeSeq
    {
        serializer.serialize_element(&self.longitude)?;
        serializer.serialize_element(&self.latitude)?;
        serializer.serialize_element(&self.height)?;
        Ok(())
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Orientation {
    #[serde(flatten)]
    pub interpolatable_property: Option<InterpolatableProperty>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_quaternion: Option<Sequence<UnitQuaternionValue>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub velocity_reference: Option<String>,
}

#[derive(Serialize)]
pub struct UnitQuaternionValue {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl Sequenceable for UnitQuaternionValue {
    fn seq<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: SerializeSeq
    {
        serializer.serialize_element(&self.x)?;
        serializer.serialize_element(&self.y)?;
        serializer.serialize_element(&self.z)?;
        serializer.serialize_element(&self.w)?;
        Ok(())
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ViewFrom {
    // TODO(Quentin) : Sequence doesnt fit here. This is
    // Sequence::Single only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cartesian: Option<Sequence<Cartesian3Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Billboard {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<Vec<Uri>>, // TODO(Quentin) : Need to rework this Uri or Vec<Uri>
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pixel_offset: Option<PixelOffset>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eye_offset: Option<EyeOffset>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub horizontal_origin: Option<HorizontalOrigin>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vertical_origin: Option<VerticalOrigin>,
}

#[derive(Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Uri {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<TimeInterval>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>, // TODO(Quentin) : UriValue ?
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PixelOffset {
    #[serde(flatten)]
    pub interpolatable_property: Option<InterpolatableProperty>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cartesian2: Option<Sequence<Cartesian2Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
}

pub struct Cartesian2Value {
    pub x: f64,
    pub y: f64,
}

impl Sequenceable for Cartesian2Value {
    fn seq<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: SerializeSeq
    {
        serializer.serialize_element(&self.x)?;
        serializer.serialize_element(&self.y)?;
        Ok(())
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EyeOffset {
    #[serde(flatten)]
    pub interpolatable_property: Option<InterpolatableProperty>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cartesian: Option<Sequence<Cartesian3Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HorizontalOrigin {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub horizontal_origin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VerticalOrigin {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vertical_origin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
}

// TODO(Quentin)
#[derive(Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Polyline {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub positions: Option<PositionList>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub granularity: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]   
    pub material: Option<PolylineMaterial>,
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub follow_surface: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub shadows: Option<ShadowMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub depth_fail_material: Option<PolylineMaterial>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distance_display_condition: Option<DistanceDisplayCondition>
}

// TODO(Quentin) : fields are missing
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PositionList {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_frame: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub references: Option<Vec<String>>,
}

// TODO(Quentin) : fields are missing
#[derive(Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PolylineMaterial {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub solid_color: Option<SolidColorMaterial>,
}

#[derive(Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SolidColorMaterial {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<Color>,
}

#[derive(Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ShadowMode {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shadow_mode: Option<ShadowModeValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ShadowModeValue {
    Disabled,
    Enabled,
    CastOnly,
    ReceiveOnly
}

// TODO(Quentin) : fields are missing & rgba to impl
#[derive(Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Color {
    #[serde(flatten)]
    pub interpolatable_property: Option<InterpolatableProperty>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rgba: Option<Vec<u8>>,
}

// TODO(Quentin) : fields are missing
#[derive(Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DistanceDisplayCondition {
    #[serde(flatten)]
    pub interpolatable_property: Option<InterpolatableProperty>,
}

pub mod helper {

}