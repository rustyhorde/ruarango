use anyhow::{anyhow, Result};
use getset::{Getters, MutGetters};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Getters, Serialize)]
#[getset(get = "pub(crate)")]
pub(crate) struct OutputDoc {
    #[serde(rename = "_key")]
    key: String,
    #[serde(rename = "_id")]
    id: String,
    #[serde(rename = "_rev")]
    rev: String,
    test: String,
}

#[derive(Clone, Debug, Deserialize, Getters, MutGetters, Serialize)]
#[getset(get = "pub(crate)", get_mut = "pub(crate)")]
pub(crate) struct TestDoc {
    #[serde(rename = "_key", skip_serializing_if = "Option::is_none")]
    key: Option<String>,
    test: String,
}

impl Default for TestDoc {
    fn default() -> Self {
        Self {
            key: None,
            test: "test".to_string(),
        }
    }
}

pub(crate) fn unwrap_doc<'a>(doc_opt: &'a Option<TestDoc>) -> Result<&TestDoc> {
    Ok(doc_opt.as_ref().ok_or_else(|| anyhow!("bad"))?)
}

#[derive(Clone, Debug, Deserialize, Getters, Serialize)]
pub(crate) struct SearchDoc {
    #[serde(rename = "_key")]
    key: String,
}

impl SearchDoc {
    pub(crate) fn new<S>(key: S) -> Self
    where
        S: Into<String>,
    {
        SearchDoc { key: key.into() }
    }
}
