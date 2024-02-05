use opentelemetry::metrics::{Meter, ObservableGauge};

fn init_metric<T, E: std::fmt::Display>(result: Result<T, E>) -> Option<T> {
    match result {
        Ok(t) => Some(t),
        Err(e) => {
            tracing::error!("Failed to initialize metric: {}", e);
            None
        }
    }
}

#[derive(Clone)]
pub struct CommonMetrics {
    mem_allocated_recorder: Option<ObservableGauge<u64>>,
    mem_resident_recorder: Option<ObservableGauge<u64>>,
}

impl CommonMetrics {
    pub fn new(meter: &Meter) -> Self {
        let mem_resident_recorder =
            init_metric(meter.u64_observable_gauge("svix.mem_resident").try_init());
        let mem_allocated_recorder =
            init_metric(meter.u64_observable_gauge("svix.mem_allocated").try_init());

        Self {
            mem_allocated_recorder,
            mem_resident_recorder,
        }
    }

    pub fn record_mem_allocated(&self, value: u64) {
        if let Some(ref recorder) = self.mem_allocated_recorder {
            recorder.observe(value, &[]);
        }
    }

    pub fn record_mem_resident(&self, value: u64) {
        if let Some(ref recorder) = self.mem_resident_recorder {
            recorder.observe(value, &[]);
        }
    }
}
