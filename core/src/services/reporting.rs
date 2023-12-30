use futures::future::BoxFuture;
use sqlx::Acquire;

use crate::model::report::{NewNuisanceReport, NewNuisanceFamily, NewNuisanceType, NuisanceReport, InsertNuisanceReport, InsertNuisanceFamily, NuisanceFamily, NuisanceType, InsertNuisanceType};
use crate::model::session::Session;
use crate::repositories::nuisance_families::traits::NuisanceFamilyRepository;
use crate::repositories::nuisance_reports::traits::NuisanceReportRepository;
use crate::repositories::nuisance_types::traits::NuisanceTypeRepository;
use crate::{Error, Validator, Issue, Issues};

use super::logger::logs::NuisanceReportCreated;
use super::logger::traits::Logger;

pub mod traits {
    use futures::future::BoxFuture;

    use crate::{model::{report::{NewNuisanceReport, NuisanceReport, NewNuisanceFamily, NuisanceFamily, NewNuisanceType, NuisanceType}, session::Session}, Error};

    pub trait Reporting<'q> {
        /// Report a nuisance
        fn report_nuisance<'a, 'b, NR: Into<NewNuisanceReport> + std::marker::Send + 'b>(self, args: NR, actor: &'a Session) -> BoxFuture<'b, Result<NuisanceReport, Error>> where 'q: 'b, 'a: 'b;
        /// Create a nuisance family
        fn create_nuisance_family<'a, 'b, NF: Into<NewNuisanceFamily> + std::marker::Send + 'b>(self, args: NF, actor: &'a Session) -> BoxFuture<'b, Result<NuisanceFamily, Error>> where 'q: 'b;
        /// Create a nuisance type
        fn create_nuisance_type<'a, 'b, NT: Into<NewNuisanceType> + std::marker::Send + 'b>(self, args: NT, actor: &'a Session) -> BoxFuture<'b, Result<NuisanceType, Error>> where 'q: 'b;
    }
}

impl Validator for &NewNuisanceReport {
    fn validate(self, issues: &mut crate::Issues) {

        issues.geojson_is_point(
            &self.location, 
            Issue::new_invalid_form(
            "La localisation doit être un point.", 
            ["location"]
            )
        );

        issues.within(
            self.intensity, 
            (1, 5), 
            Issue::new_invalid_form(
                "L'intensité doit être compris entre 1 et 5", 
                ["intensity"]
            )
        )
    }
}

impl Into<InsertNuisanceReport> for NewNuisanceReport {
    fn into(self) -> InsertNuisanceReport {
        InsertNuisanceReport {
            id: None,
            type_id: self.type_id,
            user_id: self.user_id,
            location: self.location,
            intensity: self.intensity,
            created_at: None
        }
    }
}

impl Validator for &NewNuisanceFamily {
    fn validate(self, issues: &mut Issues) {
        issues.not_empty(
            &self.label, 
            Issue::new_invalid_form(
                "Le libellé ne doit pas être vide", 
                ["label"]
            )
        )
    }
}

impl Into<InsertNuisanceFamily> for NewNuisanceFamily {
    fn into(self) -> InsertNuisanceFamily {
        InsertNuisanceFamily {
            id: None,
            label: self.label,
            description: self.description
        }
    }
}

impl Validator for &NewNuisanceType {
    fn validate(self, issues: &mut Issues) {
        issues.not_empty(
            &self.label, 
            Issue::new_invalid_form(
                "Le libellé ne doit pas être vide", 
                ["label"]
            )
        )
    }
}

impl Into<InsertNuisanceType> for NewNuisanceType {
    fn into(self) -> InsertNuisanceType {
        InsertNuisanceType {
            id: None,
            label: self.label,
            description: self.description,
            family_id: self.family_id
        }
    }
}

impl<'q> traits::Reporting<'q> for &'q mut super::ServiceTx<'_> {
    fn report_nuisance<'a, 'b, NR: Into<NewNuisanceReport> + std::marker::Send + 'b>(self, args: NR, actor: &'a Session) -> BoxFuture<'b, Result<NuisanceReport, Error>> where 'q: 'b, 'a: 'b{
        Box::pin(async {
            let mut new = args.into();
            // Inject user 
            new.user_id = actor.user.as_ref().map(|u| u.id);
            
            let mut issues = Issues::new();        
            new.validate(&mut issues);
    
            issues.assert_valid()?;

            let report = {
                let querier = self.querier.acquire().await?;
                let insert: InsertNuisanceReport = new.into();
                self.repos.insert_nuisance_report(querier, insert).await?
            };

            self.log(NuisanceReportCreated::new(&report, actor)).await?;

            Ok(report)
        })
    }

    fn create_nuisance_family<'a, 'b, NF: Into<NewNuisanceFamily> + std::marker::Send + 'b>(self, args: NF, _actor: &'a Session) -> BoxFuture<'b, Result<NuisanceFamily, Error>> where 'q: 'b {
        Box::pin(async {
            let new_nuisance_family = args.into();
            let mut issues = Issues::new();        
            new_nuisance_family.validate(&mut issues);
            issues.assert_valid()?;

            let nuisance_family = {
                let querier = self.querier.acquire().await?;
                let insert: InsertNuisanceFamily = new_nuisance_family.into();
                self.repos.insert_nuisance_family(querier, insert).await?
            };

            Ok(nuisance_family)
        })
    }

    fn create_nuisance_type<'a, 'b, NT: Into<NewNuisanceType> + std::marker::Send + 'b>(self, args: NT, _actor: &'a Session) -> BoxFuture<'b, Result<NuisanceType, Error>> where 'q: 'b {
        Box::pin(async {
            let new = args.into();
            let mut issues = Issues::new();        
            new.validate(&mut issues);
            issues.assert_valid()?;

            let nuisance_type = {
                let querier = self.querier.acquire().await?;
                self.repos.insert_nuisance_type(querier, new).await?
            };

            Ok(nuisance_type)
        })
    }
}