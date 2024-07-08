use log::SetLoggerError;
use sqlx::migrate::MigrateError;

use crate::issues::{self, Issues};

#[derive(Debug)]
pub enum ErrorKind {
    InternalError,
    MissingConfiguration(String),
    MigrationError(MigrateError),
    DatabaseError,
    Invalid(Issues),
    LoggerError(SetLoggerError),
    InvalidCredential,
    Unauthorized
}

pub struct Error {
    kind: ErrorKind,
    source: Option<Box<dyn std::error::Error + 'static>>
}

impl From<sqlx::Error> for Error {
    fn from(value: sqlx::Error) -> Self {
        Self {
            kind: ErrorKind::DatabaseError,
            source: Some(Box::new(value))
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source()
    }

    fn cause(&self) -> Option<&dyn std::error::Error> {
        self.source()
    }
}

impl Error {
    pub fn new_with_source(kind: ErrorKind, source: Option<Box<dyn std::error::Error + 'static>>) {
        Self { kind, source }
    }

    pub fn internal_error<E: std::error::Error>(source: E) -> Self {
        Self{kind: ErrorKind::InternalError, source: Box::new(source)}
    }

    pub fn invalid(issues: Issues) -> Self {
        Self {kind: ErrorKind::InvalidError(), source: None}
    }

    pub fn invalid_credential() -> Self {
        Self{kind: ErrorKind::InvalidCredential, source: None}
    }
}

impl ErrorKind {
    pub fn code(&self) -> String {
        match self {
            Self::MissingConfiguration(_) => "missing_configuration".into(),
            Self::MigrationError(_) => "database_migration_error".into(),
            Self::DatabaseError => "database_error".into(),
            Self::ValidationError(_) => "validation_error".into(),
            Self::LoggerError(_) => "logging_error".into(),
            Self::InvalidCredential => "invalid_credential".into(),
            Self::Unauthorized => "unauthorized".into()
        }
    }

    pub fn is_validation_error(&self) -> bool {
        match self {
            Self::ValidationError(_) => true,
            _ => false
        }
    }

    pub fn issues_or_empty(&self) -> Vec<Issue> {
        match self {
            Self::ValidationError(issues) => issues.clone(),
            _ => vec![]
        }
    }

    pub fn message(&self) -> String {
        match self {
            Self::MissingConfiguration(cfg) => format!("missing configuration {cfg}").into(),
            Self::MigrationError(error) => format!("migration failed, reason: {error}").into(),
            Self::DatabaseError(error) => format!("failed database operation, reason: {error}").into(),
            Self::ValidationError(_) => "validation has failed".into(),
            Self::LoggerError(error) => format!("{error}").into(),
            Self::InvalidCredential => format!("given credential is invalid").into(),
            Self::Unauthorized => "you are not allowed to perfom this action".into() 
        }
    }

    pub fn unauthorized() -> Self {
        return Self::Unauthorized
    }
    pub fn invalid_credentials() -> Self {
        return Self::InvalidCredential;
    }
    pub fn validation_error<T: Into<Vec<Issue>>>(issues: T) -> Self {
        return Self::ValidationError(issues.into())
    }
    pub fn missing_env<S: Into<String>>(value: S) -> Self {
        return Self::MissingConfiguration(value.into());
    }
}