use core::fmt;
use std::{
    collections::HashMap,
    sync::{atomic::AtomicI64, Arc},
};

use anyhow::Result;

#[derive(Debug, Clone)]
pub struct AMapMetrics {
    data: Arc<HashMap<&'static str, AtomicI64>>,
}

impl AMapMetrics {
    pub fn new(metrics_names: &[&'static str]) -> Self {
        let map = metrics_names
            .iter()
            .map(|&name| (name, AtomicI64::new(0)))
            .collect();
        AMapMetrics {
            data: Arc::new(map),
        }
    }

    pub fn inc(&self, key: impl AsRef<str>) -> Result<()> {
        let key = key.as_ref();
        if let Some(counter) = self.data.get(key) {
            counter.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        }

        Ok(())
    }
}

impl fmt::Display for AMapMetrics {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (key, value) in self.data.iter() {
            writeln!(f, "{}: {}", key, value.load(std::sync::atomic::Ordering::Relaxed))?;
        }

        Ok(())
    }
}