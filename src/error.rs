// Copyright (c) 2021 ruarango developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! `ruarango` error

use crate::model::doc::output::DocErr;
use serde::ser::{Serialize, SerializeStruct, Serializer};
use std::error::Error;
#[cfg(test)]
use std::num::ParseIntError;

/// When bad things happen
#[derive(thiserror::Error, Clone, Debug, Eq, PartialEq)]
#[allow(variant_size_differences)]
pub enum RuarangoErr {
    ///
    #[error("{}\nInvalid Body: {}", err, body)]
    InvalidBody {
        ///
        err: String,
        ///
        body: String,
    },
    ///
    #[error("Unreachable: {}", msg)]
    Unreachable {
        ///
        msg: String,
    },
    ///
    #[error("You have supplied an invalid connection url")]
    InvalidConnectionUrl,
    ///
    #[error("Invalid document response: {}", status)]
    InvalidDocResponse {
        ///
        status: u16,
    },
    ///
    #[error("The document you requested was not found")]
    DocumentNotFound,
    ///
    #[error("The document you requested has not been modified")]
    NotModified,
    ///
    #[error("A precondition has failed: '{}'", doc_err(err))]
    PreconditionFailed {
        ///
        err: Option<DocErr>,
    },
    ///
    #[error("A precondition has failed: '{}'", doc_err(err))]
    Conflict {
        ///
        err: Option<DocErr>,
    },
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

impl Serialize for RuarangoErr {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("RuarangoErr", 2)?;
        state.serialize_field("reason", &format!("{}", self))?;
        if let Some(source) = self.source() {
            state.serialize_field("source", &format!("{}", source))?;
        }
        state.end()
    }
}

fn doc_err(err: &Option<DocErr>) -> String {
    err.as_ref().map_or_else(
        || "No matching document found".to_string(),
        ToString::to_string,
    )
}

#[cfg(test)]
impl From<&str> for RuarangoErr {
    fn from(val: &str) -> Self {
        Self::TestError {
            val: val.to_string(),
        }
    }
}

#[cfg(test)]
impl From<String> for RuarangoErr {
    fn from(val: String) -> Self {
        Self::TestError { val }
    }
}

#[cfg(test)]
mod test {
    use super::RuarangoErr::{self, TestError};
    use anyhow::Result;

    #[test]
    fn serialize_with_source_works() -> Result<()> {
        match str::parse::<usize>("test") {
            Ok(_) => panic!("this shouldn't happen"),
            Err(e) => {
                let err: RuarangoErr = e.into();
                let result = serde_json::to_string(&err)?;
                assert_eq!("{\"reason\":\"Unable to parse the given value\",\"source\":\"invalid digit found in string\"}", result);
            }
        }

        Ok(())
    }

    #[test]
    fn serialize_no_source_works() -> Result<()> {
        let err: RuarangoErr = TestError {
            val: "test".to_string(),
        };
        let result = serde_json::to_string(&err)?;
        assert_eq!("{\"reason\":\"A test error has occurred: test\"}", result);
        Ok(())
    }

    #[test]
    fn from_str_works() -> Result<()> {
        let err: RuarangoErr = "test".into();
        let result = serde_json::to_string(&err)?;
        assert_eq!("{\"reason\":\"A test error has occurred: test\"}", result);
        Ok(())
    }

    #[test]
    fn from_string_works() -> Result<()> {
        let err: RuarangoErr = String::from("test").into();
        let result = serde_json::to_string(&err)?;
        assert_eq!("{\"reason\":\"A test error has occurred: test\"}", result);
        Ok(())
    }
}
