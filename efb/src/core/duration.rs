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

use std::fmt::{Display, Formatter, Result};

#[derive(Copy, Clone)]
pub struct Duration {
    pub hours: u8,
    pub minutes: u8,
    pub seconds: u8,
}

impl Duration {
    pub fn from_seconds(s: u32) -> Self {
        Self {
            hours: (s / 3600 % 24) as u8,
            minutes: (s / 60 % 60) as u8,
            seconds: (s % 60) as u8,
        }
    }

    pub fn round(self) -> Self {
        Self {
            hours: self.hours,
            minutes: if self.seconds >= 30 {
                self.minutes + 1
            } else {
                self.minutes
            },
            seconds: 0,
        }
    }
}

impl Display for Duration {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        if self.seconds > 0 {
            write!(
                f,
                "{:02}:{:02}:{:02}",
                self.hours, self.minutes, self.seconds
            )
        } else {
            write!(f, "{:02}:{:02}", self.hours, self.minutes)
        }
    }
}
