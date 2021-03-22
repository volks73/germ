// Copyright (C) 2021  Christopher R. Field
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

pub const MILLISECONDS_IN_A_SECOND: f64 = 1000.0;
pub const MILLISECONDS_UNITS: &str = "ms";
pub const SECONDS_UNITS: &str = "secs";
pub const SHELL_VAR_NAME: &str = "SHELL";
pub const TERM_VAR_NAME: &str = "TERM";

pub mod asciicast;
pub mod sequence;
pub mod termsheets;

pub trait ApplySpeed {
    type Output;

    fn speed(self, speed: f64) -> Self::Output;
}

impl ApplySpeed for f64 {
    type Output = Self;

    fn speed(self, speed: f64) -> Self::Output {
        self / speed
    }
}

pub trait SecondsConversions {
    type Output;

    fn into_seconds(self) -> Self::Output;

    fn into_milliseconds(self) -> Self::Output;
}

impl SecondsConversions for f64 {
    type Output = Self;

    fn into_seconds(self) -> Self::Output {
        self / MILLISECONDS_IN_A_SECOND
    }

    fn into_milliseconds(self) -> Self::Output {
        self * MILLISECONDS_IN_A_SECOND
    }
}
