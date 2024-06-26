use byteorder::{BigEndian, ByteOrder, LittleEndian, NativeEndian, ReadBytesExt, WriteBytesExt};
use std::{
    convert::Infallible,
    io::{Cursor, Write},
    ops::Deref,
};

use super::{
    Geometry, GeometryKind, LineString, LineStringS, LineStringZ, MultiLineString,
    MultiLineStringS, MultiLineStringZ, MultiPoint, MultiPointS, MultiPointZ, MultiPolygon,
    MultiPolygonS, MultiPolygonZ, Point, Polygon, PolygonS, PolygonZ, Vector, VectorArray,
    VectorMatrix, VectorTensor,
};

/// Objet intermédiaire pour encoder/decoder au format EWKB toute géométrie.
pub struct EWKBGeometry(Geometry);

impl EWKBGeometry {
    pub fn kind(&self) -> EWKBGeometryKind {
        EWKBGeometryKind(self.0.kind())
    }
}

impl<T> From<T> for EWKBGeometry
where
    Geometry: From<T>,
{
    fn from(value: T) -> Self {
        Self(value.into())
    }
}

impl Deref for EWKBGeometry {
    type Target = Geometry;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Copy, Clone)]
pub struct EWKBGeometryKind(GeometryKind);

impl From<GeometryKind> for EWKBGeometryKind {
    fn from(value: GeometryKind) -> Self {
        Self(value)
    }
}

impl From<EWKBGeometryKind> for GeometryKind {
    fn from(value: EWKBGeometryKind) -> Self {
        value.0
    }
}

impl From<EWKBGeometryKind> for u32 {
    fn from(value: EWKBGeometryKind) -> Self {
        match value.0 {
            GeometryKind::PointS => 1,
            GeometryKind::LineStringS => 2,
            GeometryKind::PolygonS => 3,
            GeometryKind::MultiPointS => 4,
            GeometryKind::MultiLineStringS => 5,
            GeometryKind::MultiPolygonS => 6,
            GeometryKind::GeometryCollectionS => 7,

            GeometryKind::PointZ => 0x80000001,
            GeometryKind::LineStringZ => 0x80000002,
            GeometryKind::PolygonZ => 0x80000003,
            GeometryKind::MultiPointZ => 0x80000004,
            GeometryKind::MultiLineStringZ => 0x80000005,
            GeometryKind::MultiPolygonZ => 0x80000006,
            GeometryKind::GeometryCollectionZ => 0x80000007,
        }
    }
}

impl TryFrom<u32> for EWKBGeometryKind {
    type Error = Infallible;

    fn try_from(mut value: u32) -> Result<Self, Self::Error> {
        let with_z = if value > 0x8000000 {
            value -= 0x8000000;
            true
        } else {
            false
        };

        let kind: EWKBGeometryKind = match value {
            1 => GeometryKind::PointS,
            2 => GeometryKind::LineStringS,
            3 => GeometryKind::PolygonS,
            4 => GeometryKind::MultiPointS,
            5 => GeometryKind::MultiLineStringS,
            6 => GeometryKind::MultiPolygonS,
            7 => GeometryKind::GeometryCollectionS,

            0x80000001 => GeometryKind::PointZ,
            0x80000002 => GeometryKind::LineStringZ,
            0x80000003 => GeometryKind::PolygonZ,
            0x80000004 => GeometryKind::MultiPointZ,
            0x80000005 => GeometryKind::MultiLineStringZ,
            0x80000006 => GeometryKind::MultiPolygonZ,
            0x80000007 => GeometryKind::GeometryCollectionZ,

            _ => panic!("invalid EWKB geometry"),
        }
        .into();

        Ok(kind)
    }
}

const BIG_ENDIAN: u8 = 0;
const LITTLE_ENDIAN: u8 = 1;

impl TryFrom<&[u8]> for EWKBGeometry {
    type Error = std::io::Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let mut buf = Cursor::new(value);
        Self::try_decode(&mut buf)
    }
}
impl EWKBGeometry {
    /// Encode une géométrie au format EWKB dans le flux de sortie.
    ///
    /// Utilise par défaut le boutisme natif.
    pub(self) fn encode<W: Write>(buf: &mut W) -> Result<(), std::io::Error> {
        self.encode_with_endianess::<NativeEndian, _>(buf)
    }

