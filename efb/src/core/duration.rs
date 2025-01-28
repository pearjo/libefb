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
use std::ops::Add;

/// A duration measured in hours, minutes and seconds.
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Duration {
    pub hours: u8,
    pub minutes: u8,
    pub seconds: u8,
}

impl Duration {
    /// Returns self with the seconds rounded to the nearest minute.
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

impl Add for Duration {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let seconds: u32 = self.into();
        let rhs: u32 = rhs.into();
        Duration::from(seconds + rhs)
    }
}

impl From<u32> for Duration {
    /// Converts the seconds into a duration.
    ///
    /// ```
    /// use efb::Duration;
    /// let duration = Duration::from(3600);
    /// assert_eq!(duration, Duration { hours: 1, minutes: 0, seconds: 0 });
    /// ```
    fn from(seconds: u32) -> Self {
        Self {
            hours: (seconds / 3600 % 24) as u8,
            minutes: (seconds / 60 % 60) as u8,
            seconds: (seconds % 60) as u8,
        }
    }
}

impl From<Duration> for u32 {
    /// Converts the duration into seconds.
    ///
    /// ```
    /// use efb::Duration;
    /// let duration = Duration { hours: 1, minutes: 0, seconds: 0 };
    /// assert_eq!(3600u32, duration.into());
    /// ```
    fn from(value: Duration) -> Self {
        value.hours as u32 * 3600 + value.minutes as u32 * 60 + value.seconds as u32
    }
}

impl Display for Duration {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        // TODO: Allow formatting to round or show full duration!
        let rounded = self.round();
        write!(f, "{:02}:{:02}", rounded.hours, rounded.minutes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_seconds() {
        let duration = Duration::from(3661);
        assert_eq!(duration.hours, 1);
        assert_eq!(duration.minutes, 1);
        assert_eq!(duration.seconds, 1);
    }

    #[test]
    fn to_seconds() {
        let duration = Duration {
            hours: 1,
            minutes: 1,
            seconds: 1,
        };
        assert_eq!(3661u32, duration.into());
    }

    #[test]
    fn add() {
        let sum = Duration::from(3561) + Duration::from(100);
        assert_eq!(sum.hours, 1);
        assert_eq!(sum.minutes, 1);
        assert_eq!(sum.seconds, 1);
    }

    #[test]
    fn round() {
        let duration = Duration::from(29);
        assert_eq!(duration.round().minutes, 0);

        let duration = Duration::from(30);
        assert_eq!(duration.round().minutes, 1);
    }
}
