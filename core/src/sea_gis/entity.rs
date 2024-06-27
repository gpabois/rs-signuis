use std::ops::{Deref, DerefMut};

use sea_orm::Value;

use super::EWKBGeometry;

pub type PointS = Point<2, f64>;
pub type MultiPointS = MultiPoint<2, f64>;
pub type LineStringS = LineString<2, f64>;
pub type MultiLineStringS = MultiLineString<2, f64>;
pub type PolygonS = Polygon<2, f64>;
pub type MultiPolygonS = MultiPolygon<2, f64>;

pub type PointZ = Point<3, f64>;
pub type MultiPointZ = MultiPoint<3, f64>;
pub type LineStringZ = LineString<3, f64>;
pub type MultiLineStringZ = MultiLineString<3, f64>;
pub type PolygonZ = Polygon<3, f64>;
pub type MultiPolygonZ = MultiPolygon<3, f64>;

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum GeometryKind {
    /// 2D point
    PointS,
    /// 2D line string
    LineStringS,
    /// 2D polygon
    PolygonS,
    /// 2D set of points
    MultiPointS,
    /// 2D set of line strings
    MultiLineStringS,
    /// 2D set of polygons
    MultiPolygonS,
    /// 2D set of geometries
    GeometryCollectionS,

    /// 3D point
    PointZ,
    /// 3D line string
    LineStringZ,
    /// 3D polygon
    PolygonZ,
    /// 3D set of points
    MultiPointZ,
    /// 3D set of line strings
    MultiLineStringZ,
    /// 3D set of polygons
    MultiPolygonZ,
    /// 3D set of geometries
    GeometryCollectionZ,
}

pub enum Geometry {
    PointS(PointS),
    LineStringS(LineStringS),
    PolygonS(PolygonS),
    MultiPointS(MultiPointS),
    MultiLineStringS(MultiLineStringS),
    MultiPolygonS(MultiPolygonS),

    PointZ(PointZ),
    LineStringZ(LineStringZ),
    PolygonZ(PolygonZ),
    MultiPointZ(MultiPointZ),
    MultiLineStringZ(MultiLineStringZ),
    MultiPolygonZ(MultiPolygonZ),
}

impl Geometry {
    pub fn kind(&self) -> GeometryKind {
        match self {
            Geometry::PointS(_) => GeometryKind::PointS,
            Geometry::LineStringS(_) => GeometryKind::LineStringS,
            Geometry::PolygonS(_) => GeometryKind::PolygonS,
            Geometry::MultiPointS(_) => GeometryKind::MultiPolygonS,
            Geometry::MultiLineStringS(_) => GeometryKind::MultiLineStringS,
            Geometry::MultiPolygonS(_) => GeometryKind::MultiPolygonS,
            Geometry::PointZ(_) => GeometryKind::PointZ,
            Geometry::LineStringZ(_) => GeometryKind::LineStringZ,
            Geometry::PolygonZ(_) => GeometryKind::PolygonZ,
            Geometry::MultiPointZ(_) => GeometryKind::MultiPointZ,
            Geometry::MultiLineStringZ(_) => GeometryKind::MultiLineStringZ,
            Geometry::MultiPolygonZ(_) => GeometryKind::MultiPolygonZ,
        }
    }

    pub fn mbr(&self) -> MBR<f64> {
        match self {
            Geometry::PointS(a) => a.mbr(),
            Geometry::LineStringS(a) => a.mbr(),
            Geometry::PolygonS(a) => a.mbr(),
            Geometry::MultiPointS(a) => a.mbr(),
            Geometry::MultiLineStringS(a) => a.mbr(),
            Geometry::MultiPolygonS(a) => a.mbr(),
            Geometry::PointZ(a) => a.mbr(),
            Geometry::LineStringZ(a) => a.mbr(),
            Geometry::PolygonZ(a) => a.mbr(),
            Geometry::MultiPointZ(a) => a.mbr(),
            Geometry::MultiLineStringZ(a) => a.mbr(),
            Geometry::MultiPolygonZ(a) => a.mbr(),
        }
    }

