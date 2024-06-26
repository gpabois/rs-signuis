use std::ops::{Deref, DerefMut};

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
    /// 2D Point
    PointS,
    /// 2D Line String
    LineStringS,
    /// 2D Polygon
    PolygonS,
    MultiPointS,
    MultiLineStringS,
    MultiPolygonS,
    GeometryCollectionS,

    /// 3D Point
    PointZ,
    LineStringZ,
    PolygonZ,
    MultiPointZ,
    MultiLineStringZ,
    MultiPolygonZ,
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

impl From<PointS> for Geometry {
    fn from(value: PointS) -> Self {
        Self::PointS(value)
    }
}

impl From<LineStringS> for Geometry {
    fn from(value: LineStringS) -> Self {
        Self::LineStringS(value)
    }
}

impl From<PolygonS> for Geometry {
    fn from(value: PolygonS) -> Self {
        Self::PolygonS(value)
    }
}

impl From<MultiPointS> for Geometry {
    fn from(value: MultiPointS) -> Self {
        Self::MultiPointS(value)
    }
}

impl From<MultiLineStringS> for Geometry {
    fn from(value: MultiLineStringS) -> Self {
        Self::MultiLineStringS(value)
    }
}

impl From<MultiPolygonS> for Geometry {
    fn from(value: MultiPolygonS) -> Self {
        Self::MultiPolygonS(value)
    }
}

impl From<PointZ> for Geometry {
    fn from(value: PointZ) -> Self {
        Self::PointZ(value)
    }
}

impl From<LineStringZ> for Geometry {
    fn from(value: LineStringZ) -> Self {
        Self::LineStringZ(value)
    }
}

impl From<PolygonZ> for Geometry {
    fn from(value: PolygonZ) -> Self {
        Self::PolygonZ(value)
    }
}

impl From<MultiPointZ> for Geometry {
    fn from(value: MultiPointZ) -> Self {
        Self::MultiPointZ(value)
    }
}

impl From<MultiLineStringZ> for Geometry {
    fn from(value: MultiLineStringZ) -> Self {
        Self::MultiLineStringZ(value)
    }
}

impl From<MultiPolygonZ> for Geometry {
    fn from(value: MultiPolygonZ) -> Self {
        Self::MultiPolygonZ(value)
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

/// Un tableau de vecteur de dimension N.
pub struct VectorArray<const N: usize, U>(Vec<Vector<N, U>>);

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

/// Une matrice de vecteur de dimension N.
pub struct VectorMatrix<const N: usize, U>(Vec<VectorArray<N, U>>);

impl<const N: usize, U> VectorMatrix<N, U> {
    pub fn new(coordinates: Vec<VectorArray<N, U>>) -> Self {
        Self(coordinates)
    }
}

pub struct VectorTensor<const N: usize, U>(Vec<VectorMatrix<N, U>>);

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
    coordinates: VectorArray<N, U>,
    srid: u32,
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
    coordinates: VectorArray<N, U>,
    srid: u32,
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

/// Un ensemble de lignes brisées.
pub struct MultiLineString<const N: usize, U> {
    coordinates: Vec<VectorArray<N, U>>,
}

/// Un polygone
pub struct Polygon<const N: usize, U> {
    coordinates: VectorMatrix<N, U>,
    srid: u32,
}

impl<const N: usize, U> Polygon {
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

/// Un enemble de polygones
pub struct MultiPolygon<const N: usize, U> {
    coordinates: Vec<Vec<VectorArray<N, U>>>,
}
