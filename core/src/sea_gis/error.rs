use super::GeometryKind;

pub enum Error {
    InvalidGeometryKind {
        expecting: GeometryKind,
        got: GeometryKind
    }
}

impl Error {
    pub fn invalid_geometry_kind(expecting: GeometryKind, got: GeometryKind) -> Self {
        Self::InvalidGeometryKind { expecting, got }
    }
}