    /// Encode une géométrie au format EWKB dans le flux de sortie, avec un boutisme défini.
    pub(self) fn encode_with_endianess<E: ByteOrder, W: Write>(
        &self,
        buf: &mut W,
    ) -> Result<(), std::io::Error> {
        // Write endianness.
        buf.write_u8(Endianess::from(E).into())?;

        // Write the EWKB type
        buf.write_u32::<E>(self.kind().into())?;

        Ok(())
    }

    /// Décode une géométrie encodée en EWKB
    pub(self) fn decode<R: Read>(buf: &mut R) -> Result<Self, std::io::Error> {
        // 0: Big Endian, 1: Little Endian
        let endianess = buf.read_u8()?;

        if endianess == BIG_ENDIAN {
            Self::decode_with_endianess::<BigEndian, _>(buf)
        } else {
            Self::decode_with_endianess::<LittleEndian, _>(buf)
        }
    }

    /// Décode une géométrie avec un boutisme défini
    pub(self) fn decode_with_endianess<E: ByteOrder, R: Read>(
        buf: &mut R,
    ) -> Result<Self, std::io::Error> {
        let kind = EWKBGeometryKind.try_from(buf.read_u32::<E>()?)?;
        match kind.0 {}
    }
}

impl<const N: usize> Point<N, f64> {
    /// Encode un point dans un flux binaire.
    pub(self) fn encode<E: ByteOrder, W: Write>(
        self,
        stream: &mut W,
    ) -> Result<(), std::io::Error> {
        stream.write_u32::<E>(self.srid)?;
        self.coordinates.try_encode(stream)
    }

    /// Décode un point depuis un flux binaire
    pub(self) fn decode<ENDIAN: ByteOrder, R: Read>(value: &mut R) -> Result<Self, std::io::Error> {
        let srid = value.read_u32()?;
        let coordinates = Vector::<N, f64>::try_decode::<ENDIAN, _>(value)?;
        Ok(Self::new_with_srid(coordinates, srid))
    }
}

impl<const N: usize> MultiPoint<N, f64> {
    pub(self) fn encode<E: ByteOrder, W: Write>(self, stream: &mut W)
    /// Décode un point depuis un flux binaire
    pub(self) fn decode<ENDIAN: ByteOrder, R: Read>(value: &mut R) -> Result<Self, std::io::Error> {
        let srid = value.read_u32()?;
        let coordinates = VectorArray::<N, f64>::try_decode::<ENDIAN, _>(value)?;
        Ok(Self::new_with_srid(coordinates, srid))
    }
}

impl<const N: usize> LineString<N, f64> {
    /// Décode un point depuis un flux binaire
    pub(self) fn decode<ENDIAN: ByteOrder, R: Read>(value: &mut R) -> Result<Self, std::io::Error> {
        let srid = value.read_u32()?;
        let coordinates = VectorArray::<N, f64>::try_decode(value)?;
        Ok(Self::new_with_srid(coordinates, srid))
    }
}

impl<const N: usize> MultiLineString<N, f64> {
    /// Décode un point depuis un flux binaire
    pub(self) fn try_decode<ENDIAN: ByteOrder, R: Read>(
        value: &mut R,
    ) -> Result<Self, std::io::Error> {
        let srid = value.read_u32()?;
        let coordinates = VectorMatrix::<N, f64>::try_decode(value)?;
        Ok(Self::new_with_srid(coordinates, srid))
    }
}

impl<const N: usize> Polygon<N, f64> {
    pub(self) fn decode<ENDIAN: ByteOrder, R: Read>(value: &mut R) -> Result<Self, std::io::Error> {
        let srid = value.read_u32()?;
        let coordinates = VectorMatrix::<N, f64>::try_decode(value)?;
        Ok(Self::new_with_srid(coordinates, srid))
    }
}

impl<const N: usize> MultiPolygon<N, f64> {
    pub(self) fn decode<ENDIAN: ByteOrder, R: Read>(value: &mut R) -> Result<Self, std::io::Error> {
        let srid = value.read_u32()?;
        let coordinates = VectorTensor::<N, f64>::try_decode(value)?;
        Ok(Self::new_with_srid(coordinates, srid))
    }
}

