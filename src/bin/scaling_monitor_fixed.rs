//! Real-time Scaling Monitor
//! 
//! A dedicated monitoring tool that displays live scaling metrics
//! in a dashboard-like format to visualize auto-scaling behavior.

use std::time::{Duration, Instant};
use std::io::{stdout, Write};
use crossterm::{
    execute,
    terminal::{Clear, ClearType},
    cursor::{MoveTo, Hide, Show},
    style::{Color, Print, ResetColor, SetForegroundColor},
    Result as CrosstermResult,
};
use tokio::time::sleep;
use gridtokenx_blockchain::{ScalingCoordinator, ScalingConfig};

#[derive(Debug, Clone)]
struct MetricsSnapshot {
    timestamp: Instant,
    active_shards: usize,
    total_tps: f64,
    average_latency_ms: f64,
    memory_usage_mb: f64,
    cpu_usage_percent: f64,
    storage_ops_per_sec: f64,
}

#[derive(Debug, Clone)]
struct ScalingEvent {
    timestamp: Instant,
    from_shards: usize,
    to_shards: usize,
    trigger: String,
}

struct ScalingMonitor {
    coordinator: ScalingCoordinator,
    start_time: Instant,
    metrics_history: Vec<MetricsSnapshot>,
    scaling_events: Vec<ScalingEvent>,
}

impl ScalingMonitor {
    /// Create a new scaling monitor
    pub async fn new() -> anyhow::Result<Self> {
        let scaling_config = ScalingConfig::default();

        let coordinator = ScalingCoordinator::new(scaling_config).await?;
        
        Ok(Self {
            coordinator,
            start_time: Instant::now(),
            metrics_history: Vec::new(),
            scaling_events: Vec::new(),
        })
    }

