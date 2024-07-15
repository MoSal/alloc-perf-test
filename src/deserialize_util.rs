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
