// czml-rs/examples/basic.rs
//
// Copyright © 2018 Quentin Baradat
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate serde_json;
extern crate chrono;
extern crate czml;

use czml::*;
use chrono::prelude::*;

fn main() {
    let mut czml = Czml::new();
    czml.push(
        Packet {
            id: Some(1.to_string()),
            name: Some(String::from("test")),
            clock: Some(Clock {
                interval: Some(TimeInterval {
                    start: Utc.ymd(2014, 7, 8).and_hms(9, 10, 11),
                    stop: Utc.ymd(2015, 6, 7).and_hms(3, 8, 4)
                }),
                current_time: Some(Utc.ymd(2014, 7, 8).and_hms(9, 10, 11)),
                multiplier: Some(1.0),
                range: Some(ClockRange::Unbounded),
                step: Some(ClockStep::SystemClockMultiplier)
            }),
            position: Some(Position {
                interpolatable_property: Some(InterpolatableProperty {
                    epoch: Some(String::from("test")),
                    ..Default::default()
                }),
                // TODO(Quentin) : create a macro pos![ a, b, c ] 
                // or pos![ a1, b1, c1, d1, a2, b2, c2, d2, ...]
                // cartesian: Some(pos![ 10.0, 11.0, 13.0 ]),
                cartesian: Some(Sequence::Array(vec! [
                    TimeTaggedValue { time: 0.0 , value: Cartesian3Value { x: 10.0, y: 11.0, z: 13.0}},
                    TimeTaggedValue { time: 1.0 , value: Cartesian3Value { x: 20.0, y: 21.0, z: 23.0}},
                ])),
                ..Default::default()
            }),
            ..Default::default()
        }
    );

    println!("{}", serde_json::to_string(&czml).unwrap());
}