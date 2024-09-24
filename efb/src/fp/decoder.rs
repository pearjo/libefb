// SPDX-License-Identifier: Apache-2.0
// Copyright 2024 Joe Pearson
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

// TODO make pretty and fix structure
use std::str::FromStr;

use crate::error::Error;
use crate::{Speed, Wind};

impl FromStr for Wind {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() == 6 && &s[3..4] == "@" {
            let direction = s[0..3].parse::<i16>();
            let speed = s[4..6].parse::<f32>();

            match (direction, speed) {
                (Ok(direction), Ok(speed)) => Ok(Wind {
                    direction: direction.into(),
                    speed: Speed::Knots(speed),
                }),
                _ => Err(Error::UnexpectedString),
            }
        } else {
            Err(Error::UnexpectedString)
        }
    }
}

impl FromStr for Speed {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() == 5 && &s[3..5] == "KT" {
            if let Ok(speed) = s[0..3].parse::<f32>() {
                Ok(Speed::Knots(speed))
            } else {
                Err(Error::UnexpectedString)
            }
        } else {
            Err(Error::UnexpectedString)
        }
    }
}
