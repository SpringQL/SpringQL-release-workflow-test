// This file is part of https://github.com/SpringQL/SpringQL which is licensed under MIT OR Apache-2.0. See file LICENSE-MIT or LICENSE-APACHE for full license details.

use anyhow::anyhow;
use std::sync::{Arc, Mutex, MutexGuard};

use crate::{
    api::{
        error::{Result, SpringError},
        low_level_rs::spring_config::SpringConfig,
    },
    stream_engine::StreamEngine,
};

#[derive(Clone, Debug)]
pub(super) struct EngineMutex(Arc<Mutex<StreamEngine>>);

impl EngineMutex {
    pub(super) fn new(config: &SpringConfig) -> Self {
        let engine = StreamEngine::new(config);
        Self(Arc::new(Mutex::new(engine)))
    }

    /// # Failure
    ///
    /// - [SpringError::ThreadPoisoned](crate::error::SpringError::ThreadPoisoned)
    pub(super) fn get(&self) -> Result<MutexGuard<'_, StreamEngine>> {
        self.0
            .lock()
            .map_err(|e| {
                anyhow!(
                    "another thread sharing the same stream-engine got panic: {:?}",
                    e
                )
            })
            .map_err(SpringError::SpringQlCoreIo)
    }
}