    /// Start monitoring with real-time dashboard
    pub async fn start_monitoring(&mut self, duration_seconds: Option<u64>) -> anyhow::Result<()> {
        // Setup terminal
        execute!(stdout(), Hide)?;
        
        let end_time = duration_seconds.map(|d| Instant::now() + Duration::from_secs(d));
        let mut last_shard_count = 1usize;
        
        println!("🚀 GridTokenX Real-time Scaling Monitor");
        println!("⏰ Started at: {}", chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC"));
        if let Some(duration) = duration_seconds {
            println!("⏲️  Duration: {} seconds", duration);
        } else {
            println!("⏲️  Duration: Continuous (Ctrl+C to stop)");
        }
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

        loop {
            // Check if we should stop
            if let Some(end) = end_time {
                if Instant::now() >= end {
                    break;
                }
            }

            // Collect metrics
            match self.coordinator.get_scaling_metrics().await {
                Ok(metrics) => {
                    let snapshot = MetricsSnapshot {
                        timestamp: Instant::now(),
                        active_shards: metrics.active_shards,
                        total_tps: metrics.total_tps,
                        average_latency_ms: metrics.average_latency_ms,
                        memory_usage_mb: metrics.memory_usage_mb,
                        cpu_usage_percent: metrics.cpu_usage_percent,
                        storage_ops_per_sec: metrics.storage_ops_per_sec,
                    };

                    // Detect scaling events
                    if metrics.active_shards != last_shard_count {
                        let event = ScalingEvent {
                            timestamp: Instant::now(),
                            from_shards: last_shard_count,
                            to_shards: metrics.active_shards,
                            trigger: self.determine_scaling_trigger(&metrics),
                        };
                        self.scaling_events.push(event);
                        last_shard_count = metrics.active_shards;
                    }

                    self.metrics_history.push(snapshot.clone());
                    
                    // Keep only recent history (last 60 points)
                    if self.metrics_history.len() > 60 {
                        self.metrics_history.remove(0);
                    }

                    // Display dashboard
                    self.display_dashboard(&snapshot)?;
                }
                Err(e) => {
                    println!("⚠️  Error collecting metrics: {}", e);
                }
            }

            sleep(Duration::from_secs(2)).await;
        }

        // Cleanup terminal
        execute!(stdout(), Show, Clear(ClearType::All))?;
        println!("\n🏁 Monitoring completed!");
        self.display_summary();

        Ok(())
    }

    fn determine_scaling_trigger(&self, metrics: &gridtokenx_blockchain::ScalingMetrics) -> String {
        if metrics.cpu_usage_percent > 75.0 {
            "High CPU".to_string()
        } else if metrics.memory_usage_mb > 400.0 {
            "High Memory".to_string()
        } else if metrics.total_tps > 80.0 {
            "High TPS".to_string()
        } else {
            "Auto-scaling".to_string()
        }
    }

    fn display_dashboard(&self, current: &MetricsSnapshot) -> CrosstermResult<()> {
        execute!(stdout(), Clear(ClearType::All), MoveTo(0, 0))?;

        let runtime = self.start_time.elapsed();
        let timestamp = chrono::Utc::now().format("%H:%M:%S");

        // Header
        execute!(stdout(), 
            SetForegroundColor(Color::Cyan),
            Print(format!("╭─ GridTokenX Live Scaling Dashboard ─ {} ─ Runtime: {}:{:02}:{:02} ─╮\n",
                timestamp,
                runtime.as_secs() / 3600,
                (runtime.as_secs() % 3600) / 60,
                runtime.as_secs() % 60
            )),
            ResetColor
        )?;

        // Current metrics
        execute!(stdout(), Print("│\n"))?;
        execute!(stdout(), 
            Print(format!("│ 🔧 Active Shards: ")),
            SetForegroundColor(Color::Green),
            Print(format!("{:>3}", current.active_shards)),
            ResetColor,
            Print(format!("    📊 TPS: ")),
            SetForegroundColor(Color::Yellow),
            Print(format!("{:>6.1}", current.total_tps)),
            ResetColor,
            Print(format!("    ⚡ Latency: ")),
            SetForegroundColor(Color::Blue),
            Print(format!("{:>5.1}ms", current.average_latency_ms)),
            ResetColor,
            Print("  │\n")
        )?;

        execute!(stdout(), 
            Print(format!("│ 💾 Memory: ")),
            SetForegroundColor(Color::Magenta),
            Print(format!("{:>6.1}MB", current.memory_usage_mb)),
            ResetColor,
            Print(format!("  🔥 CPU: ")),
            SetForegroundColor(Color::Red),
            Print(format!("{:>5.1}%", current.cpu_usage_percent)),
            ResetColor,
            Print(format!("      💽 Storage: ")),
            SetForegroundColor(Color::Cyan),
            Print(format!("{:>5.1} ops/s", current.storage_ops_per_sec)),
            ResetColor,
            Print(" │\n")
        )?;

        execute!(stdout(), Print("│\n"))?;

        // Scaling events
        execute!(stdout(), 
            SetForegroundColor(Color::Yellow),
            Print("│ 📈 Recent Scaling Events:\n"),
            ResetColor
        )?;

        if self.scaling_events.is_empty() {
            execute!(stdout(), Print("│    No scaling events yet...\n"))?;
        } else {
            let recent_events: Vec<_> = self.scaling_events.iter().rev().take(5).collect();
            for event in recent_events {
                let elapsed = event.timestamp.elapsed();
                let direction = if event.to_shards > event.from_shards { "⬆️ UP" } else { "⬇️ DOWN" };
                execute!(stdout(), 
                    Print(format!("│    {} {} → {} shards ({}s ago) - {}\n",
                        direction,
                        event.from_shards,
                        event.to_shards,
                        elapsed.as_secs(),
                        event.trigger
                    ))
                )?;
            }
        }

        execute!(stdout(), Print("│\n"))?;

        // Performance graph (simplified ASCII)
        execute!(stdout(), 
            SetForegroundColor(Color::Green),
            Print("│ 📊 TPS Trend (last 30 samples):\n"),
            ResetColor
        )?;

        if self.metrics_history.len() >= 2 {
            let recent_tps: Vec<_> = self.metrics_history.iter()
                .rev()
                .take(30)
                .map(|m| m.total_tps)
                .collect();

            let max_tps = recent_tps.iter().fold(0.0f64, |a, &b| a.max(b)).max(1.0);
            let graph_width = 50;

            execute!(stdout(), Print("│ TPS: "))?;
            for &tps in recent_tps.iter().rev() {
                let bar_height = ((tps / max_tps) * 8.0) as u8;
                let char = match bar_height {
                    0 => "▁",
                    1 => "▂", 
                    2 => "▃",
                    3 => "▄",
                    4 => "▅",
                    5 => "▆",
                    6 => "▇",
                    _ => "█",
                };
                execute!(stdout(), Print(char))?;
            }
            execute!(stdout(), Print(format!(" (max: {:.1})\n", max_tps)))?;
        } else {
            execute!(stdout(), Print("│    Collecting data...\n"))?;
        }

        execute!(stdout(), Print("│\n"))?;

        // Footer
        execute!(stdout(), 
            SetForegroundColor(Color::Cyan),
            Print("╰──────────────────────────────────────────────────────────────────────────╯\n"),
            ResetColor
        )?;

        execute!(stdout(), Print("Press Ctrl+C to stop monitoring...\n"))?;

        stdout().flush()?;
        Ok(())
    }

    fn display_summary(&self) {
        println!("\n📊 === Scaling Session Summary ===");
        println!("⏱️  Total Runtime: {:?}", self.start_time.elapsed());
        println!("📈 Scaling Events: {}", self.scaling_events.len());
        
        if !self.scaling_events.is_empty() {
            println!("\n🔄 Event Timeline:");
            for (i, event) in self.scaling_events.iter().enumerate() {
                let direction = if event.to_shards > event.from_shards { "⬆️ SCALE UP" } else { "⬇️ SCALE DOWN" };
                println!("   {}. {} from {} to {} shards - {}", 
                    i + 1, direction, event.from_shards, event.to_shards, event.trigger);
            }
        }

        if !self.metrics_history.is_empty() {
            let final_metrics = &self.metrics_history[self.metrics_history.len() - 1];
            println!("\n📊 Final Metrics:");
            println!("   🔧 Shards: {}", final_metrics.active_shards);
            println!("   📊 TPS: {:.1}", final_metrics.total_tps);
            println!("   ⚡ Latency: {:.1}ms", final_metrics.average_latency_ms);
            println!("   💾 Memory: {:.1}MB", final_metrics.memory_usage_mb);
            println!("   🔥 CPU: {:.1}%", final_metrics.cpu_usage_percent);
        }

        println!("\n🎯 GridTokenX Scaling Monitor - Session Complete! 🎯");
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("🚀 Starting GridTokenX Scaling Monitor...");
    
    let mut monitor = ScalingMonitor::new().await?;
    
    // Monitor for 5 minutes by default, or indefinitely if no duration specified
    monitor.start_monitoring(Some(300)).await?;
    
    Ok(())
}
