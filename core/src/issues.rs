use email_address::EmailAddress;
use futures::future::BoxFuture;
use geojson::{Geometry, Value};

use crate::Error;

#[derive(Debug, Clone)]
pub struct Issue {
    pub path: Vec<String>,
    pub code: String,
    pub message: String
}

impl Issue {
    pub fn new(code: String, message: String, path: Vec<String>) -> Self {
        Self{code, path, message}
    }

    pub fn new_invalid_form<M: Into<String>, E: Into<String>, I: IntoIterator<Item=E>>(message: M, path: I) -> Self {
        Self { 
            path: path.into_iter().map(|m| m.into()).collect(), 
            code: "invalid_form".into(), 
            message: message.into()
        }
    }
}

pub struct Issues {
    issues: Vec<Issue>
}

pub trait Validator {
    fn validate(self, issues: &mut Issues);
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
    pub fn assert_valid(&self) -> Result<(), Error> {
        if self.issues.len() > 0 {
            return Err(Error::ValidationError(self.issues.clone()));
        }

        Ok(())
    }

    pub async fn async_assert_true(&mut self, f: BoxFuture<'_, Result<bool, Error>>, issue: Issue) -> Result<(), Error> {
        if !f.await? {
            self.add(issue);
            return Err(Error::ValidationError(self.issues.clone()));
        }

        Ok(())
    }

    pub async fn async_true(&mut self, f: BoxFuture<'_, Result<bool, Error>>, issue: Issue) -> Result<(), Error> {
        if f.await? {
            self.add(issue);
        }

        Ok(())
    }

    pub async fn async_is_success<D>(&mut self, f: BoxFuture<'_, Result<D, Error>>, issue: Issue) -> Result<D, Error> {
        let result = f.await;
        if result.is_err() {
            self.add(issue);
        }
        result
    }
    
    pub fn assert_some<D>(&mut self, value: Option<D>, issue: Issue) -> Result<D, Error> {
        match value {
            None => {
                self.add(issue);
                Err(Error::ValidationError(self.issues.clone()))
            },
            Some(value) => Ok(value)
        }
    }

    pub fn assert_success<D, E>(&mut self, value: Result<D, E>, issue: Issue) -> Result<D, Error> {
        match value {
            Err(_) => {
                self.add(issue);
                Err(Error::ValidationError(self.issues.clone()))
            },
            Ok(value) => Ok(value)
        }
    }

    pub fn is_success<D, E>(&mut self, value: Result<D, E>, issue: Issue) -> Result<D, E> {
        if value.is_err() {
            self.add(issue);
        }
        value
    }

    pub fn is_some<D>(&mut self, value: Option<D>, issue: Issue) -> Option<D> {
        if value.is_none() {
            self.add(issue);
        }
        value
    }

    pub fn eq<A: PartialEq>(&mut self, a: A, b: A, issue: Issue) {
        if a != b {
            self.add(issue);
        }
    }

    pub fn within<A: PartialOrd>(&mut self, a: A, bounds: (A, A), issue: Issue) {
        if a < bounds.0 || a > bounds.1 {
            self.add(issue);
        }
    }

    pub fn assert_lt<A: PartialOrd>(&mut self, a: A, b: A, issue: Issue) -> Result<A, Error> {
        if a >= b {
            self.add(issue);
            return Err(Error::ValidationError(self.issues.clone()));
        }     

        Ok(a)
    }

    pub fn lt<A: PartialOrd>(&mut self, a: A, b: A, issue: Issue) {
        if a < b {
            self.add(issue);
        }
    }

    pub fn not_empty(&mut self, value: &str, issue: Issue) {
        if value.is_empty() {
            self.add(issue);
        }
    }

    pub fn email(&mut self, value: &str, issue: Issue) {
        if !EmailAddress::is_valid(value) {
            self.add(issue);
        }
    }

    pub fn geojson_is_point(&mut self, geom: &Geometry, issue: Issue) {
        match geom.value {
            Value::Point(_) => {},
            _ => {
                self.add(issue);
            }
        };
    }
}

impl Into<Vec<Issue>> for Issues {
    fn into(self) -> Vec<Issue> {
        return self.issues
    }
}