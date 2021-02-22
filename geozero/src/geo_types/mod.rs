//! [geo-types](https://github.com/georust/geo) conversions.
pub(crate) mod geo_types_reader;
pub(crate) mod geo_types_writer;

pub use geo_types_reader::*;
pub use geo_types_writer::*;

pub(crate) mod conversion {
    use super::geo_types_writer::*;
    use crate::error::Result;
    use crate::wkb::{FromWkb, WkbDialect};
    use crate::{GeozeroGeometry, GeozeroGeometryReader};
    use std::io::Read;

    /// Convert to geo-types Geometry.
    pub trait ToGeo {
        /// Convert to geo-types Geometry.
        fn to_geo(&self) -> Result<geo_types::Geometry<f64>>;
    }

    impl<T: GeozeroGeometry> ToGeo for T {
        fn to_geo(&self) -> Result<geo_types::Geometry<f64>> {
            let mut geo = GeoWriter::new();
            self.process_geom(&mut geo)?;
            Ok(geo.geom)
        }
    }

    /// Read as geo-types Geometry.
    pub trait ReadAsGeo {
        /// Read as geo-types Geometry.
        fn read_as_geo<R: Read>(reader: R) -> Result<geo_types::Geometry<f64>>;
    }

    impl<T: GeozeroGeometryReader> ReadAsGeo for T {
        fn read_as_geo<R: Read>(reader: R) -> Result<geo_types::Geometry<f64>> {
            let mut geo = GeoWriter::new();
            T::read_geom(reader, &mut geo)?;
            Ok(geo.geom)
        }
    }

    impl FromWkb for geo_types::Geometry<f64> {
        fn from_wkb<R: Read>(rdr: &mut R, dialect: WkbDialect) -> Result<Self> {
            let mut geo = GeoWriter::new();
            crate::wkb::process_wkb_type_geom(rdr, &mut geo, dialect)?;
            Ok(geo.geom)
        }
    }
}