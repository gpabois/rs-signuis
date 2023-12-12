// Inspired by Zod
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

pub enum Error {
    DatabaseError(String),
    ValidationError(Vec<Issue>),
    InvalidCredentials
}

impl Error {
    pub fn invalid_credentials() -> Self {
        return Self::InvalidCredentials;
    }
    pub fn validator_error<T: Into<Vec<Issue>>>(issues: T) -> Self {
        return Self::ValidationError(issues.into())
    }
}

impl<D> Into<Result<D, Error>> for Error {
    fn into(self) -> Result<D, Error> {
        Result::Err(self)
    }
}

impl From<sqlx::Error> for Error {
    fn from(value: sqlx::Error) -> Self {
        Self::DatabaseError(value.to_string())
    }
}