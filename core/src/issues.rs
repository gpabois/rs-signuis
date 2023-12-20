use futures::future::BoxFuture;

use crate::Error;

#[derive(Debug)]
pub struct Issue {
    pub path: Vec<String>,
    pub code: String,
    pub message: String
}

impl Issue {
    pub fn new(code: String, message: String, path: Vec<String>) -> Self {
        Self{code, path, message}
    }
}

pub struct Issues {
    issues: Vec<Issue>
}

impl Issues {
    pub fn new() -> Self {
        return Self {
            issues: Vec::default()
        }
    }

    pub fn add(&mut self, issue: Issue) -> &mut Self {
        self.issues.push(issue);
        return self
    }
}

impl Issues {
    pub fn validate(self) -> Result<(), Error> {
        if self.issues.len() > 0 {
            return Err(Error::ValidationError(self.issues));
        }

        Ok(())
    }

    pub async fn async_add_if_not_true(&mut self, f: BoxFuture<'_, Result<bool, Error>>, issue: Issue) -> Result<(), Error> {
        if f.await? {
            self.add(issue);
        }

        Ok(())
    }
}

impl Into<Vec<Issue>> for Issues {
    fn into(self) -> Vec<Issue> {
        return self.issues
    }
}