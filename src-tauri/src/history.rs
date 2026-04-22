use serde::{Deserialize, Serialize};

const MAX_HISTORY: usize = 20;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanHistoryEntry {
    pub timestamp: String,
    pub results_count: usize,
    pub total_bytes: u64,
    pub categories: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ScanHistory {
    pub entries: Vec<ScanHistoryEntry>,
}

impl ScanHistory {
    pub fn push(&mut self, entry: ScanHistoryEntry) {
        self.entries.push(entry);
        if self.entries.len() > MAX_HISTORY {
            self.entries.remove(0);
        }
    }
}

pub fn record_scan(results: &[crate::models::ScanResult]) -> ScanHistoryEntry {
    let mut cats: Vec<String> = results.iter().map(|r| r.category.clone()).collect();
    cats.sort();
    cats.dedup();

    ScanHistoryEntry {
        timestamp: chrono_now(),
        results_count: results.len(),
        total_bytes: results.iter().map(|r| r.size_bytes).sum(),
        categories: cats,
    }
}

fn chrono_now() -> String {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    format!("{now}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_fifo() {
        let mut history = ScanHistory::default();
        for i in 0..25 {
            history.push(ScanHistoryEntry {
                timestamp: format!("{i}"),
                results_count: i,
                total_bytes: 0,
                categories: vec![],
            });
        }
        assert_eq!(history.entries.len(), MAX_HISTORY);
        assert_eq!(history.entries[0].timestamp, "5");
    }

    #[test]
    fn test_record_scan_deduplicates_categories() {
        use crate::models::{Confidence, ScanResult};
        let results = vec![
            ScanResult {
                category: "npm".into(),
                path: "/a".into(),
                size_bytes: 100,
                confidence: Confidence::Safe,
                reason: "test".into(),
            },
            ScanResult {
                category: "npm".into(),
                path: "/b".into(),
                size_bytes: 200,
                confidence: Confidence::Safe,
                reason: "test".into(),
            },
        ];
        let entry = record_scan(&results);
        assert_eq!(entry.categories.len(), 1);
        assert_eq!(entry.total_bytes, 300);
    }
}
