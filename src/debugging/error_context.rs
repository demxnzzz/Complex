//! Rich error handling for production diagnosis
//!
//! Production-grade error types with context for uncertain failures,
//! distinguishing recoverable vs fatal errors.

use std::fmt;

/// Error severity level
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ErrorSeverity {
    Warning,
    Recoverable,
    Critical,
    Fatal,
}

/// Production error with rich context
#[derive(Clone, Debug)]
pub struct SystemError {
    pub code: u32,
    pub severity: ErrorSeverity,
    pub message: String,
    pub context: Option<String>,
    pub source_location: Option<(String, u32)>, // file, line
}

impl SystemError {
    /// Create a new system error
    pub fn new(code: u32, severity: ErrorSeverity, message: &str) -> Self {
        Self {
            code,
            severity,
            message: message.to_string(),
            context: None,
            source_location: None,
        }
    }

    /// Add context information
    pub fn with_context(mut self, context: &str) -> Self {
        self.context = Some(context.to_string());
        self
    }

    /// Set source location
    pub fn with_location(mut self, file: &str, line: u32) -> Self {
        self.source_location = Some((file.to_string(), line));
        self
    }

    /// Check if error is recoverable
    pub fn is_recoverable(&self) -> bool {
        matches!(self.severity, ErrorSeverity::Warning | ErrorSeverity::Recoverable)
    }
}

impl fmt::Display for SystemError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "[{:?}] Error {}: {}",
            self.severity, self.code, self.message
        )?;
        if let Some(ctx) = &self.context {
            write!(f, " ({})", ctx)?;
        }
        if let Some((file, line)) = &self.source_location {
            write!(f, " [{}:{}]", file, line)?;
        }
        Ok(())
    }
}

impl std::error::Error for SystemError {}

/// Result type for system operations
pub type SysResult<T> = Result<T, SystemError>;
