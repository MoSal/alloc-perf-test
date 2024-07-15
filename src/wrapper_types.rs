/*
    This file is a part of alloc-perf-test.

    Copyright (C) 2024 Mohammad AlSaleh <CE.Mohammad.AlSaleh at gmail.com>
    https://github.com/MoSal

    alloc-perf-test is free software: you can redistribute it and/or modify
    it under the terms of the Affero GNU General Public License as
    published by the Free Software Foundation.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    Affero GNU General Public License for more details.

    You should have received a copy of the Affero GNU General Public License
    along with this program.  If not, see <http://www.gnu.org/licenses/>.
*/

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
