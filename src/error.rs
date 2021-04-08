// Copyright (c) 2021 ruarango developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! `ruarango` error

use serde::ser::{Serialize, SerializeStruct, Serializer};
use std::error::Error;
#[cfg(test)]
use std::num::ParseIntError;

#[derive(thiserror::Error, Debug)]
#[allow(variant_size_differences)]
pub(crate) enum RuarangoError {
    #[error("{}\nInvalid Body: {}", err, body)]
    InvalidBody { err: String, body: String },
    #[error("Unreachable: {}", msg)]
    Unreachable { msg: String },
    #[error("You have supplied an invalid connection url")]
    InvalidConnectionUrl,
    #[cfg(test)]
    #[error("Unable to parse the given value")]
    ParseInt(#[from] ParseIntError),
    #[cfg(test)]
    #[error("A test error has occurred: {}", val)]
    TestError { val: String },
    #[cfg(test)]
    #[error("You have requested an invalid mock")]
    InvalidMock,
}

impl Serialize for RuarangoError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("RuarangoError", 2)?;
        state.serialize_field("reason", &format!("{}", self))?;
        if let Some(source) = self.source() {
            state.serialize_field("source", &format!("{}", source))?;
        }
        state.end()
    }
}

#[cfg(test)]
impl From<&str> for RuarangoError {
    fn from(val: &str) -> Self {
        Self::TestError {
            val: val.to_string(),
        }
    }
}

#[cfg(test)]
impl From<String> for RuarangoError {
    fn from(val: String) -> Self {
        Self::TestError { val }
    }
}

#[cfg(test)]
mod test {
    use super::RuarangoError::{self, TestError};
    use anyhow::Result;

    #[test]
    fn serialize_with_source_works() -> Result<()> {
        match str::parse::<usize>("test") {
            Ok(_) => panic!("this shouldn't happen"),
            Err(e) => {
                let err: RuarangoError = e.into();
                let result = serde_json::to_string(&err)?;
                assert_eq!("{\"reason\":\"Unable to parse the given value\",\"source\":\"invalid digit found in string\"}", result);
            }
        }

        Ok(())
    }

    #[test]
    fn serialize_no_source_works() -> Result<()> {
        let err: RuarangoError = TestError {
            val: "test".to_string(),
        };
        let result = serde_json::to_string(&err)?;
        assert_eq!("{\"reason\":\"A test error has occurred: test\"}", result);
        Ok(())
    }

    #[test]
    fn from_str_works() -> Result<()> {
        let err: RuarangoError = "test".into();
        let result = serde_json::to_string(&err)?;
        assert_eq!("{\"reason\":\"A test error has occurred: test\"}", result);
        Ok(())
    }

    #[test]
    fn from_string_works() -> Result<()> {
        let err: RuarangoError = String::from("test").into();
        let result = serde_json::to_string(&err)?;
        assert_eq!("{\"reason\":\"A test error has occurred: test\"}", result);
        Ok(())
    }
}
