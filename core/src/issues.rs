use std::ops::Deref;

use serde::{Deserialize, Serialize};

use crate::error::Error;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Issue {
    pub path: Vec<String>,
    pub code: String,
    pub message: String,
}

impl Issue {
    pub fn new<S1: ToString, S2: ToString, S3: ToString, I: IntoIterator<Item = S3>>(
        code: S1,
        message: S2,
        path: I,
    ) -> Self {
        Self {
            code: code.to_string(),
            message: message.to_string(),
            path: path.into_iter().map(|s| s.to_string()).collect(),
        }
    }

    pub fn new_invalid_form<M: Into<String>, E: Into<String>, I: IntoIterator<Item = E>>(
        message: M,
        path: I,
    ) -> Self {
        Self {
            path: path.into_iter().map(|m| m.into()).collect(),
            code: "invalid_form".into(),
            message: message.into(),
        }
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Issues {
    issues: Vec<Issue>,
}

impl Deref for Issues {
    type Target = [Issue];

    fn deref(&self) -> &Self::Target {
        &self.issues
    }
}

impl Issues {
    pub fn new() -> Self {
        Self {
            issues: Vec::default(),
        }
    }

    pub fn iter_by_path<S: ToString, P: IntoIterator<Item = S>>(&self, path: P) -> Vec<&Issue> {
        let path: Vec<String> = path.into_iter().map(|u| u.to_string()).collect();

        self.issues.iter().filter(|i| i.path == path).collect()
    }

    pub fn add(&mut self, issue: Issue) -> &mut Self {
        self.issues.push(issue);
        self
    }

    pub fn into_error(self) -> Error {
        Error::invalid(self)
    }
}
