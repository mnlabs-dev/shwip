use crate::logger;
use crate::models::ScanConfig;
use crate::scanners;
use crate::trash;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "shwip", about = "Intelligent Mac cleanup for developers")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Scan {
        #[arg(long, help = "Output as JSON")]
        json: bool,
    },
    Clean {
        #[arg(long, help = "Preview without deleting (default)")]
        dry_run: bool,
        #[arg(long, help = "Actually move items to trash")]
        confirm: bool,
        #[arg(long, help = "Include REVIEW items")]
        include_review: bool,
    },
    Report {
        #[arg(long, default_value = "md", help = "Output format: md or json")]
        format: String,
        #[arg(long, help = "Enrich report with LLM explanations (requires Ollama)")]
        explain: bool,
    },
    Logs {
        #[arg(long, help = "Clear all log files")]
        clear: bool,
        #[arg(long, default_value = "50", help = "Number of lines to show")]
        lines: usize,
    },
}

pub fn run_cli(cli: Cli) {
    let rt = tokio::runtime::Runtime::new().expect("Failed to create runtime");

    match cli.command {
        Commands::Scan { json } => rt.block_on(cmd_scan(json)),
        Commands::Clean {
            dry_run,
            confirm,
            include_review,
        } => rt.block_on(cmd_clean(dry_run, confirm, include_review)),
        Commands::Report { format, explain } => rt.block_on(cmd_report(&format, explain)),
        Commands::Logs { clear, lines } => cmd_logs(clear, lines),
    }
}

async fn cmd_scan(json: bool) {
    let config = ScanConfig::default();
    let home = dirs::home_dir().unwrap_or_else(|| std::path::PathBuf::from("/tmp"));
    let scanners = scanners::all_scanners();
    let mut results = Vec::new();

    for scanner in scanners {
        match scanner.scan(&home, &config) {
            Ok(r) => results.extend(r),
            Err(e) => eprintln!("scanner '{}' failed: {e}", scanner.name()),
        }
    }

    if json {
        println!(
            "{}",
            serde_json::to_string_pretty(&results).unwrap_or_default()
        );
    } else {
        if results.is_empty() {
            println!("Nothing to clean. Your Mac is tidy.");
            return;
        }
        println!("{:<12} {:<8} {:>10}  PATH", "CATEGORY", "LEVEL", "SIZE");
        println!("{}", "-".repeat(72));
        for r in &results {
            println!(
                "{:<12} {:<8} {:>10}  {}",
                truncate(&r.category, 12),
                format!("{:?}", r.confidence),
                format_size(r.size_bytes),
                truncate(&r.path, 40),
            );
        }
        let total: u64 = results.iter().map(|r| r.size_bytes).sum();
        println!(
            "\n{} items found, {} reclaimable",
            results.len(),
            format_size(total)
        );
    }
}

async fn cmd_clean(dry_run: bool, confirm: bool, include_review: bool) {
    let config = ScanConfig::default();
    let home = dirs::home_dir().unwrap_or_else(|| std::path::PathBuf::from("/tmp"));
    let scanners = scanners::all_scanners();
    let mut results = Vec::new();

    for scanner in scanners {
        if let Ok(r) = scanner.scan(&home, &config) {
            results.extend(r);
        }
    }

    let items: Vec<_> = results
        .iter()
        .filter(|r| {
            r.confidence == crate::models::Confidence::Safe
                || (include_review && r.confidence == crate::models::Confidence::Review)
        })
        .collect();

    if items.is_empty() {
        println!("Nothing to clean.");
        return;
    }

    let total: u64 = items.iter().map(|r| r.size_bytes).sum();

    if !confirm || dry_run {
        println!("DRY RUN (use --confirm to actually clean)\n");
        for item in &items {
            println!(
                "  {:?}  {:>10}  {}",
                item.confidence,
                format_size(item.size_bytes),
                item.path
            );
        }
        println!(
            "\n{} items, {} would be freed",
            items.len(),
            format_size(total)
        );
        return;
    }

    let mut cleaned = 0u64;
    let mut count = 0;
    for item in &items {
        let path = std::path::Path::new(&item.path);
        match trash::move_to_trash(path) {
            Ok(()) => {
                cleaned += item.size_bytes;
                count += 1;
            }
            Err(e) => eprintln!("failed to trash {}: {}", item.path, e),
        }
    }

    println!(
        "Cleaned {} items, {} freed",
        count,
        format_size(cleaned)
    );
}

async fn cmd_report(format: &str, explain: bool) {
    let config = ScanConfig::default();
    let home = dirs::home_dir().unwrap_or_else(|| std::path::PathBuf::from("/tmp"));
    let scanners = scanners::all_scanners();
    let mut results = Vec::new();

    for scanner in scanners {
        if let Ok(r) = scanner.scan(&home, &config) {
            results.extend(r);
        }
    }

    if format == "json" {
        println!(
            "{}",
            serde_json::to_string_pretty(&results).unwrap_or_default()
        );
        return;
    }

    let llm_client = if explain {
        let client = crate::llm::OllamaClient::default();
        if client.is_available().await {
            Some(client)
        } else {
            eprintln!("Ollama not available, using fallback explanations");
            None
        }
    } else {
        None
    };

    println!("# shwip scan report\n");
    let mut groups: std::collections::HashMap<String, Vec<&crate::models::ScanResult>> =
        std::collections::HashMap::new();
    for r in &results {
        groups.entry(r.category.clone()).or_default().push(r);
    }

    for (category, items) in &groups {
        let cat_total: u64 = items.iter().map(|r| r.size_bytes).sum();
        println!("## {} ({})\n", category, format_size(cat_total));
        for item in items {
            let explanation = if let Some(ref client) = llm_client {
                client.explain_item(item).await
            } else if explain {
                crate::llm::fallback_explanation(item)
            } else {
                item.reason.clone()
            };
            println!(
                "- **{:?}** {} -- {}",
                item.confidence,
                format_size(item.size_bytes),
                explanation
            );
        }
        println!();
    }

    let total: u64 = results.iter().map(|r| r.size_bytes).sum();
    println!(
        "---\n**Total**: {} items, {} reclaimable",
        results.len(),
        format_size(total)
    );
}

fn cmd_logs(clear: bool, lines: usize) {
    let dir = logger::log_dir();

    if clear {
        if dir.exists() {
            if let Ok(entries) = std::fs::read_dir(&dir) {
                for entry in entries.flatten() {
                    let _ = std::fs::remove_file(entry.path());
                }
            }
        }
        println!("Logs cleared.");
        return;
    }

    match logger::latest_log_path() {
        Some(path) => {
            let content = std::fs::read_to_string(&path).unwrap_or_default();
            let all_lines: Vec<&str> = content.lines().collect();
            let start = all_lines.len().saturating_sub(lines);
            for line in &all_lines[start..] {
                println!("{line}");
            }
        }
        None => println!("No log files found in {}", dir.display()),
    }
}

fn format_size(bytes: u64) -> String {
    if bytes >= 1_073_741_824 {
        format!("{:.1} GB", bytes as f64 / 1_073_741_824.0)
    } else if bytes >= 1_048_576 {
        format!("{:.1} MB", bytes as f64 / 1_048_576.0)
    } else if bytes >= 1_024 {
        format!("{} KB", bytes / 1_024)
    } else {
        format!("{bytes} B")
    }
}

fn truncate(s: &str, max: usize) -> String {
    if s.len() <= max {
        s.to_string()
    } else {
        format!("{}...", &s[..max - 3])
    }
}
