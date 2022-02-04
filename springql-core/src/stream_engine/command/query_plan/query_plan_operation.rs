// Copyright (c) 2021 TOYOTA MOTOR CORPORATION. Licensed under MIT OR Apache-2.0.

use std::time::Duration;

use crate::{
    expression::ValueExprPh1,
    pipeline::{field::field_pointer::FieldPointer, name::StreamName},
};

#[derive(Clone, PartialEq, Debug)]
pub(crate) enum QueryPlanOperation {
    Collect {
        stream: StreamName,
    },
    EvalValueExpr {
        expressions: Vec<ValueExprPh1>,
    },
    Projection {
        field_pointers: Vec<FieldPointer>,
    },
    TimeBasedSlidingWindow {
        /// cannot use chrono::Duration here: <https://github.com/chronotope/chrono/issues/117>
        lower_bound: Duration,
    },
}
