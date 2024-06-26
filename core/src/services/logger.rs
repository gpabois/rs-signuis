use futures::future::BoxFuture;
use crate::{Error, models::log::NewLog};
use sqlx::Acquire;

use crate::{repositories::logs::traits::LogsRepository, models::log::Log};

pub mod traits {
    use futures::future::BoxFuture;

    use crate::{models::log::{NewLog, Log, LogFilter}, Error};

    pub trait Logger<'q> {
        /// Log an event
        fn log<'b, L: Into<NewLog> + Send + 'b>(self, args: L) -> BoxFuture<'b, Result<Log, Error>> where 'q: 'b;

        fn count_log_by<'b>(self, filter: LogFilter) -> BoxFuture<'b, Result<i64, Error>> where 'q: 'b;
    }
}

pub mod logs {
    use crate::models::{user_session::Session, log::NewLog, nuisance_family::NuisanceReport};

    pub struct NuisanceReportCreated<'a, 'b>(&'a NuisanceReport, &'b Session);
    impl<'a, 'b> NuisanceReportCreated<'a, 'b> {
        pub fn new(report: &'a NuisanceReport, by: &'b Session) -> Self {
            Self(report, by)
        }
    }
    impl<'a, 'b> Into<NewLog> for NuisanceReportCreated<'a, 'b> {
        fn into(self) -> NewLog {
            NewLog::new("report::created")
            .from_actor(&self.1)
            .to_owned()
        }
    }

    pub struct AuthenticationFailed<'a>(&'a Session);

    impl<'a> AuthenticationFailed<'a> {
        pub fn new(actor: &'a Session) -> Self {
            Self(actor)
        }
    }

    impl<'a> Into<NewLog> for AuthenticationFailed<'a> {
        fn into(self) -> NewLog {
            NewLog::new("authentication::failed")
            .from_actor(&self.0)
            .to_owned()
        }
    }
}

impl<'q> traits::Logger<'q> for &'q mut super::ServiceTx<'_>
{
    fn log<'b, L: Into<NewLog> + Send + 'b>(self, args: L) -> BoxFuture<'b, Result<Log, Error>> where 'q: 'b {
        Box::pin(async {
            let new: NewLog = args.into();
            let querier = self.querier.acquire().await?;
            self.repos.insert_log(querier, new.into()).await
        })
    }

    fn count_log_by<'b>(self, args: crate::models::log::LogFilter) -> BoxFuture<'b, Result<i64, Error>> where 'q: 'b {
        Box::pin(async {
            let querier = self.querier.acquire().await?;
            self.repos.count_log_by(querier, args).await
        })
    }
}