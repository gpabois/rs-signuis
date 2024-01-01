use log::SetLoggerError;
use sqlx::migrate::MigrateError;

use crate::Issue;

#[derive(Debug)]
pub enum Error {
    MissingConfiguration(String),
    MigrationError(MigrateError),
    DatabaseError(sqlx::Error),
    ValidationError(Vec<Issue>),
    LoggerError(SetLoggerError),
    InvalidCredential,
    Unauthorized
}

impl Error {
    pub fn code(&self) -> String {
        match self {
            Self::MissingConfiguration(_) => "missing_configuration".into(),
            Self::MigrationError(_) => "database_migration_error".into(),
            Self::DatabaseError(_) => "database_error".into(),
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

impl<D> Into<Result<D, Error>> for Error {
    fn into(self) -> Result<D, Error> {
        Result::Err(self)
    }
}

impl From<sqlx::migrate::MigrateError> for Error {
    fn from(value: sqlx::migrate::MigrateError) -> Self {
        Self::MigrationError(value)
    } 
}

impl From<SetLoggerError> for Error {
    fn from(value: SetLoggerError) -> Self {
        Self::LoggerError(value)
    }
}

impl From<sqlx::Error> for Error {
    fn from(value: sqlx::Error) -> Self {
        Self::DatabaseError(value)
    }
}