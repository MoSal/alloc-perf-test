use speedy::{Context, Readable, Reader, Writable, Writer};
use thiserror::Error;

use chrono::NaiveDate;

#[derive(Debug, Error)]
enum WrapperTypesError {
    #[error("invalid NaiveDate string: read bytes are not valid utf-8")]
    InvalidNaiveDateUtf8,
    #[error("invalid NaiveDate string: '{0}' is not in '%Y-%m-%d' format")]
    InvalidNaiveDateString(String),
}

#[derive(Debug)]
pub struct NDWrapper(
    pub NaiveDate
);

impl<'a, C: Context> Readable<'a, C> for NDWrapper {
    #[inline]
    fn read_from<R: Reader<'a, C>>(reader: &mut R) -> Result< Self, C::Error> {
        let mut buf = [0u8; 10];
        let _b = reader.read_bytes(&mut buf)?;

        let s = std::str::from_utf8(&buf)
            .map_err(|_| speedy::Error::custom(WrapperTypesError::InvalidNaiveDateUtf8))?;

        let nd = NaiveDate::parse_from_str(s, "%Y-%m-%d")
            .map_err(|_| speedy::Error::custom(WrapperTypesError::InvalidNaiveDateString(s.into())))?;

        Ok(Self(nd))
    }

    #[inline]
    fn minimum_bytes_needed() -> usize {
        10
    }
}


impl<C: Context> Writable<C> for NDWrapper {
    #[inline]
    fn write_to<T: ?Sized + Writer<C>>(&self, writer: &mut T) -> Result<(), C::Error> {
        let s = self.0.format("%Y-%m-%d").to_string();
        writer.write_bytes(s.as_bytes())
    }

    #[inline]
    fn bytes_needed(&self) -> Result<usize, C::Error> {
        Ok(10)
    }
}
