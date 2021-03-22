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

use serde::{Deserialize, Serialize};

use crate::sequence::Sequence;

#[derive(Debug, Deserialize, Serialize)]
pub struct Command {
    input: String,
    output: Vec<String>,
}

impl From<crate::sequence::Command> for Command {
    fn from(c: crate::sequence::Command) -> Self {
        Self {
            input: c.input().to_owned(),
            output: c.into_outputs(),
        }
    }
}

impl<'a> From<&'a crate::sequence::Command> for Command {
    fn from(c: &'a crate::sequence::Command) -> Self {
        Self {
            input: c.input().to_owned(),
            output: c.outputs().clone(),
        }
    }
}

impl From<Command> for crate::sequence::Command {
    fn from(mut c: Command) -> Self {
        let mut cmd = Self::from(c.input);
        cmd.append(&mut c.output);
        cmd
    }
}

impl From<Sequence> for Vec<Command> {
    fn from(s: Sequence) -> Self {
        s.into_iter().map(Command::from).collect()
    }
}

impl<'a> From<&'a Sequence> for Vec<Command> {
    fn from(s: &'a Sequence) -> Self {
        s.iter().map(Command::from).collect()
    }
}

impl From<Vec<Command>> for Sequence {
    fn from(t: Vec<Command>) -> Self {
        Sequence::from(
            t.into_iter()
                .map(crate::sequence::Command::from)
                .collect::<Vec<crate::sequence::Command>>(),
        )
    }
}
