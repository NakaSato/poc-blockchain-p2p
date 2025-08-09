//! Logging Infrastructure
//!
//! Provides structured logging capabilities across the application.

use serde::{Serialize, Deserialize};
use std::collections::HashMap;

/// Log level enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

impl std::fmt::Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LogLevel::Trace => write!(f, "TRACE"),
            LogLevel::Debug => write!(f, "DEBUG"),
            LogLevel::Info => write!(f, "INFO"),
            LogLevel::Warn => write!(f, "WARN"),
            LogLevel::Error => write!(f, "ERROR"),
        }
    }
}

/// Log entry structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEntry {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub level: LogLevel,
    pub message: String,
    pub module: String,
    pub fields: HashMap<String, serde_json::Value>,
}

impl LogEntry {
    pub fn new(level: LogLevel, message: String, module: String) -> Self {
        Self {
            timestamp: chrono::Utc::now(),
            level,
            message,
            module,
            fields: HashMap::new(),
        }
    }
    
    pub fn with_field(mut self, key: &str, value: serde_json::Value) -> Self {
        self.fields.insert(key.to_string(), value);
        self
    }
    
    pub fn with_fields(mut self, fields: HashMap<String, serde_json::Value>) -> Self {
        self.fields.extend(fields);
        self
    }
}

/// Logger trait
pub trait Logger: Send + Sync {
    fn log(&self, entry: LogEntry);
    fn trace(&self, message: &str, module: &str);
    fn debug(&self, message: &str, module: &str);
    fn info(&self, message: &str, module: &str);
    fn warn(&self, message: &str, module: &str);
    fn error(&self, message: &str, module: &str);
}

/// Structured logger implementation
pub struct StructuredLogger {
    min_level: LogLevel,
    format: LogFormat,
}

#[derive(Debug, Clone)]
pub enum LogFormat {
    Json,
    Text,
}

impl StructuredLogger {
    pub fn new(min_level: LogLevel, format: LogFormat) -> Self {
        Self { min_level, format }
    }
    
    fn should_log(&self, level: LogLevel) -> bool {
        level >= self.min_level
    }
    
    fn format_entry(&self, entry: &LogEntry) -> String {
        match self.format {
            LogFormat::Json => {
                serde_json::to_string(entry).unwrap_or_else(|_| {
                    format!("{{\"error\": \"Failed to serialize log entry\", \"original_message\": \"{}\"}}", entry.message)
                })
            }
            LogFormat::Text => {
                let fields_str = if entry.fields.is_empty() {
                    String::new()
                } else {
                    let fields: Vec<String> = entry.fields.iter()
                        .map(|(k, v)| format!("{}={}", k, v))
                        .collect();
                    format!(" [{}]", fields.join(", "))
                };
                
                format!(
                    "{} {} [{}] {}{}",
                    entry.timestamp.format("%Y-%m-%d %H:%M:%S%.3f UTC"),
                    entry.level,
                    entry.module,
                    entry.message,
                    fields_str
                )
            }
        }
    }
}

impl Logger for StructuredLogger {
    fn log(&self, entry: LogEntry) {
        if self.should_log(entry.level) {
            let formatted = self.format_entry(&entry);
            
            // In a real implementation, you might want to send to different outputs
            // based on log level (stdout, stderr, files, external systems, etc.)
            match entry.level {
                LogLevel::Error => eprintln!("{}", formatted),
                _ => println!("{}", formatted),
            }
        }
    }
    
    fn trace(&self, message: &str, module: &str) {
        let entry = LogEntry::new(LogLevel::Trace, message.to_string(), module.to_string());
        self.log(entry);
    }
    
    fn debug(&self, message: &str, module: &str) {
        let entry = LogEntry::new(LogLevel::Debug, message.to_string(), module.to_string());
        self.log(entry);
    }
    
    fn info(&self, message: &str, module: &str) {
        let entry = LogEntry::new(LogLevel::Info, message.to_string(), module.to_string());
        self.log(entry);
    }
    
    fn warn(&self, message: &str, module: &str) {
        let entry = LogEntry::new(LogLevel::Warn, message.to_string(), module.to_string());
        self.log(entry);
    }
    
    fn error(&self, message: &str, module: &str) {
        let entry = LogEntry::new(LogLevel::Error, message.to_string(), module.to_string());
        self.log(entry);
    }
}

impl Default for StructuredLogger {
    fn default() -> Self {
        Self::new(LogLevel::Info, LogFormat::Text)
    }
}

/// Logger configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggerConfig {
    pub min_level: LogLevel,
    pub format: String, // "json" or "text"
    pub output: String,  // "stdout", "stderr", or file path
}

impl Default for LoggerConfig {
    fn default() -> Self {
        Self {
            min_level: LogLevel::Info,
            format: "text".to_string(),
            output: "stdout".to_string(),
        }
    }
}

/// Logging macros for convenience
#[macro_export]
macro_rules! log_trace {
    ($logger:expr, $msg:expr) => {
        $logger.trace($msg, module_path!())
    };
    ($logger:expr, $msg:expr, $($field:expr),+) => {
        {
            let entry = LogEntry::new(LogLevel::Trace, $msg.to_string(), module_path!().to_string())
                $(.with_field(stringify!($field), serde_json::json!($field)))+;
            $logger.log(entry);
        }
    };
}

#[macro_export]
macro_rules! log_info {
    ($logger:expr, $msg:expr) => {
        $logger.info($msg, module_path!())
    };
    ($logger:expr, $msg:expr, $($field:expr),+) => {
        {
            let entry = LogEntry::new(LogLevel::Info, $msg.to_string(), module_path!().to_string())
                $(.with_field(stringify!($field), serde_json::json!($field)))+;
            $logger.log(entry);
        }
    };
}

#[macro_export]
macro_rules! log_error {
    ($logger:expr, $msg:expr) => {
        $logger.error($msg, module_path!())
    };
    ($logger:expr, $msg:expr, $($field:expr),+) => {
        {
            let entry = LogEntry::new(LogLevel::Error, $msg.to_string(), module_path!().to_string())
                $(.with_field(stringify!($field), serde_json::json!($field)))+;
            $logger.log(entry);
        }
    };
}
