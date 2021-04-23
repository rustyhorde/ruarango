// Copyright (c) 2021 ruarango developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Cursor operations trait

use async_trait::async_trait;

use crate::{model::cursor::input::CreateConfig, ArangoResult};

/// Cursor Operations
#[async_trait]
pub trait Cursor {
    /// Create a cursor
    async fn create(&self, config: CreateConfig) -> ArangoResult<()>;
}