    pub fn set_srid(&mut self, srid: u32) {
        match self {
            Geometry::PointS(a) => a.srid = srid,
            Geometry::LineStringS(a) => a.srid = srid,
            Geometry::PolygonS(a) => a.srid = srid,
            Geometry::MultiPointS(a) => a.srid = srid,
            Geometry::MultiLineStringS(a) => a.srid = srid,
            Geometry::MultiPolygonS(a) => a.srid = srid,
            Geometry::PointZ(a) => a.srid = srid,
            Geometry::LineStringZ(a) => a.srid = srid,
            Geometry::PolygonZ(a) => a.srid = srid,
            Geometry::MultiPointZ(a) => a.srid = srid,
            Geometry::MultiLineStringZ(a) => a.srid = srid,
            Geometry::MultiPolygonZ(a) => a.srid = srid,
        }       
    }

    pub fn srid(&self) -> u32 {
        match self {
            Geometry::PointS(p) => p.srid,
            Geometry::LineStringS(ls) => ls.srid,
            Geometry::PolygonS(p) => p.srid,
            Geometry::MultiPointS(a) => a.srid,
            Geometry::MultiLineStringS(a) => a.srid,
            Geometry::MultiPolygonS(a) => a.srid,
            Geometry::PointZ(a) => a.srid,
            Geometry::LineStringZ(a) => a.srid,
            Geometry::PolygonZ(a) => a.srid,
            Geometry::MultiPointZ(a) => a.srid,
            Geometry::MultiLineStringZ(a) => a.srid,
            Geometry::MultiPolygonZ(a) => a.srid,
        }
    }
}

impl From<Geometry> for Value {
    /// Par défaut, les formes canoniques sont encodées en EWKB 
    fn from(value: Geometry) -> Self {
        EWKBGeometry::from(value).into()
    }
}

impl sea_orm::sea_query::Nullable for EWKBGeometry {
    fn null() -> sea_orm::Value {
        sea_orm::Value::Bytes(None)
    }
}

impl ValueType for EWKBGeometry {
    fn try_from(v: Value) -> Result<Self, sea_query::ValueTypeErr> {
        match v {
            Value::Bytes(Some(buf)) => EWKBGeometry::try_from(&buf).map_err(|| sea_query::ValueTypeErr),
            _ => Err(sea_query::ValueTypeErr)
        }
    }

    fn type_name() -> String {
        stringify!(EWKBGeometry).to_owned()
    }

    fn array_type() -> sea_query::ArrayType {
        sea_orm::sea_query::ArrayType::Bytes
    }

    fn column_type() -> sea_orm::ColumnType {
        sea_orm::sea_query::ColumnType::Bit(None)
    }
}

impl From<PointS> for Geometry {
    fn from(value: PointS) -> Self {
        Self::PointS(value)
    }
}

impl TryFrom<Geometry> for PointS {
    type Error = super::error::Error;

    fn try_from(value: Geometry) -> Result<Self, Self::Error> {
        match value {
            Geometry::PointS(point) => Ok(point),
            _ => Err(super::error::Error::invalid_geometry_kind(GeometryKind::PointS, value.kind()))
        }
    }
}

impl From<LineStringS> for Geometry {
    fn from(value: LineStringS) -> Self {
        Self::LineStringS(value)
    }
}

impl TryFrom<Geometry> for LineStringS {
    type Error = super::error::Error;

    fn try_from(value: Geometry) -> Result<Self, Self::Error> {
        match value {
            Geometry::LineStringS(line_string) => Ok(line_string),
            _ => Err(super::error::Error::invalid_geometry_kind(GeometryKind::LineStringS, value.kind()))
        }
    }
}

impl From<PolygonS> for Geometry {
    fn from(value: PolygonS) -> Self {
        Self::PolygonS(value)
    }
}

impl TryFrom<Geometry> for PolygonS {
    type Error = super::error::Error;

    fn try_from(value: Geometry) -> Result<Self, Self::Error> {
        match value {
            Geometry::PolygonS(polygon) => Ok(polygon),
            _ => Err(super::error::Error::invalid_geometry_kind(GeometryKind::PolygonS, value.kind()))
        }
    }
}

impl From<MultiPointS> for Geometry {
    fn from(value: MultiPointS) -> Self {
        Self::MultiPointS(value)
    }
}

impl TryFrom<Geometry> for MultiPointS {
    type Error = super::error::Error;

    fn try_from(value: Geometry) -> Result<Self, Self::Error> {
        match value {
            Geometry::MultiPointS(points) => Ok(points),
            _ => Err(super::error::Error::invalid_geometry_kind(GeometryKind::MultiPointS, value.kind()))
        }
    }
}

impl From<MultiLineStringS> for Geometry {
    fn from(value: MultiLineStringS) -> Self {
        Self::MultiLineStringS(value)
    }
}

