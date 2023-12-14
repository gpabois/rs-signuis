use sqlx::migrate::MigrateError;

#[derive(Debug)]
pub struct Issue {
    pub path: Vec<String>,
    pub code: String,
    pub message: String
}

pub struct IssueBuilder {
    issues: Vec<Issue>
}

impl IssueBuilder {
    pub fn new() -> Self {
        return Self {
            issues: Vec::default()
        }
    }

    pub fn add(mut self, issue: Issue) -> Self {
        self.issues.push(issue);
        return self
    }
}

impl Into<Vec<Issue>> for IssueBuilder {
    fn into(self) -> Vec<Issue> {
        return self.issues
    }
}

#[derive(Debug)]
pub enum Error {
    MissingEnv(String),
    MigrationError(MigrateError),
    DatabaseError(sqlx::Error),
    ValidationError(Vec<Issue>),
    InvalidCredentials
}

impl Error {
    pub fn invalid_credentials() -> Self {
        return Self::InvalidCredentials;
    }
    pub fn validation_error<T: Into<Vec<Issue>>>(issues: T) -> Self {
        return Self::ValidationError(issues.into())
    }
    pub fn missing_env<S: Into<String>>(value: S) -> Self {
        return Self::MissingEnv(value.into());
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

impl From<sqlx::Error> for Error {
    fn from(value: sqlx::Error) -> Self {
        Self::DatabaseError(value)
    }
}