use anyhow::Result;
use serde::Deserialize;

pub(crate) struct AllocatorStats {
    pub allocated: usize,
    pub resident: usize,
}

#[derive(Deserialize)]
struct ProcessStats {
    commit_current: usize,
    rss_current: usize,
}

#[derive(Deserialize)]
struct MiMallocStats {
    process: ProcessStats,
}

pub(crate) fn get_allocator_stats() -> Result<AllocatorStats> {
    let raw = mimalloc::MiMalloc::stats_json()
        .map_err(|e| anyhow::anyhow!("mimalloc stats error: {e}"))?;
    let parsed: MiMallocStats = serde_json::from_slice(raw.to_bytes())?;
    Ok(AllocatorStats {
        allocated: parsed.process.commit_current,
        resident: parsed.process.rss_current,
    })
}
