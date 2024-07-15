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

use speedy::{Readable, Writable};

use std::collections::BTreeMap;

use crate::wrapper_types::NDWrapper;

#[derive(Readable, Writable, Debug)]
pub(crate) enum MapOrSeq<V: 'static> {
    Map(BTreeMap<String, V>),
    Seq(Vec<V>),
}

impl<V: 'static> MapOrSeq<V> {
    pub(crate) fn values(&self) -> Box<dyn Iterator<Item=&V> + '_> {
        match self {
            Self::Map(map) => Box::new(map.values()),
            Self::Seq(seq) => Box::new(seq.iter()),
        }
    }
}

#[derive(Debug, Readable, Writable)]
pub(crate) enum YearOrYMD {
    Year(u64),
    YMD(NDWrapper),
}
