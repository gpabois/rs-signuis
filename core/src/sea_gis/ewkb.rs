use byteorder::{BigEndian, ByteOrder, LittleEndian, NativeEndian, ReadBytesExt, WriteBytesExt};
use sea_orm::{TryGetable, Value};
use sea_query::ValueType;
use std::{
    convert::Infallible, default, io::{Cursor, Write}, ops::Deref
};

use super::{
    Geometry, GeometryKind, LineString, LineStringS, LineStringZ, MultiLineString, MultiLineStringS, MultiLineStringZ, MultiPoint, MultiPointS, MultiPointZ, MultiPolygon, MultiPolygonS, MultiPolygonZ, Point, PointS, Polygon, PolygonS, PolygonZ, Vector, VectorArray, VectorMatrix, VectorTensor
};

/// Objet intermédiaire pour encoder/decoder 
/// au format EWKB toute géométrie.
pub struct EWKBGeometry(Geometry);

impl From<EWKBGeometry> for Value {
    fn from(value: EWKBGeometry) -> Self {
        let mut buf = Vec::<u8>::default();
        value.encode_ewkb(&mut buf).expect("cannot encode EWKB geometry");
        buf.into()
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
            Value::Bytes(Some(buf)) => EWKBGeometry::decode_ewkb(&mut buf).map_err(|| sea_query::ValueTypeErr),
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

impl From<EWKBGeometry> for Geometry {
    fn from(value: EWKBGeometry) -> Self {
        value.0
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

impl DerefMut for EWKBGeometry {
    fn deref_mut(&self) -> &Self::Target {
        &mut self.0
    }
}

impl GeometryKind {
    pub(self) fn encode_ewkb<E: ByteOrder, W: Write>(self, stream: &mut W) -> Result<(), std::io::Error> {
        let encoded = match value.0 {
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
        };

        stream.write_u32::<E>(encoded)
    }

    pub(self) fn decode_ewkb<E: ByteOrder, R: Read>(stream: &mut R) -> Result<Self, std::io::Error> {
        let kind: EWKBGeometryKind = match stream.read_u32::<E>()? {
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
        };

        Ok(kind)
    }
}

const BIG_ENDIAN: u8 = 0;
const LITTLE_ENDIAN: u8 = 1;

impl TryFrom<&[u8]> for EWKBGeometry {
    type Error = std::io::Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let mut buf = Cursor::new(value);
        Self::decode_ewkb(&mut buf)
    }
}
impl EWKBGeometry {
    /// Encode une géométrie au format EWKB dans le flux de sortie.
    ///
    /// Utilise par défaut le boutisme natif.
    pub fn encode<W: Write>(self, stream: &mut W) -> Result<(), std::io::Error> {
        self.encode_ewkb_with_endianess::<NativeEndian>(stream)
    }

    /// Encode une géométrie au format EWKB dans le flux de sortie, avec un boutisme défini.
    pub fn encode_with_endianess<E: ByteOrder, W: Write>(self, stream: &mut W) -> Result<(), std::io::Error> {
        // Write endianness.
        buf.write_u8(Endianess::from(E).into())?;

        // Write the EWKB type
        self.kind().encode_ewkb(stream)?;
    
        /// Write the SRID
        stream.write_u32::<E>(self.srid())?;

        match self.0 {
            Geometry::PointS(a) => a.encode_ewkb::<E>(stream),
            Geometry::LineStringS(a) => a.encode_ewkb::<E>(stream),
            Geometry::PolygonS(a) => a.encode_ewkb::<E>(stream),
            Geometry::MultiPointS(a) => a.encode_ewkb::<E>(stream),
            Geometry::MultiLineStringS(a) => a.encode_ewkb::<E>(stream),
            Geometry::MultiPolygonS(a) => a.encode_ewkb::<E>(stream),
            Geometry::PointZ(a) => a.encode_ewkb::<E>(stream),
            Geometry::LineStringZ(a) => a.encode_ewkb::<E>(stream),
            Geometry::PolygonZ(a) => a.encode_ewkb::<E>(stream),
            Geometry::MultiPointZ(a) => a.encode_ewkb::<E>(stream),
            Geometry::MultiLineStringZ(a) => a.encode_ewkb::<E>(stream),
            Geometry::MultiPolygonZ(a) => a.encode_ewkb::<E>(stream),
        }
    }

    /// Décode une géométrie encodée en EWKB
    pub fn decode<R: Read>(buf: &mut R) -> Result<Self, std::io::Error> {
        // 0: Big Endian, 1: Little Endian
        let endianess = buf.read_u8()?;

        if endianess == BIG_ENDIAN {
            Self::decode_ewkb_with_endianess::<BigEndian>(buf)
        } else {
            Self::decode_ewkb_with_endianess::<LittleEndian>(buf)
        }
    }

    /// Décode une géométrie avec un boutisme défini
    pub fn decode_with_endianess<E: ByteOrder, R: Read>(stream: &mut R,) -> Result<Self, std::io::Error> {
        let kind: GeometryKind = GeometryKind.decode_ewkb::<E>(stream)?;
        let srid: u32 = value.read_u32()?;

        let mut geometry = match kind {
            GeometryKind::PointS => PointS::decode_ewkb::<E>(stream)?.into(),
            GeometryKind::LineStringS => LineStringS::decode_ewkb::<E>(stream)?.into(),
            GeometryKind::PolygonS => PolygonS::decode_ewkb::<E>(stream)?.into(),
            GeometryKind::MultiPointS => MultiPointS::decode_ewkb::<E>(stream)?.into(),
            GeometryKind::MultiLineStringS => MultiLineStringS::decode_ewkb::<E>(stream)?.into(),
            GeometryKind::MultiPolygonS => MultiPolygonS::decode_ewkb::<E>(stream)?.into(),
            GeometryKind::GeometryCollectionS => GeometryCollectionS::decode::<E>(stream)?.into(),
            GeometryKind::PointZ => PointZ::decode::<E>(stream)?.into(),
            GeometryKind::LineStringZ => LineStringZ::decode_ewkb::<E>(stream)?.into(),
            GeometryKind::PolygonZ => PolygonZ::decode_ewkb::<E>(stream)?.into(),
            GeometryKind::MultiPointZ => MultiPointZ::decode_ewkb::<E>(stream)?.into(),
            GeometryKind::MultiLineStringZ => MultiPointZ::decode_ewkb::<E>(stream)?.into(),
            GeometryKind::MultiPolygonZ => MultiPolygonZ::decode_ewkb::<E>(stream)?.into(),
            GeometryKind::GeometryCollectionZ => GeometryCollectionZ::decode::<E>(stream)?.into(),
        };

        geometry.set_srid(srid);

        Ok(geometry)
    }
}

impl<const N: usize> Point<N, f64> {
    /// Encode un point dans un flux binaire.
    pub(self) fn encode_ewkb<E: ByteOrder, W: Write>(self, stream: &mut W) -> Result<(), std::io::Error> {
        self.coordinates.encode_ewkb::<E>(stream)
    }

    /// Décode un point depuis un flux binaire
    pub(self) fn decode_ewkb<E: ByteOrder, R: Read>(value: &mut R) -> Result<Self, std::io::Error> {
        let coordinates = Vector::<N, f64>::decode_ewkb::<E>(value)?;
        Ok(Self::new(coordinates))
    }
}

impl<const N: usize> MultiPoint<N, f64> {
    /// Encode un ensemble de points dans un flux binaire
    pub(self) fn encode_ewkb<E: ByteOrder, W: Write>(self, stream: &mut W) -> Result<(), std::io::Error> {
        self.coordinates.encode_ewkb::<E>(stream)
    }

    /// Décode un point depuis un flux binaire
    pub(self) fn decode_ewkb<E: ByteOrder, R: Read>(value: &mut R) -> Result<Self, std::io::Error> {
        let coordinates = VectorArray::<N, f64>::decode_ewkb::<E>(value)?;
        Ok(Self::new(coordinates))
    }
}

impl<const N: usize> LineString<N, f64> {
    /// Encode un ensemble de points dans un flux binaire
    pub(self) fn encode_ewkb<E: ByteOrder, W: Write>(self, stream: &mut W) -> Result<(), std::io::Error> {
        self.coordinates.encode_ewkb::<E>(stream)
    }

    /// Décode un point depuis un flux binaire
    pub(self) fn decode_ewkb<E: ByteOrder, R: Read>(value: &mut R) -> Result<Self, std::io::Error> {
        let coordinates = VectorArray::<N, f64>::decode_ewkb::<E>(value)?;
        Ok(Self::new(coordinates))
    }
}

impl<const N: usize> MultiLineString<N, f64> {
    /// Encode dans un flux binaire
    pub(self) fn encode_ewkb<E: ByteOrder, W: Write>(self, stream: &mut W) -> Result<(), std::io::Error> {
        self.coordinates.encode_ewkb::<E>(stream)
    }

    /// Décode depuis un flux binaire
    pub(self) fn decode_ewkb<ENDIAN: ByteOrder, R: Read>(
        value: &mut R,
    ) -> Result<Self, std::io::Error> {
        let coordinates = VectorMatrix::<N, f64>::decode_ewkb::<E>(value)?;
        Ok(Self::new(coordinates))
    }
}

impl<const N: usize> Polygon<N, f64> {
    pub(self) fn encode_ewkb<E: ByteOrder, W: Write>(self, stream: &mut W) -> Result<(), std::io::Error> {
        self.coordinates.encode_ewkb::<E>(stream)
    }

    pub(self) fn decode_ewkb<E: ByteOrder, R: Read>(value: &mut R) -> Result<Self, std::io::Error> {
        let coordinates = VectorMatrix::<N, f64>::decode_ewkb::<E>(value)?;
        Ok(Self::new(coordinates))
    }
}

impl<const N: usize> MultiPolygon<N, f64> {
    pub(self) fn encode_ewkb<E: ByteOrder, W: Write>(self, stream: &mut W) -> Result<(), std::io::Error> {
        self.coordinates.encode_ewkb::<E>(stream)
    }
    pub(self) fn decode_ewkb<E: ByteOrder, R: Read>(value: &mut R) -> Result<Self, std::io::Error> {
        let coordinates = VectorTensor::<N, f64>::decode_ewkb::<E>(value)?;
        Ok(Self::new(coordinates))
    }
}

impl<const N: usize> Vector<N, f64> {
    /// Encode un vecteur N-D
    pub(self) fn encode_ewkb<E: ByteOrder, W: Write>(
        self,
        stream: &mut W,
    ) -> Result<Self, std::io::Error> {
        for i in 0..N {
            stream.write_f64::<E>(self.coordinates[i])?;
        }

        Ok(())
    }

    /// Décode un vecteur n-D
    pub(self) fn decode_ewkb<E: ByteOrder, R: Read>(value: &mut R) -> Result<Self, std::io::Error> {
        let mut coordinates: [f64; N] = [0; N];

        for i in 0..N {
            coordinates[i] = value.read_f64::<E>()?;
        }

        Ok(Self::new(coordinates))
    }
}

impl<const N: usize> VectorArray<N, f64> {
    /// Encode une liste de vecteurs dans le flux de sortie.
    pub(self) fn encode_ewkb<E: ByteOrder, W: Write>(
        self,
        stream: &mut W,
    ) -> Result<(), std::io::Error> {
        // Write number of points.
        stream.write_u32::<E>(self.len() as u32)?;

        for vector in self.into_iter() {
            vector.encode_ewkb::<E>(stream)?;
        }

        Ok(())
    }

    /// Décode une liste de vecteurs depuis un flux d'entrée.
    pub(self) fn decode_ewkb<ENDIAN: ByteOrder, R: Read>(stream: &mut R) -> Result<Self, std::io::Error> {
        let nb_points: u32 = stream.read_u32::<ENDIAN>()?;
        let mut coordinates = Vec::<Vector<N, f32>>::with_capacity(nb_points as usize);

        for i in 0..nb_points {
            coordinates.push(Vector::decode_ewkb::<ENDIAN>(stream)?);
        }

        Ok(Self::new(coordinates))
    }
}

impl<const N: usize> VectorMatrix<N, f64> {
    /// Décode un vecteur n-D
    pub(self) fn decode_ewkb<E: ByteOrder, R: Read>(stream: &mut R) -> Result<Self, std::io::Error> {
        let nb_points: u32 = stream.read_u32::<E>()?;
        let mut coordinates = Vec::<VectorArray<N, f32>>::with_capacity(nb_points as usize);

        for i in 0..nb_points {
            coordinates.push(VectorArray::decode_ewkb::<E>(stream)?);
        }

        Ok(Self::new(coordinates))
    }
}

impl<const N: usize> VectorTensor<N, f64> {
    /// Décode un vecteur n-D
    pub(self) fn decode_ewkb<E: ByteOrder, R: Read>(stream: &mut R) -> Result<Self, std::io::Error> {
        let nb_points: u32 = stream.read_u32::<E>()?;
        let mut coordinates = Vec::<VectorMatrix<N, f32>>::with_capacity(nb_points as usize);

        for i in 0..nb_points {
            coordinates.push(VectorMatrix::decode_ewkb::<E>(stream)?);
        }

        Ok(Self::new(coordinates))
    }
}

impl GeometryKind {
    pub(self) fn encode_ewkb<E: ByteOrder, W: Write>(
        self,
        stream: &mut W,
    ) -> Result<(), std::io::Error> {
        let kind: EWKBGeometryKind = self.into();
        stream.write_u8::<E>(kind.into())
    }

    pub(self) fn decode_ewkb<E: ByteOrder, R: Read>(stream: &mut R) -> Result<Self, std::io::Error> {
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
