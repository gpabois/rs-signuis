use std::io::Read;
use byteorder::{BigEndian, ByteOrder, LittleEndian, ReadBytesExt};

use super::{Geometry, GeometryKind, Point};

const BIG_ENDIAN: u8 = 0;
const LITTLE_ENDIAN: u8 = 1;

/// Objet intermédiaire pour encoder/décoder une géométrie au format natif de SpatiaLite.
pub struct SpatiaLiteGeometry(Geometry);

impl<T> From<T> for SpatiaLiteGeometry
where
    Geometry: From<T>,
{
    fn from(value: T) -> Self {
        Self(value.into())
    }
}


impl TryFrom<&[u8]> for SpatiaLiteGeometry {
    type Error = super::error::Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let mut buf = Cursor::new(value);
        Self::decode(&mut buf)   
    }
}

impl SpatiaLiteGeometry 
{
    pub fn decode<R: Read>(stream: &mut R) -> Result<Self, std::io::Error> {
        let start = stream.read_u8()?;
        let endian = stream.read_u8()?;

        if endian = BIG_ENDIAN {
            Self::decode_with_endianess::<BigEndian>(stream)
        } else {
            Self::decode_with_endianess::<LittleEndian>(stream)
        }
    }

    pub fn decode_with_endianess<E: ByteOrder, R: Read>(stream: &mut R) -> Result<Self, std::io::Error> {
        // Read the SRID
        let srid: u32 = stream.read_u32::<E>()?;
        
        // Read MBR
        let mbr_min_x = stream.read_f64::<E>()?;
        let mbr_min_y = stream.read_f64::<E>()?;
        let mbr_max_x = stream.read_f64::<E>()?;
        let mbr_min_y = stream.read_f64::<E>()?;
        let mbr_end = stream.read_u8::<E>()?;
        
        // Read the geometry class
        let kind = GeometryKind::decode_spatialite::<E>(stream)?;

        let mut geometry: SpatiaLiteGeometry = match kind {
            GeometryKind::PointS => PointS::decode_spatialite::<E>(stream)?.into(),
            GeometryKind::LineStringS => LineStringS::decode_spatialite::<E>(stream)?.into(),
            GeometryKind::PolygonS => PolygonS::decode_spatialite::<E>(stream)?.into(),
            GeometryKind::MultiPointS => MultiPointS::decode_spatialite(stream)?.into(),
            GeometryKind::MultiLineStringS => MultiLineStringS::decode_spatialite::<E>(stream)?.into(),
            GeometryKind::MultiPolygonS => MultiPolygonS::decode_spatialite::<E>(stream)?.into(),
            GeometryKind::GeometryCollectionS => todo!(),
            GeometryKind::PointZ => PointZ::decode_spatialite::<E>(stream)?.into(),
            GeometryKind::LineStringZ => LineStringZ::decode_spatialite::<E>(stream)?.into(),
            GeometryKind::PolygonZ => PolygonZ::decode_spatialite::<E>(stream)?.into(),
            GeometryKind::MultiPointZ => MultiPointZ::decode_spatialite(stream)?.into(),
            GeometryKind::MultiLineStringZ => MultiLineStringZ::decode_spatialite(stream)?.into(),
            GeometryKind::MultiPolygonZ => MultiPolygonZ::decode_spatialite::<E>(stream)?.into(),
            GeometryKind::GeometryCollectionZ => todo!(),
        };

        geometry.set_srid(srid);

        Ok(geometry)
    }
}

impl GeometryKind {
    pub(self) fn decode_spatialite<E: ByteOrder, R: Read>(stream: &mut R) -> Result<Self, std::io::Error> {
        Ok(match stream.read_u32::<E>()? {
            1 => GeometryKind::PointS,
            2 => GeometryKind::LineStringS,
            3 => GeometryKind::PolygonS,
            4 => GeometryKind::MultiPointS,
            5 => GeometryKind::MultiLineStringS,
            6 => GeometryKind::MultiPolygonS,
            7 => GeometryKind::GeometryCollectionS,

            1001 => GeometryKind::PointZ,
            1002 => GeometryKind::LineStringZ,
            1003 => GeometryKind::PolygonZ,
            1004 => GeometryKind::MultiPointZ,
            1005 => GeometryKind::MultiLineStringZ,
            1006 => GeometryKind::MultiPolygonZ,
            1007 => GeometryKind::MultiLineStringZ,

            _ => panic!("unknown WKB geometry")
        }.into())
    }
}

impl<const N: usize> Point<N, f64> {
    /// Encode un point dans un flux binaire.
    pub(self) fn encode_spatialite<E: ByteOrder, W: Write>(self, stream: &mut W) -> Result<(), std::io::Error> {
        self.coordinates.encode_spatialite(stream)
    }

    /// Décode un point depuis un flux binaire
    pub(self) fn decode_spatialite<E: ByteOrder, R: Read>(value: &mut R) -> Result<Self, std::io::Error> {
        let coordinates = Vector::<N, f64>::decode_spatialite::<E>(value)?;
        Ok(Self::new(coordinates))
    }
}

impl<const N: usize> MultiPoint<N, f64> {
    /// Encode un ensemble de points dans un flux binaire
    pub(self) fn encode_spatialite<E: ByteOrder, W: Write>(self, stream: &mut W) -> Result<(), std::io::Error> {
        self.coordinates.encode_spatialite(stream)
    }

