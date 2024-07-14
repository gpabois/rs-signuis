use std::ops::RangeInclusive;

use email_address::EmailAddress;

use crate::error::Error;
use crate::issues::{Issue, Issues};

pub trait Validation {
    fn assert(&self, validator: &mut Validator);
}

#[derive(Default, Debug)]
pub struct Validator {
    pub issues: Issues,
}

impl Validator {
    pub fn check(&self) -> Result<(), Error> {
        if !self.issues.is_empty() {
            return Err(Error::invalid(self.issues.clone()));
        }

        Ok(())
    }
    pub fn assert_valid_email<S: ToString, P: IntoIterator<Item = S>>(
        &mut self,
        value: &str,
        message: Option<&str>,
        path: P,
    ) {
        if !EmailAddress::is_valid(value) {
            let issue = Issue::new("invalid", message.unwrap_or("the email is invalid"), path);

            self.issues.add(issue);
        }
    }

    pub fn assert_in_range_inclusive<U: PartialOrd, S: ToString, P: IntoIterator<Item = S>>(
        &mut self,
        value: &U,
        range: RangeInclusive<U>,
        message: Option<&str>,
        path: P,
    ) {
        if !range.contains(value) {
            let issue = Issue::new("invalid", message.unwrap_or("value is not in range"), path);
            self.issues.add(issue);
        }
    }

    pub fn assert_is_some<T, S: ToString, P: IntoIterator<Item = S>>(
        &mut self,
        value: &Option<T>,
        message: Option<&str>,
        path: P,
    ) {
        if value.is_none() {
            let issue = Issue::new("invalid", message.unwrap_or("value is None"), path);
            self.issues.add(issue);
        }
    }

    pub fn assert_false<S: ToString, P: IntoIterator<Item = S>>(
        &mut self,
        a: bool,
        message: Option<&str>,
        path: P,
    ) {
        if a {
            let issue = Issue::new("invalid", message.unwrap_or("value must be false"), path);

            self.issues.add(issue);
        }
    }

    pub fn assert_true<S: ToString, P: IntoIterator<Item = S>>(
        &mut self,
        a: bool,
        message: Option<&str>,
        path: P,
    ) {
        if !a {
            let issue = Issue::new("invalid", message.unwrap_or("value must be true"), path);

            self.issues.add(issue);
        }
    }

    pub fn assert_eq<A: PartialEq, S: ToString, P: IntoIterator<Item = S>>(
        &mut self,
        a: A,
        b: A,
        message: Option<&str>,
        path: P,
    ) {
        if a != b {
            let issue = Issue::new(
                "invalid",
                message.unwrap_or("both elements are not equal"),
                path,
            );

            self.issues.add(issue);
        }
    }

    pub fn assert_not_empty<S: ToString, P: IntoIterator<Item = S>>(
        &mut self,
        value: &str,
        message: Option<&str>,
        path: P,
    ) {
        if value.is_empty() {
            let issue = Issue::new("invalid", message.unwrap_or("element is empty"), path);

            self.issues.add(issue);
        }
    }
}