impl TryFrom<Geometry> for MultiLineStringS {
    type Error = super::error::Error;

    fn try_from(value: Geometry) -> Result<Self, Self::Error> {
        match value {
            Geometry::MultiLineStringS(line_strings) => Ok(line_strings),
            _ => Err(super::error::Error::invalid_geometry_kind(GeometryKind::MultiLineStringS, value.kind()))
        }
    }
}

impl From<MultiPolygonS> for Geometry {
    fn from(value: MultiPolygonS) -> Self {
        Self::MultiPolygonS(value)
    }
}

impl TryFrom<Geometry> for MultiPolygonS {
    type Error = super::error::Error;

    fn try_from(value: Geometry) -> Result<Self, Self::Error> {
        match value {
            Geometry::MultiPolygonS(polygons) => Ok(polygons),
            _ => Err(super::error::Error::invalid_geometry_kind(GeometryKind::MultiPolygonS, value.kind()))
        }
    }
}

impl From<PointZ> for Geometry {
    fn from(value: PointZ) -> Self {
        Self::PointZ(value)
    }
}

impl TryFrom<Geometry> for PointZ {
    type Error = super::error::Error;

    fn try_from(value: Geometry) -> Result<Self, Self::Error> {
        match value {
            Geometry::PointZ(point) => Ok(point),
            _ => Err(super::error::Error::invalid_geometry_kind(GeometryKind::PointZ, value.kind()))
        }
    }
}

impl From<LineStringZ> for Geometry {
    fn from(value: LineStringZ) -> Self {
        Self::LineStringZ(value)
    }
}

impl TryFrom<Geometry> for LineStringZ {
    type Error = super::error::Error;

    fn try_from(value: Geometry) -> Result<Self, Self::Error> {
        match value {
            Geometry::LineStringZ(line_string) => Ok(line_string),
            _ => Err(super::error::Error::invalid_geometry_kind(GeometryKind::LineStringZ, value.kind()))
        }
    }
}

impl From<PolygonZ> for Geometry {
    fn from(value: PolygonZ) -> Self {
        Self::PolygonZ(value)
    }
}

impl TryFrom<Geometry> for PolygonZ {
    type Error = super::error::Error;

    fn try_from(value: Geometry) -> Result<Self, Self::Error> {
        match value {
            Geometry::PolygonZ(polygon) => Ok(polygon),
            _ => Err(super::error::Error::invalid_geometry_kind(GeometryKind::PolygonZ, value.kind()))
        }
    }
}

impl From<MultiPointZ> for Geometry {
    fn from(value: MultiPointZ) -> Self {
        Self::MultiPointZ(value)
    }
}

impl TryFrom<Geometry> for MultiPointZ {
    type Error = super::error::Error;

    fn try_from(value: Geometry) -> Result<Self, Self::Error> {
        match value {
            Geometry::MultiPointZ(points) => Ok(points),
            _ => Err(super::error::Error::invalid_geometry_kind(GeometryKind::MultiPointZ, value.kind()))
        }
    }
}

impl From<MultiLineStringZ> for Geometry {
    fn from(value: MultiLineStringZ) -> Self {
        Self::MultiLineStringZ(value)
    }
}

impl TryFrom<Geometry> for MultiLineStringZ {
    type Error = super::error::Error;

    fn try_from(value: Geometry) -> Result<Self, Self::Error> {
        match value {
            Geometry::MultiLineStringZ(line_strings) => Ok(line_strings),
            _ => Err(super::error::Error::invalid_geometry_kind(GeometryKind::MultiLineStringZ, value.kind()))
        }
    }
}

impl From<MultiPolygonZ> for Geometry {
    fn from(value: MultiPolygonZ) -> Self {
        Self::MultiPolygonZ(value)
    }
}

impl TryFrom<Geometry> for MultiPolygonZ {
    type Error = super::error::Error;

    fn try_from(value: Geometry) -> Result<Self, Self::Error> {
        match value {
            Geometry::MultiPolygonZ(polygons) => Ok(polygons),
            _ => Err(super::error::Error::invalid_geometry_kind(GeometryKind::MultiPolygonZ, value.kind()))
        }
    }
}


/// Un vecteur dimension N.
#[derive(PartialEq, Eq, Clone)]
pub struct Vector<const N: usize, U>([U; N]);

impl<const N: usize, U> Vector<N, U> {
    pub fn new(coordinates: [U; N]) -> Self {
        Self(coordinates)
    }
}

