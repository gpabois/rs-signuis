use std::str::FromStr;

use futures::future::BoxFuture;
use geojson::{Geometry, Value};
use signuis_core::{services::{ServicePool, reporting::traits::Reporting as _}, model::{session::Session, report::{NuisanceFamily, NuisanceType, NuisanceReport}}};
use tonic::{Status, Request, Response};
use uuid::Uuid;

use crate::{codegen, error::into_status};

#[derive(Clone)]
pub struct Reporting(pub ServicePool);

impl codegen::reporting_server::Reporting for Reporting {
    #[must_use]
    #[allow(clippy::type_complexity,clippy::type_repetition_in_bounds)]
    fn create_nuisance_family<'a,'b>(&'a self, request: Request<codegen::NewNuisanceFamily>) 
        ->  BoxFuture<'b, Result<Response<codegen::CreateNuisanceFamilyResult>, Status>> 
    where 'a:'b, Self: 'b {
        Box::pin(async move {
            self.0.with(|tx| {
            Box::pin(async move {
                let actor = request.extensions().get::<Session>().unwrap();
                let result = tx.create_nuisance_family(
                    request
                    .get_ref(), 
                    actor
                ).await;
                Ok(Response::new(result.into()))
            })
        }).await.map_err(into_status)})
    }

    fn create_nuisance_type<'a,'b>(&'a self, request: Request<codegen::NewNuisanceType>) 
        ->  BoxFuture<'b, Result<Response<codegen::CreateNuisanceTypeResult>, Status>> 
    where 'a:'b, Self: 'b {
        Box::pin(async move {
            self.0.with(|tx| {
            Box::pin(async move {
                let actor = request.extensions().get::<Session>().unwrap();
                let result = tx.create_nuisance_type(
                    request
                    .get_ref(), 
                    actor
                ).await;
                Ok(Response::new(result.into()))
            })
        }).await.map_err(into_status)})
    }

    fn report_nuisance<'a,'b>(&'a self, request: Request<codegen::NewNuisanceReport>) 
        ->  BoxFuture<'b, Result<Response<codegen::CreateNuisanceReportResult>, Status>> 
    where 'a:'b, Self: 'b {
        Box::pin(async move {
            self.0.with(|tx| {
            Box::pin(async move {
                let actor = request.extensions().get::<Session>().unwrap();
                let result = tx.report_nuisance(
                    request
                    .get_ref(),
                    actor
                ).await;
                Ok(Response::new(result.into()))
            })
        }).await.map_err(into_status)})
    }
}

impl Into<signuis_core::model::report::NewNuisanceFamily> for &codegen::NewNuisanceFamily {
    fn into(self) -> signuis_core::model::report::NewNuisanceFamily {
        signuis_core::model::report::NewNuisanceFamily {
            label: self.label.clone(),
            description: self.description.clone()
        }
    }
}

impl Into<codegen::NuisanceFamily> for signuis_core::model::report::NuisanceFamily {
    fn into(self) -> codegen::NuisanceFamily {
        codegen::NuisanceFamily {
            id: self.id.to_string(),
            label: self.label,
            description: self.description,
        }
    }
}

impl Into<codegen::create_nuisance_family_result::Args> for Result<NuisanceFamily, signuis_core::Error> {
    fn into(self) -> codegen::create_nuisance_family_result::Args {
        match self {
            Ok(data) => codegen::create_nuisance_family_result::Args::Data(data.into()),
            Err(error) => codegen::create_nuisance_family_result::Args::Error(error.into())
        }
    }
}

impl Into<codegen::CreateNuisanceFamilyResult> for Result<NuisanceFamily, signuis_core::Error> {
    fn into(self) -> codegen::CreateNuisanceFamilyResult {
        codegen::CreateNuisanceFamilyResult {
            ok: self.is_ok(),
            args: Some(self.into())
        }
    }
}

