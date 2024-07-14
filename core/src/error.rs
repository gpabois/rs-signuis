use crate::issues::Issues;

#[derive(Debug)]
pub enum ErrorKind {
    InternalError,
    DatabaseError,
    Invalid(Issues),
    Unauthorized,
}

#[derive(Debug)]
pub struct Error {
    pub kind: ErrorKind,
    source: Option<Box<dyn std::error::Error + 'static + Send + Sync>>,
}

#[cfg(feature = "backend")]
impl From<::sqlx::Error> for Error {
    fn from(value: ::sqlx::Error) -> Self {
        Self {
            kind: ErrorKind::DatabaseError,
            source: Some(Box::new(value)),
        }
    }
}

#[cfg(feature = "backend")]
impl From<::actix::MailboxError> for Error {
    fn from(value: ::actix::MailboxError) -> Self {
        Self {
            kind: ErrorKind::InternalError,
            source: Some(Box::new(value)),
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|s| &**s as _)
    }
}

impl Error {
    pub fn new_with_source(
        kind: ErrorKind,
        source: Option<Box<dyn std::error::Error + Sync + Send + 'static>>,
    ) -> Self {
        Self { kind, source }
    }

    pub fn internal_error_with_source<E: std::error::Error + Sync + Send + 'static>(
        source: E,
    ) -> Self {
        Self {
            kind: ErrorKind::InternalError,
            source: Some(Box::new(source)),
        }
    }
    pub fn internal_error() -> Self {
        Self {
            kind: ErrorKind::InternalError,
            source: None,
        }
    }
    pub fn invalid(issues: Issues) -> Self {
        Self {
            kind: ErrorKind::Invalid(issues),
            source: None,
        }
    }
}
