// CMapMetrics data structure
// functionality: inc/dec/snapshot

use anyhow::Result;

use core::fmt;
use std::sync::Arc;

use dashmap::DashMap;

#[derive(Debug, Clone)]
pub struct CMapMetrics {
    data: Arc<DashMap<String, i64>>,
}

impl CMapMetrics {
    pub fn new() -> Self {
        Self {
            data: Arc::new(DashMap::new()),
        }
    }

    pub fn inc(&self, key: impl Into<String>) -> Result<()> {
        let mut counter = self.data.entry(key.into()).or_insert(0);
        *counter += 1;

        Ok(())
    }
}

impl fmt::Display for CMapMetrics {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for entry in self.data.iter() {
            writeln!(f, "{}: {}", entry.key(), entry.value())?;
        }

        Ok(())
    }
}
