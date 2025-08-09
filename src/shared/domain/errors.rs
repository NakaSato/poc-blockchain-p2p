//! Domain Error Types
//!
//! Defines all error types that can occur within the domain layer.
//! These errors represent business rule violations and invalid domain states.

use std::fmt;

/// Result type alias for domain operations
pub type DomainResult<T> = Result<T, DomainError>;

/// Domain error types following DDD patterns
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DomainError {
    BusinessRuleViolation { message: String },
    InvalidValue { message: String },
    InvalidOperation { message: String },
    AggregateNotFound { id: String },
    ConcurrencyConflict { message: String },
    ConstraintViolation { constraint: String },
    Infrastructure { message: String },
    ExternalService { service: String, message: String },
    Authorization { message: String },
    ResourceExhausted { resource: String },
    Timeout { operation: String },
}

impl DomainError {
    pub fn business_rule_violation<S: Into<String>>(message: S) -> Self {
        Self::BusinessRuleViolation { message: message.into() }
    }
    
    pub fn invalid_value<S: Into<String>>(message: S) -> Self {
        Self::InvalidValue { message: message.into() }
    }
    
    pub fn invalid_operation<S: Into<String>>(message: S) -> Self {
        Self::InvalidOperation { message: message.into() }
    }
    
    pub fn aggregate_not_found<S: Into<String>>(id: S) -> Self {
        Self::AggregateNotFound { id: id.into() }
    }
    
    pub fn concurrency_conflict<S: Into<String>>(message: S) -> Self {
        Self::ConcurrencyConflict { message: message.into() }
    }
    
    pub fn constraint_violation<S: Into<String>>(constraint: S) -> Self {
        Self::ConstraintViolation { constraint: constraint.into() }
    }
    
    pub fn infrastructure<S: Into<String>>(message: S) -> Self {
        Self::Infrastructure { message: message.into() }
    }
    
    pub fn external_service<S: Into<String>, T: Into<String>>(service: S, message: T) -> Self {
        Self::ExternalService { 
            service: service.into(), 
            message: message.into() 
        }
    }
    
    pub fn authorization<S: Into<String>>(message: S) -> Self {
        Self::Authorization { message: message.into() }
    }
    
    pub fn resource_exhausted<S: Into<String>>(resource: S) -> Self {
        Self::ResourceExhausted { resource: resource.into() }
    }
    
    pub fn timeout<S: Into<String>>(operation: S) -> Self {
        Self::Timeout { operation: operation.into() }
    }
}

impl fmt::Display for DomainError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DomainError::BusinessRuleViolation { message } => {
                write!(f, "Business rule violation: {}", message)
            }
            DomainError::InvalidValue { message } => {
                write!(f, "Invalid value: {}", message)
            }
            DomainError::InvalidOperation { message } => {
                write!(f, "Invalid operation: {}", message)
            }
            DomainError::AggregateNotFound { id } => {
                write!(f, "Aggregate not found: {}", id)
            }
            DomainError::ConcurrencyConflict { message } => {
                write!(f, "Concurrency conflict: {}", message)
            }
            DomainError::ConstraintViolation { constraint } => {
                write!(f, "Domain constraint violation: {}", constraint)
            }
            DomainError::Infrastructure { message } => {
                write!(f, "Infrastructure error: {}", message)
            }
            DomainError::ExternalService { service, message } => {
                write!(f, "External service error: {} - {}", service, message)
            }
            DomainError::Authorization { message } => {
                write!(f, "Authorization error: {}", message)
            }
            DomainError::ResourceExhausted { resource } => {
                write!(f, "Resource exhausted: {}", resource)
            }
            DomainError::Timeout { operation } => {
                write!(f, "Timeout: {}", operation)
            }
        }
    }
}

impl std::error::Error for DomainError {}

/// Convert anyhow errors to domain errors
impl From<anyhow::Error> for DomainError {
    fn from(error: anyhow::Error) -> Self {
        DomainError::infrastructure(error.to_string())
    }
}
