use std::ops::{Deref, DerefMut};

pub type GeometryS = Geometry<2, f64>;
pub type PointS = Point<2, f64>;

pub type GeometryZ = Geometry<3, f64>;
pub type PointZ = Point<3, f64>;

#[derive(PartialEq, Eq, Copy, Clone)]
pub enum GeometryType {
    Point,
    LineString,
    Polygon,
    MultiPoint,
    MultiLineString,
    MultiPolygon,
    GeometryCollection,
}

pub struct Geometry<const N: usize, U> {
    pub srid: Option<u32>,
    pub kind: GeometryKind<N, U>
}

impl<const N: usize, U> Deref for Geometry<N, U> {
    type Target = GeometryKind<N, U>;

    fn deref(&self) -> &Self::Target {
        &self.kind
    }
}

impl<U> DerefMut for Geometry<U> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.kind
    }
}

impl Geometry {
    pub fn r#type(&self) -> GeometryType {
        self.kind.r#type()
    }
}

#[derive(PartialEq, Eq, Clone)]
/// Représente une géométrie 2D quelconque (Point, polygone, etc.)
pub enum GeometryKind<const N: usize, U> {
    Point(Point<N, U>),
    LineString(LineString<N, U>),
    Polygon(Polygon<N, U>),
    MultiPoint(MultiPoint<N,U>),
    MultiLineString(MultiLineString<N,U>),
    MultiPolygon(MultiPolygon<N,U>)
}

impl GeometryKind {
    /// Retourne le type sous-jacent de géométrie.
    pub fn r#type(&self) -> GeometryType {
        match self {
            GeometryKind::Point(_) => GeometryType::Point,
            GeometryKind::LineString(_) => GeometryType::LineString,
            GeometryKind::Polygon(_) => GeometryType::Polygon,
            GeometryKind::MultiPoint(_) => GeometryType::MultiPoint,
            GeometryKind::MultiLineString(_) => GeometryType::MultiLineString,
            GeometryKind::MultiPolygon(_) => GeometryType::MultiPolygon
        }
    }
}

/// Un vecteur dimension N.
#[derive(PartialEq, Eq, Clone)]
pub struct Vector<const N: usize, U>([U;N]);

impl<const N: usize, U> Vector<N, U> {
    pub fn new(coordinates: [U;N]) -> Self {
        Self(coordinates)
    }
}

impl<const N: usize, U> Deref for Vector<N, U> {
    type Target = [U;N];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<const N: usize, U> DerefMut for Vector<N, U> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}


/// Un ensemble de vecteur de dimension N.
pub struct VectorSet<const N: usize, U>(Vec<Vector<N, U>>);

impl<const N: usize, U> Deref for VectorSet<N, U> {
    type Target = [[U;N]];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<const N: usize, U> DerefMut for Vector<N, U> {

    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}


/// Un point dans un espace n-d.
#[derive(PartialEq, Eq, Clone)]
pub struct Point<const N: usize, U> {
    coordinates: Vector<N, U>,
    srid: u32
}

impl<const N: usize, U> Point<N,U> {
    pub fn new(coordinates: Vector<N, U>) -> Self {
        Self { coordinates, srid: 4326 }
    }

    pub fn new_with_srid(coordinates: Vector<N, U>, srid: u32) -> Self {
        Self {coordinates, srid }
    }
}

impl<const N: usize, U> Deref for Point<N, U> {
    type Target = [U;N];

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
    coordinates: VectorSet<N,U>
}

/// Une suite reliée de points dans un espace 2D.
#[derive(PartialEq, Eq, Clone)]
pub struct LineString<const N: usize, U>  {
    coordinates: VectorSet<N,U>,
    srid: u32
}

impl<const N: usize, U> LineString<N, U> {
    pub fn new(coordinates: VectorSet<N,U>) -> Self {
        Self { coordinates, srid: 4326 }
    }

    pub fn new_with_srid(coordinates: VectorSet<N,U>, srid: u32) -> Self {
        Self { coordinates, srid }
    }
}


/// Un ensemble de lignes brisées.
pub struct MultiLineString<const N: usize, U>  {
    coordinates: Vec<VectorSet<N,U>>
}

/// Un polygone
pub struct Polygon<const N: usize, U>  {
    coordinates: Vec<VectorSet<N,U>>
}

/// Un enemble de polygones
pub struct MultiPolygon<const N: usize, U>  {
    coordinates: Vec<Vec<VectorSet<N,U>>>
}