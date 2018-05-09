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

#[derive(Serialize)]
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
}

#[derive(Serialize)]
pub enum CzmlString {
    StringValue { string: String },
    ReferenceValue { reference: String }
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
#[serde(tag = "type")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ClockRange {
    Unbounded,
    Clamped,
    LoopStop
}

#[derive(Serialize)]
#[serde(tag = "type")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ClockStep {
    TickDependent,
    SystemClockMultiplier,
    SystemClock
}