    /// Décode un point depuis un flux binaire
    pub(self) fn decode_spatialite<ENDIAN: ByteOrder, R: Read>(value: &mut R) -> Result<Self, std::io::Error> {
        let coordinates = VectorArray::<N, f64>::decode_spatialite::<E>(value)?;
        Ok(Self::new(coordinates))
    }
}

impl<const N: usize> LineString<N, f64> {
    /// Encode un ensemble de points dans un flux binaire
    pub(self) fn encode_spatialite<E: ByteOrder, W: Write>(self, stream: &mut W) -> Result<(), std::io::Error> {
        self.coordinates.encode_spatialite(stream)
    }

    /// Décode un point depuis un flux binaire
    pub(self) fn decode_spatialite<ENDIAN: ByteOrder, R: Read>(value: &mut R) -> Result<Self, std::io::Error> {
        let coordinates = VectorArray::<N, f64>::decode_spatialite::<E>(value)?;
        Ok(Self::new(coordinates))
    }
}

impl<const N: usize> MultiLineString<N, f64> {

    pub(self) fn encode_spatialite<E: ByteOrder, W: Write>(self, stream: &mut W) -> Result<(), std::io::Error> {
        self.coordinates.encode_spatialite(stream)
    }

    pub(self) fn decode_spatialite<E: ByteOrder, R: Read>(value: &mut R) -> Result<Self, std::io::Error> {
        let coordinates = VectorMatrix::<N, f64>::decode_spatialite::<E>(value)?;
        Ok(Self::new(coordinates))
    }
}

impl<const N: usize> Polygon<N, f64> {
    pub(self) fn encode_spatialite<E: ByteOrder, W: Write>(self, stream: &mut W) -> Result<(), std::io::Error> {
        self.coordinates.encode_spatialite(stream)
    }

    pub(self) fn decode_spatialite<ENDIAN: ByteOrder, R: Read>(value: &mut R) -> Result<Self, std::io::Error> {
        let coordinates = VectorMatrix::<N, f64>::decode_spatialite(value)?;
        Ok(Self::new_with_srid(coordinates, srid))
    }
}

impl<const N: usize> MultiPolygon<N, f64> {
    pub(self) fn encode_spatialite<E: ByteOrder, W: Write>(self, stream: &mut W) -> Result<(), std::io::Error> {
        self.coordinates.encode_spatialite::<E>(stream)
    }
    pub(self) fn decode_ewkb<E: ByteOrder, R: Read>(value: &mut R) -> Result<Self, std::io::Error> {
        let coordinates = VectorTensor::<N, f64>::decode_spatialite::<E>(value)?;
        Ok(Self::new_with_srid(coordinates, srid))
    }
}

impl<const N: usize> Vector<N, f64> {
    /// Encode un vecteur N-D
    pub(self) fn encode_spatialite<E: ByteOrder, W: Write>(self, stream: &mut W) -> Result<Self, std::io::Error> {
        for i in 0..N {
            stream.write_f64::<E>(self.coordinates[i])?;
        }

        Ok(())
    }

    /// Décode un vecteur n-D
    pub(self) fn decode_spatialite<E: ByteOrder, R: Read>(value: &mut R) -> Result<Self, std::io::Error> {
        let mut coordinates: [f64; N] = [0; N];

        for i in 0..N {
            coordinates[i] = value.read_f64::<E>()?;
        }

        Ok(Self::new(coordinates))
    }
}

impl<const N: usize> VectorArray<N, f64> {
    /// Encode une liste de vecteurs dans le flux de sortie.
    pub(self) fn encode_spatialite<E: ByteOrder, W: Write>(self, stream: &mut W) -> Result<(), std::io::Error> {
        // Write number of points.
        stream.write_u32::<E>(self.len() as u32)?;

        for vector in self.into_iter() {
            vector.encode_spatialite::<E>(stream)?;
        }

        Ok(())
    }

    /// Décode une liste de vecteurs depuis un flux d'entrée.
    pub(self) fn decode_spatialite<E: ByteOrder, R: Read>(value: &mut R) -> Result<Self, std::io::Error> {
        let nb_points: u32 = value.read_u32::<E>()?;
        let mut coordinates = Vec::<Vector<N, f32>>::with_capacity(nb_points as usize);

        for i in 0..nb_points {
            coordinates.push(Vector::decode_spatialite::<E>(value)?);
        }

        Ok(Self::new(coordinates))
    }
}

impl<const N: usize> VectorMatrix<N, f64> {
    pub(self) fn decode_spatialite<ENDIAN: ByteOrder, R: Read>(value: &mut R) -> Result<Self, std::io::Error> {
        let nb_points: u32 = value.read_u32()?;
        let mut coordinates = Vec::<VectorArray<N, f32>>::with_capacity(nb_points as usize);

        for i in 0..nb_points {
            coordinates.push(VectorArray::decode_spatialite::<E>(value)?);
        }

        Ok(Self::new(coordinates))
    }
}

impl<const N: usize> VectorTensor<N, f64> {

    pub(self) fn decode_spatialite<E: ByteOrder, R: Read>(value: &mut R) -> Result<Self, std::io::Error> {
        let nb_points: u32 = value.read_u32()?;
        let mut coordinates = Vec::<VectorMatrix<N, f32>>::with_capacity(nb_points as usize);

        for i in 0..nb_points {
            coordinates.push(VectorMatrix::decode_spatialite::<E>(value)?);
        }

        Ok(Self::new(coordinates))
    }
}