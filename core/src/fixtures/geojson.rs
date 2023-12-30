use fake::{faker::address::fr_fr::{Longitude, Latitude}, Fake};
use geojson::{Geometry, Value};

pub fn random_point() -> Geometry {
    let coords = vec![Longitude().fake(), Latitude().fake()];
    Geometry::new(Value::Point(coords))
}