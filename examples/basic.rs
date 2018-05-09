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
            delete: None,
            name: Some(String::from("test")),
            parent: None,
            description: None,
            clock: Some(Clock {
                interval: Some(TimeInterval {
                    start: Utc.ymd(2014, 7, 8).and_hms(9, 10, 11),
                    stop: Utc.ymd(2015, 6, 7).and_hms(3, 8, 4)
                }),
                current_time: Some(Utc.ymd(2014, 7, 8).and_hms(9, 10, 11)),
                multiplier: Some(1.0),
                range: Some(ClockRange::Unbounded),
                step: Some(ClockStep::SystemClockMultiplier)
            })
        }
    );

    println!("{}", serde_json::to_string(&czml).unwrap());
}