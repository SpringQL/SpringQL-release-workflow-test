// This file is part of https://github.com/SpringQL/SpringQL which is licensed under MIT OR Apache-2.0. See file LICENSE-MIT or LICENSE-APACHE for full license details.

use std::sync::Arc;

use crate::stream_engine::autonomous_executor::{
    event_queue::{BlockingEventTag, EventTag, NonBlockingEventQueue, NonBlockingEventTag},
    memory_state_machine::MemoryStateTransition,
    performance_metrics::{
        MetricsUpdateByTaskExecutionOrPurge, PerformanceMetrics, PerformanceMetricsSummary,
    },
    pipeline_derivatives::PipelineDerivatives,
    task_executor::{
        scheduler::SourceScheduler,
        task_worker_thread_handler::{
            TaskWorkerLoopState, TaskWorkerThreadArg, TaskWorkerThreadHandler,
        },
    },
    worker::{WorkerSetupCoordinator, WorkerThread},
};

/// Runs a worker thread.
#[derive(Debug)]
pub struct SourceWorkerThread;

impl WorkerThread for SourceWorkerThread {
    const THREAD_NAME: &'static str = "SourceWorker";

    type ThreadArg = TaskWorkerThreadArg;

    type LoopState = TaskWorkerLoopState<SourceScheduler>;

    fn setup_ready(worker_setup_coordinator: Arc<WorkerSetupCoordinator>) {
        worker_setup_coordinator.ready_source_worker()
    }

    fn event_subscription() -> Vec<EventTag> {
        vec![
            EventTag::Blocking(BlockingEventTag::UpdatePipeline),
            EventTag::NonBlocking(NonBlockingEventTag::ReplacePerformanceMetrics),
        ]
    }

    fn main_loop_cycle(
        current_state: Self::LoopState,
        thread_arg: &Self::ThreadArg,
        event_queue: &NonBlockingEventQueue,
    ) -> Self::LoopState {
        TaskWorkerThreadHandler::main_loop_cycle::<SourceScheduler>(
            current_state,
            thread_arg,
            event_queue,
        )
    }

    fn ev_update_pipeline(
        current_state: Self::LoopState,
        pipeline_derivatives: Arc<PipelineDerivatives>,
        thread_arg: &Self::ThreadArg,
        _event_queue: Arc<NonBlockingEventQueue>,
    ) -> Self::LoopState {
        log::debug!(
            "[SourceWorker#{}] got UpdatePipeline event",
            thread_arg.worker_id
        );

        let mut state = current_state;
        state.pipeline_derivatives = Some(pipeline_derivatives);
        state
    }

    fn ev_replace_performance_metrics(
        current_state: Self::LoopState,
        metrics: Arc<PerformanceMetrics>,
        thread_arg: &Self::ThreadArg,
        _event_queue: Arc<NonBlockingEventQueue>,
    ) -> Self::LoopState {
        log::debug!(
            "[SourceWorker#{}] got ReplacePerformanceMetrics event",
            thread_arg.worker_id
        );

        let mut state = current_state;
        state.metrics = Some(metrics);
        state
    }

    fn ev_transit_memory_state(
        _current_state: Self::LoopState,
        _memory_state_transition: Arc<MemoryStateTransition>,
        _thread_arg: &Self::ThreadArg,
        _event_queue: Arc<NonBlockingEventQueue>,
    ) -> Self::LoopState {
        unreachable!();
    }

    fn ev_incremental_update_metrics(
        _current_state: Self::LoopState,
        _metrics: Arc<MetricsUpdateByTaskExecutionOrPurge>,
        _thread_arg: &Self::ThreadArg,
        _event_queue: Arc<NonBlockingEventQueue>,
    ) -> Self::LoopState {
        unreachable!()
    }

    fn ev_report_metrics_summary(
        _current_state: Self::LoopState,
        _metrics_summary: Arc<PerformanceMetricsSummary>,
        _thread_arg: &Self::ThreadArg,
        _event_queue: Arc<NonBlockingEventQueue>,
    ) -> Self::LoopState {
        unreachable!()
    }
}
