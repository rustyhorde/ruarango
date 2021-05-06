// Copyright (c) 2021 ruarango developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Graph Input Structs

mod create;
mod delete;
mod edge;
mod edge_def;
mod read;
mod vertex;

pub use create::{
    Config as CreateConfig, ConfigBuilder as CreateConfigBuilder,
    ConfigBuilderError as CreateConfigBuilderError, GraphMeta, GraphMetaBuilder,
    GraphMetaBuilderError,
};
pub use delete::{
    Config as DeleteConfig, ConfigBuilder as DeleteConfigBuilder,
    ConfigBuilderError as DeleteConfigBuilderError,
};
pub use edge::create::{
    Config as EdgeCreateConfig, ConfigBuilder as EdgeCreateConfigBuilder,
    ConfigBuilderError as EdgeCreateConfigBuilderError, FromTo, FromToBuilder, FromToBuilderError,
};
pub use edge::delete::{
    Config as EdgeDeleteConfig, ConfigBuilder as EdgeDeleteConfigBuilder,
    ConfigBuilderError as EdgeDeleteConfigBuilderError,
};
pub use edge::read::{
    Config as EdgeReadConfig, ConfigBuilder as EdgeReadConfigBuilder,
    ConfigBuilderError as EdgeReadConfigBuilderError,
};
pub use edge::replace::{
    Config as EdgeReplaceConfig, ConfigBuilder as EdgeReplaceConfigBuilder,
    ConfigBuilderError as EdgeReplaceConfigBuilderError,
};
pub use edge::update::{
    Config as EdgeUpdateConfig, ConfigBuilder as EdgeUpdateConfigBuilder,
    ConfigBuilderError as EdgeUpdateConfigBuilderError,
};
pub use edge_def::create::{
    Config as CreateEdgeDefConfig, ConfigBuilder as CreateEdgeDefConfigBuilder,
    ConfigBuilderError as CreateEdgeDefConfigBuilderError,
};
pub use edge_def::delete::{
    Config as DeleteEdgeDefConfig, ConfigBuilder as DeleteEdgeDefConfigBuilder,
    ConfigBuilderError as DeleteEdgeDefConfigBuilderError,
};
pub use edge_def::read::{
    Config as ReadEdgeDefsConfig, ConfigBuilder as ReadEdgeDefsConfigBuilder,
    ConfigBuilderError as ReadEdgeDefsConfigBuilderError,
};
pub use edge_def::replace::{
    Config as ReplaceEdgeDefConfig, ConfigBuilder as ReplaceEdgeDefConfigBuilder,
    ConfigBuilderError as ReplaceEdgeDefConfigBuilderError,
};
pub use read::{
    Config as ReadConfig, ConfigBuilder as ReadConfigBuilder,
    ConfigBuilderError as ReadConfigBuilderError,
};
pub use vertex::read::{
    Config as ReadVertexCollsConfig, ConfigBuilder as ReadVertexCollsConfigBuilder,
    ConfigBuilderError as ReadVertexCollsConfigBuilderError,
};
