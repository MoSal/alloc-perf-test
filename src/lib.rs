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

#![feature(try_blocks)]
#![feature(let_chains)]
#![feature(iter_array_chunks)]
#![feature(lazy_cell)] // for bootstrapping from stable

mod try_util;
mod deserialize_util;
mod wrapper_types;
mod fs_util;
mod storage_util;
mod spawn_util;
mod conf;
mod category;
mod booies;
mod all;
mod booies_cache;
pub mod cli;

use std::ops::RangeInclusive;
use thiserror::Error;

pub(crate) const  SPAWN_CHUNK_SZ: usize = 8;

pub type AllocPerfRes<T> = Result<T, AllocPerfError>;

#[derive(Debug, Error)]
pub enum AllocPerfError {
    #[error("fs_util error: {0}")]
    FsUtil(#[from] crate::fs_util::FsUtilError),
    #[error("storage_util error: {0}")]
    StorageUtil(#[from] crate::storage_util::StorageUtilError),
    #[error("multiple errors:\n {}", .0.iter().map(|e| format!("{e}")).collect::<Vec<_>>().join("\n "))]
    Multi(Vec<Self>),
}

pub fn rand_str(len_range: RangeInclusive<usize>) -> String {
    {0..fastrand::usize(len_range)}
        .map(|_| fastrand::alphanumeric())
        .collect()
}
