//! Allocator stats are only available when we're using jemalloc, and jemalloc doesn't work on windows.
//!
//! 2 impls for the helper functions are therefore provided. One set that does nothing (for windows)
//! and another that works in the non-windows world.
//!
//! Care should be taken to keep the signatures aligned between these two so the callsites can be
//! used consistently regardless of whether jemalloc is in use or not.

#[cfg(all(not(target_env = "msvc"), feature = "jemalloc"))]
pub use supported::*;
#[cfg(any(target_env = "msvc", not(feature = "jemalloc")))]
pub use unsupported::*;

#[cfg(all(not(target_env = "msvc"), feature = "jemalloc"))]
mod supported {
    use std::sync::Arc;

    use tikv_jemalloc_ctl::{epoch, stats};

    pub struct AllocatorStatMibs {
        epoch: tikv_jemalloc_ctl::epoch_mib,
        allocated: stats::allocated_mib,
        resident: stats::resident_mib,
    }

    pub fn get_allocator_stats(
        bust_cache: bool,
        mibs: Arc<AllocatorStatMibs>,
    ) -> anyhow::Result<Option<(usize, usize)>> {
        if bust_cache {
            // Stats are cached internally and advancing the epoch is a way to invalidate those caches.
            mibs.epoch.advance()?;
        }
        let allocated = mibs.allocated.read()?;
        let resident = mibs.resident.read()?;
        Ok(Some((allocated, resident)))
    }

    pub fn get_allocator_stat_mibs() -> anyhow::Result<Arc<AllocatorStatMibs>> {
        let e = epoch::mib()?;
        let allocated = stats::allocated::mib()?;
        let resident = stats::resident::mib()?;

        Ok(Arc::new(AllocatorStatMibs {
            epoch: e,
            allocated,
            resident,
        }))
    }
}

#[cfg(any(target_env = "msvc", not(feature = "jemalloc")))]
mod unsupported {
    use std::sync::Arc;

    use anyhow::anyhow;

    pub struct AllocatorStatMibs;

    pub fn get_allocator_stats(
        _bust_cache: bool,
        _mibs: Arc<AllocatorStatMibs>,
    ) -> anyhow::Result<Option<(usize, usize)>> {
        Ok(None)
    }

    pub fn get_allocator_stat_mibs() -> anyhow::Result<Arc<AllocatorStatMibs>> {
        Err(anyhow!("metric collection is not supported"))
    }
}
