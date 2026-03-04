use opentelemetry::metrics::{Gauge, Meter};

#[derive(Clone)]
pub struct CommonMetrics {
    mem_allocated_recorder: Gauge<u64>,
    mem_resident_recorder: Gauge<u64>,
}

impl CommonMetrics {
    pub fn new(meter: &Meter) -> Self {
        let mem_resident_recorder = meter.u64_gauge("svix.mem_resident").build();
        let mem_allocated_recorder = meter.u64_gauge("svix.mem_allocated").build();

        Self {
            mem_allocated_recorder,
            mem_resident_recorder,
        }
    }

    pub fn record_mem_allocated(&self, value: u64) {
        self.mem_allocated_recorder.record(value, &[]);
    }

    pub fn record_mem_resident(&self, value: u64) {
        self.mem_resident_recorder.record(value, &[]);
    }
}
