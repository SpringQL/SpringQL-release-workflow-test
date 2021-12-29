// Copyright (c) 2021 TOYOTA MOTOR CORPORATION. Licensed under MIT OR Apache-2.0.

use super::sink::SinkWriterInstance;
use crate::error::Result;
use crate::pipeline::{option::Options, server_model::server_type::ServerType};
use crate::stream_engine::autonomous_executor::server_instance::sink::in_memory_queue::InMemoryQueueSinkServerInstance;
use crate::stream_engine::autonomous_executor::server_instance::sink::net::NetSinkServerInstance;

pub(in crate::stream_engine) struct ServerInstanceFactory;

impl ServerInstanceFactory {
    pub(in crate::stream_engine) fn sink(
        server_type: &ServerType,
        options: &Options,
    ) -> Result<Box<dyn SinkWriterInstance>> {
        match server_type {
            ServerType::SinkNet => {
                let server = NetSinkServerInstance::start(options)?;
                Ok(Box::new(server) as Box<dyn SinkWriterInstance>)
            }
            ServerType::SinkInMemoryQueue => {
                let server = InMemoryQueueSinkServerInstance::start(options)?;
                Ok(Box::new(server) as Box<dyn SinkWriterInstance>)
            }

            ServerType::SourceNet => unreachable!(),
        }
    }
}
