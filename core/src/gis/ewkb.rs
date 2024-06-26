use std::{convert::Infallible, io::Cursor};
use byteorder::{BigEndian, ByteOrder, LittleEndian, ReadBytesExt};

use super::{LineString, Point, Vector, VectorSet};

#[derive(Copy, Clone)]
pub struct WKBGeometryType {
    kind: GeometryType,
    with_z: bool
}

impl From<WKBGeometryType> for u32 {
    fn from(value: WKBGeometryType) -> Self {
        let mut u: u32 = match value.kind {
            GeometryType::Point => 1,
            GeometryType::LineString => 2,
            GeometryType::Polygon => 3,
            GeometryType::MultiPoint => 4,
            GeometryType::MultiLineString => 5,
            GeometryType::MultiPolygon => 6,
            GeometryType::GeometryCollection => 7,
        };

        if value.with_z {
            u += 0x80000000;
        }

        u
    }
}

impl TryFrom<u32> for WKBGeometryType {
    type Error = Infallible;

    fn try_from(mut value: u32) -> Result<Self, Self::Error> {
        let with_z = if value > 0x8000000 {
            value -= 0x8000000;
            true
        } else {false};

        let kind = match value {
            1 => GeometryType::Point,
            2 => GeometryType::LineString,
            3 => GeometryType::Polygon,
            4 => GeometryType::MultiPoint,
            5 => GeometryType::MultiLineString,
            6 => GeometryType::MultiPolygon,
            7 => GeometryType::GeometryCollection,
            _ => panic!("invalid WKB geometry type")
        };

        Ok(Self { with_z, kind })
    }
}

pub enum WKBGeometry {
    PointS(PointS),
    PointZ(PointZ)
}

const BIG_ENDIAN: u8 = 0;
const LITTLE_ENDIAN: u8 = 1;

impl TryFrom<&[u8]> for WKBGeometry {
    type Error = std::io::Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let mut buf = Cursor::new(value);
        
        // 0: Big Endian, 1: Little Endian
        let endianess = buf.read_u8()?;

        let r#type = WKBGeometryType.try_from(if endianess == BIG_ENDIAN {
            buf.read_u32::<BigEndian>()?
        } else {
            buf.read_u32::<LittleEndian>()?
        })?;

        match r#type.kind {
            GeometryType::Point => {

            },
            GeometryType::LineString => {

            },
            GeometryType::Polygon => {

            },
            GeometryType::MultiPoint => {

            },
            GeometryType::MultiLineString => {

            },
            GeometryType::MultiPolygon => {

            },
            GeometryType::GeometryCollection => {

            },
        }
    }
}

impl<const N: usize> Point<N, f64> {
    /// Décode un point depuis un flux binaire
    fn try_decode<ENDIAN: ByteOrder>(value: &mut Cursor<&[u8]>) -> Result<Self, std::io::Error> {
        let srid = value.read_u32()?;
        let coordinates = Vector::<N, f64>::try_decode(value)?;
        Self::new_with_srid(coordinates, srid)
    }
}

impl<const N: usize> LineString<N, f64> {
    /// Décode un point depuis un flux binaire
    fn try_decode<ENDIAN: ByteOrder>(value: &mut Cursor<&[u8]>) -> Result<Self, std::io::Error> {
        let srid = value.read_u32()?;
        let coordinates = Vector::<N, f64>::try_decode(value)?;
        Self::new_with_srid(coordinates, srid)
    } 
}

impl<const N: usize> Vector<N, f64> {
    /// Décode un vecteur n-D
    fn try_decode<ENDIAN: ByteOrder>(value: &mut Cursor<&[u8]>) -> Result<Self, std::io::Error> {
        let mut coordinates: [f64; N] = [0; N];
        
        for i in 0..N {
            coordinates[i] = value.read_f64::<ENDIAN>()?;
        }

        Self::new(coordinates)
    }
}

impl<const N: usize> VectorSet<N, f64> {
    /// Décode un vecteur n-D
    fn try_decode<ENDIAN: ByteOrder>(value: &mut Cursor<&[u8]>) -> Result<Self, std::io::Error> {
        let nb_points: u32 = value.read_u32()?;
        let set = Vec::<Vector<N, f32>>::with_capacity(nb_points as usize);

        let mut coordinates: [f64; N] = [0; N];
        
        for i in 0..N {
            coordinates[i] = value.read_f64::<ENDIAN>()?;
        }

        Self::new(coordinates)
    }
}

pub enum WKBGeometryKind {
    
}