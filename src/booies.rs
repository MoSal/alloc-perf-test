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

use crate::deserialize_util::{MapOrSeq, YearOrYMD};
use crate::storage_util::StorageInfo;

#[derive(Readable, Writable)]
pub struct BooiesList(Vec<Booies>);

impl StorageInfo for BooiesList {
    const DESC: &'static str = "a list of all booies";
}

#[derive(Readable, Writable)]
pub struct Booies {
    pub(crate) num: u64,
    pub(crate) name: String,
    pub(crate) booies_id: u64,
    pub(crate) last_modified: i64,
    pub(crate) genre: String,
    pub(crate) release_date: Option<YearOrYMD>,
    pub(crate) category_id: Option<u64>,
    pub(crate) category_ids: Option<Vec<u64>>,
    pub(crate) rating: Option<f64>,
}

#[derive(Readable, Writable, Debug)]
pub(crate) struct BooiesExampleSadioInfo {
    pub(crate) boec_name: Option<String>,
    pub(crate) bad_rate: u64,
    pub(crate) channels: u64,
}

#[derive(Readable, Writable, Debug)]
pub(crate) struct BooiesExampleFigureInfo {
    pub(crate) boec_name: Option<String>,
    pub(crate) wigth: u64,
    pub(crate) feight: u64,
}

#[derive(Readable, Writable, Debug)]
pub(crate) struct BooiesExampleInfo {
    pub(crate) duration_secs: Option<u64>,
    // HH:MM:SS
    pub(crate) duration: Option<String>,
    pub(crate) bitrate: Option<u64>,
    pub(crate) figure: Option<BooiesExampleFigureInfo>,
    pub(crate) sadio: Option<BooiesExampleSadioInfo>,
}

#[derive(Readable, Writable, Debug)]
pub(crate) struct BooiesExample {
    pub(crate) id: u64,
    pub(crate) chapter: Option<u64>,
    pub(crate) example_num: Option<u64>,
    pub(crate) title: String,
    pub(crate) container_extension: String,
    pub(crate) added: i64,
    pub(crate) info: Option<BooiesExampleInfo>,
}

#[derive(Readable, Writable, Debug)]
pub(crate) struct BooiesDetails {
    pub(crate) examples: Option<MapOrSeq<Vec<BooiesExample>>>,
}

impl StorageInfo<true> for BooiesDetails {
    const DESC: &'static str = "booies #BOOIES_ID# detailed info";
}