impl<const N: usize> Vector<N, f64> {
    /// Encode un vecteur N-D
    pub(self) fn encode<ENDIAN: ByteOrder, W: Write>(
        self,
        stream: &mut W,
    ) -> Result<Self, std::io::Error> {
        for i in 0..N {
            stream.write_f64::<ENDIAN>(self.coordinates[i])?;
        }

        Ok(())
    }

    /// Décode un vecteur n-D
    pub(self) fn decode<ENDIAN: ByteOrder, R: Read>(value: &mut R) -> Result<Self, std::io::Error> {
        let mut coordinates: [f64; N] = [0; N];

        for i in 0..N {
            coordinates[i] = value.read_f64::<ENDIAN>()?;
        }

        Ok(Self::new(coordinates))
    }
}

impl<const N: usize> VectorArray<N, f64> {
    /// Encode une liste de vecteurs dans le flux de sortie.
    pub(self) fn encode<E: ByteOrder, W: Write>(
        self,
        stream: &mut W,
    ) -> Result<(), std::io::Error> {
        // Write number of points.
        stream.write_u32::<E>(self.len() as u32)?;

        for v in self.into_iter() {
            v.encode::<E, _>(stream)?;
        }

        Ok(())
    }

    /// Décode une liste de vecteurs depuis un flux d'entrée.
    pub(self) fn decode<ENDIAN: ByteOrder, R: Read>(value: &mut R) -> Result<Self, std::io::Error> {
        let nb_points: u32 = value.read_u32::<ENDIAN>()?;
        let mut coordinates = Vec::<Vector<N, f32>>::with_capacity(nb_points as usize);

        for i in 0..nb_points {
            coordinates.push(Vector::try_decode::<ENDIAN>(value)?);
        }

        Ok(Self::new(coordinates))
    }
}

impl<const N: usize> VectorMatrix<N, f64> {
    /// Décode un vecteur n-D
    pub(self) fn decode<ENDIAN: ByteOrder, R: Read>(value: &mut R) -> Result<Self, std::io::Error> {
        let nb_points: u32 = value.read_u32()?;
        let mut coordinates = Vec::<VectorArray<N, f32>>::with_capacity(nb_points as usize);

        for i in 0..nb_points {
            coordinates.push(VectorArray::try_decode::<ENDIAN>(value)?);
        }

        Ok(Self::new(coordinates))
    }
}

impl<const N: usize> VectorTensor<N, f64> {
    /// Décode un vecteur n-D
    pub(self) fn decode<ENDIAN: ByteOrder, R: Read>(value: &mut R) -> Result<Self, std::io::Error> {
        let nb_points: u32 = value.read_u32()?;
        let mut coordinates = Vec::<VectorMatrix<N, f32>>::with_capacity(nb_points as usize);

        for i in 0..nb_points {
            coordinates.push(VectorMatrix::try_decode::<ENDIAN>(value)?);
        }

        Ok(Self::new(coordinates))
    }
}

impl GeometryKind {
    pub(self) fn encode<E: ByteOrder, W: Write>(
        self,
        stream: &mut W,
    ) -> Result<(), std::io::Error> {
        let kind: EWKBGeometryKind = self.into();
        stream.write_u8::<E>(kind.into())
    }

    pub(self) fn decode<E: ByteOrder, R: Read>(stream: &mut R) -> Result<Self, std::io::Error> {
        let kind: Self = EWKBGeometryKind::try_from(stream.read_u32::<E>()?)?.into();
        Ok(kind)
    }
}

pub enum Endianess {
    BigEndian,
    LittleEndian,
}

impl From<BigEndian> for Endianess {
    fn from(value: BigEndian) -> Self {
        Endianess::BigEndian
    }
}

impl From<LittleEndian> for Endianess {
    fn from(value: LittleEndian) -> Self {
        Endianess::LittleEndian
    }
}

impl From<Endianess> for u8 {
    fn from(value: Endianess) -> Self {
        match value {
            Endianess::BigEndian => BIG_ENDIAN,
            Endianess::LittleEndian => LITTLE_ENDIAN,
        }
    }
}
