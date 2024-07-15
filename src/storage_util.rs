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

use lz4_flex::frame::{FrameEncoder, FrameDecoder};
use speedy::{Readable, Writable, Endianness};
use thiserror::Error;

use std::io::{Cursor, Read, Write};
use std::path::{Path, PathBuf};

use crate::conf::SubFull;
use crate::fs_util::file::{UpdatableWritableFile, UpdatedOrRolledBack, ExistentReadableFile};

use crate::AllocPerfRes;

#[derive(Debug, Error)]
pub enum StorageUtilError {
    #[error("failed reading speedy-serialized bytes: {0}")]
    SpeedyReadFailed(#[source] speedy::Error),
    #[error("failed to speedy-serialize to bytes: {0}")]
    SpeedyWriteFailed(#[source] speedy::Error),
    #[error("failed to decompress supposedly brotli-compressed bytes: {0}")]
    Decompress(#[source] std::io::Error),
    #[error("failed to brotli-compress bytes: {0}")]
    Compress(#[source] std::io::Error),
    #[error("failed to finish lz4 compression: {0}")]
    CompressFinish(#[source] lz4_flex::frame::Error),
}

pub(crate) trait StorageInfo<const HAS_PARAM: bool = false> {
    const DESC: &'static str;

}

const ENDIANNESS: Endianness = Endianness::NATIVE;

pub(crate) trait IsSpeedyRwRd:
    Writable<Endianness> +
    for<'a> Readable<'a, Endianness> +
    Send + Sync + 'static
{
    const FILE_NAME: &'static str;
}

trait StoragePrivSpeedy: StorageInfo + IsSpeedyRwRd {
    fn serialized_from_binz_blocking(binz: Vec<u8>) -> AllocPerfRes<Vec<u8>> {
        let mut decomp = FrameDecoder::new(&*binz);
        let mut serialized = Vec::with_capacity(binz.len() * 4);
        decomp.read_to_end(&mut serialized)
            .map_err(|source| StorageUtilError::Decompress(source))?;
        Ok(serialized)
    }

    fn from_serialized_blocking(serialized: Vec<u8>) -> AllocPerfRes<Self> {
        let serialized_s = Cursor::new(serialized);

        let val = Self::read_from_stream_buffered_with_ctx(ENDIANNESS, serialized_s)
            .map_err(|source| StorageUtilError::SpeedyReadFailed(source))?;

        Ok(val)
    }

    async fn from_binz(binz: Vec<u8>) -> AllocPerfRes<Self> {
        blocking::unblock(move || {
            let serialized = Self::serialized_from_binz_blocking(binz)?;
            Self::from_serialized_blocking(serialized)
        }).await
    }

    fn to_serialized_blocking(&self) -> AllocPerfRes<Vec<u8>> {
        let bytes = self.write_to_vec_with_ctx(ENDIANNESS)
            .map_err(|source| StorageUtilError::SpeedyWriteFailed(source))?;
        Ok(bytes)
    }

    async fn with_binz(self) -> AllocPerfRes<(Self, Vec<u8>)> {
        blocking::unblock(move || {
            let serialized = self.to_serialized_blocking()?;
            let mut binz = Vec::with_capacity(serialized.len() / 2);
            let mut comp = FrameEncoder::new(&mut binz);
            comp.write_all(&*serialized)
                .map_err(|source| StorageUtilError::Compress(source))?;
            comp.flush()
                .map_err(|source| StorageUtilError::Compress(source))?;
            comp.finish()
                .map_err(|source| StorageUtilError::CompressFinish(source))?;
            Ok((self, binz))
        }).await
    }
}

pub(crate) trait StorageOpsSpeedy: StorageInfo + IsSpeedyRwRd {
    async fn get_path(sub: &SubFull) -> AllocPerfRes<PathBuf> {
        let sub_dir_path = sub.mk_sub_dir_path().await?;
        Ok(sub_dir_path.join(Self::FILE_NAME))
    }

    async fn from_binz_file(path: impl AsRef<Path>) -> AllocPerfRes<Self> {
        tracing::debug!("getting {desc} from binz file @ {path_str:?}",
            desc=Self::DESC,
            path_str=path.as_ref().to_string_lossy());

        let binz_bytes = ExistentReadableFile::open(path)
            .await?
            .read()
            .await?;
        Self::from_binz(binz_bytes).await
    }

    async fn from_local(sub: &SubFull) -> AllocPerfRes<Self> {
        let save_path = Self::get_path(sub).await?;
        tracing::info!("get {desc} of account {idx} from {dom} using file @ '{save_path_str}'",
            desc=Self::DESC,
            save_path_str=save_path.to_string_lossy(),
            idx=sub.idx,
            dom=sub.domain);
        Ok(Self::from_binz_file(save_path).await?)
    }

    async fn with_updated_binz_file(self, path: impl AsRef<Path>) -> AllocPerfRes<(Self, UpdatedOrRolledBack)> {
        let (self_, binz) = self.with_binz().await?;

        let updatable_f = UpdatableWritableFile::update_or_create(&path).await?;

        tracing::debug!("{} {desc} to binz file @ {path_str:?}",
            updatable_f.updating().then_some("updating").unwrap_or("saving"),
            desc=Self::DESC,
            path_str=path.as_ref().to_string_lossy());

        Ok((self_, updatable_f.update_or_rollback(&*binz).await?))
    }
}

impl<T> StoragePrivSpeedy for T where T: StorageInfo + IsSpeedyRwRd {}
impl<T> StorageOpsSpeedy for T where T: StorageInfo + IsSpeedyRwRd + StoragePrivSpeedy {}
