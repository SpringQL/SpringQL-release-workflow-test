use crate::{
    error::Result,
    model::option::Options,
    stream_engine::{
        autonomous_executor::data::foreign_row::foreign_source_row::ForeignSourceRow,
        pipeline::server_model::server_type::ServerType,
    },
};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

pub(in crate::stream_engine) mod net;

#[derive(Serialize, Deserialize, new)]
pub(in crate::stream_engine) struct SourceServerSeed {
    pub(in crate::stream_engine) server_type: ServerType,
    pub(in crate::stream_engine) options: Options,
}

pub(in crate::stream_engine) trait SourceServerStandby {
    type Act: SourceServerActive + Sized;

    fn new(options: &Options) -> Result<Self>
    where
        Self: Sized;

    /// Blocks until the server is ready to provide ForeignSourceRow.
    fn start(self) -> Result<Self::Act>;
}

/// Active: ready to provide ForeignSourceRow.
pub(in crate::stream_engine) trait SourceServerActive:
    Debug + Sync + Send + 'static
{
    /// Returns currently available foreign row.
    ///
    /// # Failure
    ///
    /// - [SpringError::ForeignSourceTimeout](crate::error::SpringError::ForeignSourceTimeout) when:
    ///   - Remote source does not provide row within timeout.
    /// - [SpringError::ForeignIo](crate::error::SpringError::ForeignIo) when:
    ///   - Failed to parse response from remote source.
    ///   - Unknown foreign error.
    fn next_row(&mut self) -> Result<ForeignSourceRow>;

    fn seed(&self) -> SourceServerSeed;
}
