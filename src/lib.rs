#![feature(try_blocks)]
#![feature(let_chains)]
#![feature(iter_array_chunks)]

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