impl Into<signuis_core::model::report::NewNuisanceType> for &codegen::NewNuisanceType {
    fn into(self) -> signuis_core::model::report::NewNuisanceType {
        signuis_core::model::report::NewNuisanceType {
            label: self.label.clone(),
            description: self.description.clone(),
            family_id: Uuid::from_str(&self.family_id).unwrap()
        }
    }
}

impl Into<codegen::NuisanceType> for signuis_core::model::report::NuisanceType {
    fn into(self) -> codegen::NuisanceType {
        codegen::NuisanceType {
            id: self.id.to_string(),
            label: self.label,
            description: self.description,
            family: Some(self.family.into())
        }
    }
}

impl Into<codegen::create_nuisance_type_result::Args> for Result<NuisanceType, signuis_core::Error> {
    fn into(self) -> codegen::create_nuisance_type_result::Args {
        match self {
            Ok(data) => codegen::create_nuisance_type_result::Args::Data(data.into()),
            Err(error) => codegen::create_nuisance_type_result::Args::Error(error.into())
        }
    }
}

impl Into<codegen::CreateNuisanceTypeResult> for Result<NuisanceType, signuis_core::Error> {
    fn into(self) -> codegen::CreateNuisanceTypeResult {
        codegen::CreateNuisanceTypeResult {
            ok: self.is_ok(),
            args: Some(self.into())
        }
    }
}

impl Into<codegen::Geometry> for geojson::Geometry {

    fn into(self) -> codegen::Geometry {
        match self.value {
            Value::Point(coordinates) => codegen::Geometry { 
                r#type: "Point".to_string(), 
                coordinates: coordinates.into_iter().map(|f| f as f32).collect() 
            },
            Value::MultiPoint(coordinates) => todo!(),
            Value::LineString(_) => todo!(),
            Value::MultiLineString(_) => todo!(),
            Value::Polygon(_) => todo!(),
            Value::MultiPolygon(_) => todo!(),
            Value::GeometryCollection(_) => todo!(),
        }
    }
}

impl TryInto<geojson::Geometry> for codegen::Geometry {
    type Error = signuis_core::Error;

    fn try_into(self) -> Result<Geometry, Self::Error> {
        Ok(match self.r#type.as_str() {
            "Point" => Geometry::new(Value::Point(self.coordinates.into_iter().map(|f| f as f64).collect::<Vec<_>>())),
            _ => unreachable!("invalid geometry type")
        })
    }
}

impl TryInto<signuis_core::model::report::NewNuisanceReport> for &codegen::NewNuisanceReport {
    type Error = signuis_core::Error;

    fn try_into(self) -> Result<signuis_core::model::report::NewNuisanceReport, Self::Error> {
        Ok(signuis_core::model::report::NewNuisanceReport {
            location: self.location.clone().unwrap().try_into()?,
            intensity: self.intensity as i8,
            type_id: Uuid::from_str(&self.type_id).unwrap(),
            user_id: None
        })
    }
}

impl Into<codegen::NuisanceReport> for NuisanceReport {
    fn into(self) -> codegen::NuisanceReport {
        codegen::NuisanceReport {
            id: self.id,
            location: self.location.into(),
            r#type: self.r#type,
            intensity: self.intensity,
            user: todo!(),
        }
    }
}

impl Into<codegen::create_nuisance_report_result::Args> for Result<NuisanceReport, signuis_core::Error>  
{
    fn into(self) -> codegen::create_nuisance_report_result::Args {
        match self {
            Ok(data) => codegen::create_nuisance_report_result::Args::Data(data.into()),
            Err(error) => codegen::create_nuisance_report_result::Args::Error(error.into())
        }
    }
}

impl Into<codegen::CreateNuisanceReportResult> for Result<NuisanceReport, signuis_core::Error> {
    fn into(self) -> codegen::CreateNuisanceReportResult {
        codegen::CreateNuisanceReportResult {
            ok: self.is_ok(),
            args: Some(self.into())
        }
    }
}
