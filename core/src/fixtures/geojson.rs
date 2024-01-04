use fake::{faker::address::fr_fr::{Longitude, Latitude}, Fake};
use crate::types::geojson::Geometry;

pub fn random_point() -> Geometry {
    let coords = vec![Longitude().fake(), Latitude().fake()];
    Geometry(geojson::Geometry::new(geojson::Value::Point(coords)))
}