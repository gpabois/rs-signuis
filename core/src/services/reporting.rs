use actix::prelude::*;
use futures::future::LocalBoxFuture;
use sql_gis::types::Point;

use crate::error::Error;
use crate::events::EventBus;
use crate::forms::reporting::{
    CreateNuisanceFamilyForm, CreateNuisanceReportForm, CreateNuisanceTypeForm,
};
use crate::models::nuisance_family::{NuisanceFamily, NuisanceFamilyId};
use crate::models::nuisance_report::NuisanceReportId;
use crate::models::nuisance_type::NuisanceTypeId;

use crate::models::session::Session;
use crate::repositories::nuisance_family::{
    FetchNuisanceFamilies, InsertNuisanceFamily, NuisanceFamilyExists,
};
use crate::repositories::nuisance_report::InsertNuisanceReport;
use crate::repositories::nuisance_type::{InsertNuisanceType, NuisanceTypeExists};
use crate::repositories::Repository;
use crate::validation::{Validation, Validator};

#[derive(Clone)]
pub struct Reporting(Addr<ReportingActor>);

impl Reporting {
    pub fn new(repos: Repository, events: EventBus) -> Self {
        Self(ReportingActor::new(repos, events).start())
    }

    pub async fn execute<O: ReportingOp>(&self, op: O) -> Result<O::Return, Error> {
        self.0.send(ExecuteReportingOp(op)).await?
    }
}

pub struct ReportingActor {
    repos: Repository,
    events: EventBus,
}

impl ReportingActor {
    pub fn new(repos: Repository, events: EventBus) -> Self {
        Self { repos, events }
    }
}

impl Actor for ReportingActor {
    type Context = Context<Self>;
}

impl<O> Handler<ExecuteReportingOp<O>> for ReportingActor
where
    O: ReportingOp,
{
    type Result = ResponseFuture<Result<O::Return, Error>>;

    fn handle(&mut self, msg: ExecuteReportingOp<O>, _ctx: &mut Self::Context) -> Self::Result {
        let fut = msg.0.execute(self);

        Box::pin(fut)
    }
}

pub trait ReportingOp: Sync + Send + 'static {
    type Return: Sync + Send;

    fn execute<'fut>(
        self,
        reporting: &mut ReportingActor,
    ) -> LocalBoxFuture<'fut, Result<Self::Return, Error>>;
}

pub struct ExecuteReportingOp<O>(O)
where
    O: ReportingOp;

impl<O> Message for ExecuteReportingOp<O>
where
    O: ReportingOp,
{
    type Result = Result<O::Return, Error>;
}

pub struct CreateNuisanceReport {
    pub form: CreateNuisanceReportForm,
    pub session: Session,
}

impl ReportingOp for CreateNuisanceReport {
    type Return = NuisanceReportId;

    fn execute<'fut>(
        self,
        reporting: &mut ReportingActor,
    ) -> LocalBoxFuture<'fut, Result<Self::Return, Error>> {
        let repos = reporting.repos.clone();

        Box::pin(async move {
            let mut validator = Validator::default();
            self.form.assert(&mut validator);
            validator.check()?;

            let user_id = self.session.user().map(|u| u.id);
            let type_id = self.form.type_id.unwrap();

            let exists = repos.execute(NuisanceTypeExists(type_id)).await?;

            validator.assert_true(
                exists,
                Some("le type de nuisance n'existe pas"),
                ["family_id"],
            );
            validator.check()?;

            let intensity: i8 = self
                .form
                .intensity
                .unwrap()
                .try_into()
                .map_err(Error::internal_error_with_source)?;

            let location = Point::from(self.form.location.unwrap());

            let report_id = repos
                .execute(InsertNuisanceReport {
                    user_id,
                    type_id,
                    intensity,
                    location: location.into(),
                })
                .await?;

            Ok(report_id)
        })
    }
}

pub struct CreateNuisanceType {
    pub form: CreateNuisanceTypeForm,
    pub session: Session,
}

impl ReportingOp for CreateNuisanceType {
    type Return = NuisanceTypeId;

    fn execute<'fut>(
        self,
        reporting: &mut ReportingActor,
    ) -> LocalBoxFuture<'fut, Result<Self::Return, Error>> {
        let repos = reporting.repos.clone();

        Box::pin(async move {
            let mut validator = Validator::default();
            self.form.assert(&mut validator);
            validator.check()?;

            let exists = repos
                .execute(NuisanceFamilyExists(self.form.family_id))
                .await?;

            validator.assert_true(
                exists,
                Some("la famille de nuisance n'existe pas"),
                ["family_id"],
            );

            validator.check()?;

            let nuisance_type_id = repos
                .execute(InsertNuisanceType {
                    label: self.form.label,
                    description: self.form.description,
                    family_id: self.form.family_id,
                })
                .await?;

            Ok(nuisance_type_id)
        })
    }
}
#[derive(Message)]
#[rtype(result = "Result<Vec<NuisanceFamily>, Error>")]
pub struct ListNuisanceFamilies {}

impl ListNuisanceFamilies {
    pub fn all() -> Self {
        Self {}
    }
}

impl ReportingOp for ListNuisanceFamilies {
    type Return = Vec<NuisanceFamily>;

    fn execute<'fut>(
        self,
        reporting: &mut ReportingActor,
    ) -> LocalBoxFuture<'fut, Result<Self::Return, Error>> {
        let repos = reporting.repos.clone();

        Box::pin(async move { repos.execute(FetchNuisanceFamilies::all()).await })
    }
}

pub struct CreateNuisanceFamily {
    pub form: CreateNuisanceFamilyForm,
    pub session: Session,
}

impl ReportingOp for CreateNuisanceFamily {
    type Return = NuisanceFamilyId;

    fn execute<'fut>(
        self,
        reporting: &mut ReportingActor,
    ) -> LocalBoxFuture<'fut, Result<Self::Return, Error>> {
        let repos = reporting.repos.clone();

        Box::pin(async move {
            let mut validator = Validator::default();
            self.form.assert(&mut validator);
            validator.check()?;

            let nuisance_family_id = repos
                .execute(InsertNuisanceFamily {
                    label: self.form.label,
                    description: self.form.description,
                })
                .await?;

            Ok(nuisance_family_id)
        })
    }
}
