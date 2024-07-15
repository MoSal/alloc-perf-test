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
use std::io::ErrorKind;

use crate::fs_util::FsUtilError;
use crate::conf::SubFull;
use crate::booies::BooiesDetails;
use crate::storage_util::{IsSpeedyRwRd, StorageInfo, StorageOpsSpeedy};

use crate::{AllocPerfRes, AllocPerfError};

#[derive(Readable, Writable)]
struct BooiesDetailsCacheItem {
    fetched_at: i64,
    boo_details: BooiesDetails,
}

#[derive(Readable, Writable)]
pub(crate) struct BooiesDetailsCache {
    inner: BTreeMap<u64, BooiesDetailsCacheItem>,
}

impl StorageInfo for BooiesDetailsCache {
    const DESC: &'static str = "booies details cache";
}

impl IsSpeedyRwRd for BooiesDetailsCache {
    const FILE_NAME: &'static str = "BOOIES_CACHE";
}

impl BooiesDetailsCache {
    pub(crate) fn new() -> Self {
        Self { inner: BTreeMap::new() }
    }

    pub(crate) fn insert(&mut self, num: u64, boo_details: BooiesDetails) {
        let cache_item = BooiesDetailsCacheItem{
            fetched_at: chrono::Utc::now().timestamp(),
            boo_details,
        };
        let _ = self.inner.insert(num, cache_item);
    }

    pub(crate) fn get_boo_details(&self, num: u64) -> Option<&BooiesDetails> {
        self.inner.get(&num).map(|item| &item.boo_details)
    }

    pub(crate) async fn get_local_or_new(sub: &SubFull) -> AllocPerfRes<Self> {
        let desc = Self::DESC;
        let sub_idx = sub.idx;

        match Self::from_local(sub).await {
            Ok(v) => Ok(v),
            Err(AllocPerfError::FsUtil(FsUtilError::ExistentReadableFileOpenFailed { path: _, source })) if source.kind() == ErrorKind::NotFound => {
                tracing::info!("{desc}: does not exist for sub {sub_idx} locally, start a new one..");
                Ok(Self::new())
            },
            Err(e) => {
                tracing::error!("{desc}: unexpected local failure for sub {sub_idx}: {e}");
                Err(e)
            },
        }
    }
}
