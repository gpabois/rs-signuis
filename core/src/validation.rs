use email_address::EmailAddress;

use crate::issues::{Issues, Issue};
use crate::error::Error;


pub trait Validation {
    fn assert(&self, validator: &mut Validator);
}

#[derive(Default)]
pub struct Validator {
    issues: Issues
}

impl Validator {
    pub fn check(&self) -> Result<(), Error> {
        if !self.issues.is_empty() {
            return Err(Error::invalid(self.issues.clone()))
        }

        return Ok(())
    }
    pub fn assert_valid_email<S: ToString, P: IntoIterator<Item=S>>(&mut self, value: &str, message: Option<&str>, path: P) {
        if !EmailAddress::is_valid(value) {
            let issue = Issue::new(
                "invalid", 
                message.unwrap_or("the email is invalid"),
                path.into_iter().map(ToString::to_string)
            );

            self.issues.push(issue);
        }
    }

    pub fn assert_not_true<A: PartialEq, S: ToString, P: IntoIterator<Item=S>>(&mut self, a: bool, message: Option<&str>, path: P) {
        if a {
            let issue = Issue::new(
                "invalid", 
                message.unwrap_or("both elements are not equal"),
                path.into_iter().map(ToString::to_string)
            );

            self.issues.push(issue);
        }
    }


    pub fn assert_eq<A: PartialEq, S: ToString, P: IntoIterator<Item=S>>(&mut self, a: A, b: A, message: Option<&str>, path: P) {
        if a != b {
            let issue = Issue::new(
                "invalid", 
                message.unwrap_or("both elements are not equal"),
                path.into_iter().map(ToString::to_string)
            );

            self.issues.push(issue);
        }
    }


    pub fn assert_not_empty<S: ToString, P: IntoIterator<Item=S>>(&mut self, value: &str, message: Option<&str>, path: P) {
        if value.is_empty() {
            let issue = Issue::new(
                "invalid", 
                message.unwrap_or("element is empty"),
                path.into_iter().map(ToString::to_string)
            );

            self.issues.push(issue);
        }
    }
}
