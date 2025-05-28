// Copyright (c) The Diem Core Contributors
// Copyright (c) The Move Contributors
// SPDX-License-Identifier: Apache-2.0

use criterion::{Criterion, measurement::Measurement};

// Use conditional compilation to handle platform-specific code
#[cfg(unix)]
use criterion_cpu_time::PosixTime;

pub fn cpu_time_measurement() -> impl Criterion {
    #[cfg(unix)]
    {
        // On Unix systems, use CPU time measurement
        Criterion::default().with_measurement(PosixTime::UserAndSystemTime)
    }
    
    #[cfg(not(unix))]
    {
        // On non-Unix systems (like Windows), fall back to wall time
        Criterion::default()
    }
}

pub fn wall_time_measurement() -> Criterion {
    Criterion::default()
}