impl<const N: usize, U> Vector<N, U> where U: Copy {
    pub fn x(&self) -> U {
        self.0[0]
    }

    pub fn y(&self) -> U {
        self.0[1]
    }
}

impl<const N: usize, U> Deref for Vector<N, U> {
    type Target = [U; N];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<const N: usize, U> DerefMut for Vector<N, U> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// Un tableau 1D de vecteur de dimension N.
pub struct VectorArray<const N: usize, U>(Vec<Vector<N, U>>);

impl<const N: usize, U> VectorArray<N, U> 
where U: Copy + Ord
{
    pub fn min_x(&self) -> U {
        self.0.iter().map(Vector::x).min().copied().unwrap()
    }

    pub fn max_x(&self) -> U {
        self.0.iter().map(Vector::x).max().copied().unwrap()
    }

    pub fn min_y(&self) -> U {
        self.0.iter().map(Vector::y).min().copied().unwrap()
    }

    pub fn max_y(&self) -> U {
        self.0.iter().map(Vector::y).max().copied().unwrap()
    }
}

impl<const N: usize, U> FromIterator<Vector<N, U>> for VectorArray<N, U> {
    fn from_iter<T: IntoIterator<Item = Vector<N, U>>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl<const N: usize, U> Deref for VectorArray<N, U> {
    type Target = [Vector<N, U>];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Une matrice 2D de vecteur de dimension N.
pub struct VectorMatrix<const N: usize, U>(Vec<VectorArray<N, U>>);

impl<const N: usize, U> VectorMatrix<N, U> {
    pub fn new(coordinates: Vec<VectorArray<N, U>>) -> Self {
        Self(coordinates)
    }
}

impl<const N: usize, U> VectorMatrix<N, U> 
where U: Copy + Ord
{
    pub fn min_x(&self) -> U {
        self.0.iter().map(VectorArray::min_x).min().copied().unwrap()
    }

    pub fn max_x(&self) -> U {
        self.0.iter().map(VectorArray::max_x).max().copied().unwrap()
    }

    pub fn min_y(&self) -> U {
        self.0.iter().map(VectorArray::min_y).min().copied().unwrap()
    }

    pub fn max_y(&self) -> U {
        self.0.iter().map(VectorArray::max_y).max().copied().unwrap()
    }
}

/// Un tenseur 3D de vecteur de dimension N
pub struct VectorTensor<const N: usize, U>(Vec<VectorMatrix<N, U>>);

impl<const N: usize, U> VectorTensor<N, U> 
where U: Copy + Ord
{
    pub fn min_x(&self) -> U {
        self.0.iter().map(VectorMatrix::min_x).min().copied().unwrap()
    }

    pub fn max_x(&self) -> U {
        self.0.iter().map(VectorMatrix::max_x).max().copied().unwrap()
    }

    pub fn min_y(&self) -> U {
        self.0.iter().map(VectorMatrix::min_y).min().copied().unwrap()
    }

    pub fn max_y(&self) -> U {
        self.0.iter().map(VectorMatrix::max_y).max().copied().unwrap()
    }
}

/// Un point dans un espace n-d.
#[derive(PartialEq, Eq, Clone)]
pub struct Point<const N: usize, U> {
    pub coordinates: Vector<N, U>,
    pub srid: u32,
}

impl<const N: usize, U> Point<N, U> {
    pub fn new(coordinates: Vector<N, U>) -> Self {
        Self {
            coordinates,
            srid: 4326,
        }
    }

    pub fn new_with_srid(coordinates: Vector<N, U>, srid: u32) -> Self {
        Self { coordinates, srid }
    }
}

impl<const N: usize, U> Point<N, U> where U: Copy {
    pub fn mbr(&self) -> MBR<U> {
        MBR {
            min_x: self.coordinates.x(),
            max_x: self.coordinates.x(),
            min_y: self.coordinates.y(),
            max_y: self.coordinates.y()
        }
    }
}

impl<const N: usize, U> Deref for Point<N, U> {
    type Target = [U; N];

    fn deref(&self) -> &Self::Target {
        self.coordinates.deref()
    }
}

impl<const N: usize, U> DerefMut for Point<N, U> {
    fn deref_mut(&self) -> &Self::Target {
        self.coordinates.deref_mut()
    }
}

/// Un ensemble de point non relié dans un espace 2D.
pub struct MultiPoint<const N: usize, U> {
    pub coordinates: VectorArray<N, U>,
    pub srid: u32,
}

impl<const N: usize, U> MultiPoint<N, U> where U: Copy {
    pub fn mbr(&self) -> MBR<U> {
        MBR {
            min_x: self.coordinates.min_x(),
            max_x: self.coordinates.max_x(),
            min_y: self.coordinates.min_y(),
            max_y: self.coordinates.max_y()
        }
    }
}

impl<const N: usize, U> MultiPoint<N, U> {
    pub fn new(coordinates: VectorArray<N, U>) -> Self {
        Self {
            coordinates,
            srid: 4326,
        }
    }

    pub fn new_with_srid(coordinates: VectorArray<N, U>, srid: u32) -> Self {
        Self { coordinates, srid }
    }
}

/// Une suite reliée de points dans un espace 2D.
#[derive(PartialEq, Eq, Clone)]
pub struct LineString<const N: usize, U> {
    pub coordinates: VectorArray<N, U>,
    pub srid: u32,
}

impl<const N: usize, U> LineString<N, U> {
    pub fn new(coordinates: VectorArray<N, U>) -> Self {
        Self {
            coordinates,
            srid: 4326,
        }
    }

    pub fn new_with_srid(coordinates: VectorArray<N, U>, srid: u32) -> Self {
        Self { coordinates, srid }
    }
}

impl<const N: usize, U> LineString<N, U> where U: Copy {
    pub fn mbr(&self) -> MBR<U> {
        MBR {
            min_x: self.coordinates.min_x(),
            max_x: self.coordinates.max_x(),
            min_y: self.coordinates.min_y(),
            max_y: self.coordinates.max_y()
        }
    }
}

/// Un ensemble de lignes brisées.
pub struct MultiLineString<const N: usize, U> {
    pub coordinates: Vec<VectorArray<N, U>>,
    pub srid: u32
}

impl<const N: usize, U> MultiLineString<N, U> {
    pub fn new(coordinates: VectorMatrix<N, U>) -> Self {
        Self {
            coordinates,
            srid: 4326,
        }
    }

    pub fn new_with_srid(coordinates: VectorMatrix<N, U>, srid: u32) -> Self {
        Self { coordinates, srid }
    }
}

impl<const N: usize, U> MultiLineString<N, U> where U: Copy {
    pub fn mbr(&self) -> MBR<U> {
        MBR {
            min_x: self.coordinates.min_x(),
            max_x: self.coordinates.max_x(),
            min_y: self.coordinates.min_y(),
            max_y: self.coordinates.max_y()
        }
    }
}

/// Un polygone
pub struct Polygon<const N: usize, U> {
    pub coordinates: VectorMatrix<N, U>,
    pub srid: u32,
}

impl<const N: usize, U> Polygon<N,U> {
    pub fn new(coordinates: VectorMatrix<N, U>) -> Self {
        Self {
            coordinates,
            srid: 4326,
        }
    }

    pub fn new_with_srid(coordinates: VectorMatrix<N, U>, srid: u32) -> Self {
        Self { coordinates, srid }
    }
}

impl<const N: usize, U> Polygon<N, U> where U: Copy {
    pub fn mbr(&self) -> MBR<U> {
        MBR {
            min_x: self.coordinates.min_x(),
            max_x: self.coordinates.max_x(),
            min_y: self.coordinates.min_y(),
            max_y: self.coordinates.max_y()
        }
    }
}

/// Un ensemble de polygones
pub struct MultiPolygon<const N: usize, U> {
    pub coordinates: VectorTensor<N, U>,
    pub srid: u32
}

impl<const N: usize, U> MultiPolygon<N,U> {
    pub fn new(coordinates: VectorTensor<N, U>) -> Self {
        Self {
            coordinates,
            srid: 4326,
        }
    }

    pub fn new_with_srid(coordinates: VectorTensor<N, U>, srid: u32) -> Self {
        Self { coordinates, srid }
    }
}

impl<const N: usize, U> MultiPolygon<N, U> where U: Copy {
    pub fn mbr(&self) -> MBR<U> {
        MBR {
            min_x: self.coordinates.min_x(),
            max_x: self.coordinates.max_x(),
            min_y: self.coordinates.min_y(),
            max_y: self.coordinates.max_y()
        }
    }
}

/// Rectangle à limite minimum (minimum bounding rectangle)
pub struct MBR<U> {
    pub min_x: U,
    pub min_y: U,
    pub max_x: U,
    pub max_y: U
}

