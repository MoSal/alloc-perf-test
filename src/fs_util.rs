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

use thiserror::Error;

use std::path::PathBuf;

use crate::AllocPerfRes;

#[derive(Debug, Error)]
pub enum FsUtilError {
    #[error("failed to get a usize value from u64 value '{0}'")]
    UsizeFromU64(u64),
    #[error("'{}' exists, but it's not a file", .0.to_string_lossy())]
    ExistsButNotFile(PathBuf),
    #[error("'{}' exists, but it's not a dir", .0.to_string_lossy())]
    ExistsButNotDir(PathBuf),
    #[error("getting meta info for '{}' unexpectedly failed: {source}", path.to_string_lossy())]
    UnexpectedErrorGettingMetaInfo{
        path: PathBuf,
        source: std::io::Error,
    },
    #[error("creating new dir @ '{}' failed: {source}", path.to_string_lossy())]
    NewDirCreationFailed{
        path: PathBuf,
        source: std::io::Error,
    },
    #[error("didn't set extension of path '{}' to '{ext}'", path.to_string_lossy())]
    ExtensionNotSet{
        path: PathBuf,
        ext: &'static str,
    },
    #[error("renaming/moving '{}' to '{}' failed: {source}",
        src_path.to_string_lossy(),
        dst_path.to_string_lossy())]
    RenameFailed{
        src_path: PathBuf,
        dst_path: PathBuf,
        source: std::io::Error,
    },
    #[error("creating new writable file @ '{}' failed: {source}", path.to_string_lossy())]
    NewWritableFileCreationFailed{
        path: PathBuf,
        source: std::io::Error,
    },
    #[error("writing bytes to file @ '{}' failed: {source}", path.to_string_lossy())]
    NewWritableFileWriteFailed{
        path: PathBuf,
        source: std::io::Error,
    },
    #[error("flushing written bytes @ '{}' failed: {source}", path.to_string_lossy())]
    NewWritableFileFlushFailed{
        path: PathBuf,
        source: std::io::Error,
    },
    #[error("opening readable file @ '{}' failed: {source}", path.to_string_lossy())]
    ExistentReadableFileOpenFailed{
        path: PathBuf,
        source: std::io::Error,
    },
    #[error("reading readable file @ '{}' failed: {source}", path.to_string_lossy())]
    ExistentReadableFileReadFailed{
        path: PathBuf,
        source: std::io::Error,
    },
}

pub(super) mod util {
    use std::path::Path;
    use std::io::ErrorKind as IoErrorKind;
    use super::{FsUtilError, AllocPerfRes};

    pub(super) async fn file_exists(path: impl AsRef<Path>) -> AllocPerfRes<bool> {
        match async_fs::metadata(&path).await {
            Ok(meta) => match meta.is_file() {
                true => Ok(true),
                false => Err(FsUtilError::ExistsButNotFile(path.as_ref().to_owned()))?,
            },
            Err(e) => match e.kind() {
                IoErrorKind::NotFound => Ok(false),
                _ => Err(FsUtilError::UnexpectedErrorGettingMetaInfo{
                    path: path.as_ref().to_owned(),
                    source: e,
                })?,
            },
        }
    }

    pub(super) async fn dir_exists(path: impl AsRef<Path>) -> AllocPerfRes<bool> {
        match async_fs::metadata(&path).await {
            Ok(meta) => match meta.is_dir() {
                true => Ok(true),
                false => Err(FsUtilError::ExistsButNotDir(path.as_ref().to_owned()))?,
            },
            Err(e) => match e.kind() {
                IoErrorKind::NotFound => Ok(false),
                _ => Err(FsUtilError::UnexpectedErrorGettingMetaInfo{
                    path: path.as_ref().to_owned(),
                    source: e,
                })?,
            },
        }
    }
}


pub(crate) mod dir {
    use std::path::Path;
    use super::util;
    use super::{FsUtilError, AllocPerfRes};

    // TODO: 700 perms
    pub(crate) async fn exists_or_create(path: impl AsRef<Path>) -> AllocPerfRes<()> {
        if util::dir_exists(&path).await? {
            return Ok(())
        }

        async_fs::create_dir(&path)
            .await
            .map_err(|e| FsUtilError::NewDirCreationFailed{
                path: path.as_ref().to_owned(),
                source: e,
            })?;
        Ok(())
    }
}

pub(crate) mod file {
    use futures_lite::{AsyncReadExt, AsyncWriteExt};
    #[cfg(unix)] use async_fs::unix::MetadataExt;

    use std::path::{Path, PathBuf};
    use crate::try_util::BoolExt;
    use super::{FsUtilError, AllocPerfRes};

    pub(crate) enum UpdatedOrRolledBack {
        Updated,
        RolledBack {
            _write_error: crate::AllocPerfError,
        },
    }

    pub(crate) struct UpdatableWritableFile{
        backup_path: Option<PathBuf>,
        file: NewWritableFile,
    }

    impl UpdatableWritableFile {
        const BACKUP_EXT: &str = "backup";
        pub(crate) async fn update_or_create(path: impl AsRef<Path>) -> AllocPerfRes<Self> {
            let path = path.as_ref();
            let backup_path = match super::util::file_exists(path).await? {
                true => {
                    let mut backup_path = path.to_owned();
                    backup_path.set_extension(Self::BACKUP_EXT)
                        .err_if_not(|| FsUtilError::ExtensionNotSet {
                            path: backup_path.clone(),
                            ext: Self::BACKUP_EXT,
                        })?;
                    async_fs::rename(path, &backup_path).await
                        .map_err(|source| FsUtilError::RenameFailed{
                            src_path: path.to_owned(),
                            dst_path: backup_path.clone(),
                            source,
                        })?;

                    let path_str = path.to_string_lossy();
                    let bpath_str = backup_path.to_string_lossy();
                    tracing::info!("Backed up {path_str:?} to {bpath_str:?}");

                    Some(backup_path)
                },
                false => {
                    None
                },
            };

            let file = NewWritableFile::create(path).await?;
            Ok(Self{ backup_path, file })
        }

        pub(crate) fn updating(&self) -> bool {
            self.backup_path.is_some()
        }

        pub(crate) async fn update_or_rollback(self, bytes: &[u8]) -> AllocPerfRes<UpdatedOrRolledBack> {
            let file_path = self.file.path.clone();
            match self.file.write(bytes).await {
                Ok(()) => Ok(UpdatedOrRolledBack::Updated),
                Err(_write_error) => match self.backup_path {
                    Some(backup_path) => {
                        let fp_str = file_path.to_string_lossy();
                        let bp_str = backup_path.to_string_lossy();
                        tracing::error!("write to {fp_str:?} failed: '{_write_error}'");
                        tracing::warn!("rolling back from {bp_str:?}");

                        async_fs::rename(&backup_path, &*file_path).await
                            .map_err(|source| FsUtilError::RenameFailed{
                                src_path: backup_path,
                                dst_path: file_path,
                                source,
                            })?;
                        Ok(UpdatedOrRolledBack::RolledBack{_write_error})
                    },
                    None => Err(_write_error),
                },
            }
        }
    }

    pub(crate) struct NewWritableFile {
        path: PathBuf,
        file: async_fs::File,
    }

    impl NewWritableFile {
        // TODO: 600 perms
        pub(crate) async fn create(path: impl AsRef<Path>) -> AllocPerfRes<Self> {
            let file = async_fs::OpenOptions::new()
                .create_new(true)
                .write(true)
                .open(&path)
                .await
                .map_err(|source| FsUtilError::NewWritableFileCreationFailed{
                    path: path.as_ref().to_owned(),
                    source,
                })?;
            let path = path.as_ref().to_owned();
            Ok(Self{ path, file })
        }

        pub(crate) async fn write(mut self, bytes: &[u8]) -> AllocPerfRes<()> {
            self.file.write_all(&*bytes).await
                .map_err(|source| FsUtilError::NewWritableFileWriteFailed{
                    path: self.path.clone(),
                    source,
                })?;
            self.file.flush().await
                .map_err(|source| FsUtilError::NewWritableFileFlushFailed{
                    path: self.path,
                    source,
                })?;
            Ok(())
        }
    }

    pub(crate) struct ExistentReadableFile {
        path: PathBuf,
        file: async_fs::File,
    }

    impl ExistentReadableFile {
        pub(crate) async fn open(path: impl AsRef<Path>) -> AllocPerfRes<Self> {
            let path = path.as_ref();
            let file = async_fs::OpenOptions::new()
                .create(false)
                .read(true)
                .open(&path)
                .await
                .map_err(|source| FsUtilError::ExistentReadableFileOpenFailed{
                    path: path.to_owned(),
                    source,
                })?;
            let path = path.to_owned();
            Ok(Self{path, file})
        }

        pub(crate) async fn read(mut self) -> AllocPerfRes<Vec<u8>> {
            #[cfg(unix)]
            let sz_res = {
                let sz_u64 = self.file.metadata()
                    .await
                    .map_err(|source| FsUtilError::UnexpectedErrorGettingMetaInfo{
                        path: self.path.clone(),
                        source,
                    })?
                    .size();
                usize::try_from(sz_u64)
                    .map_err(|_| FsUtilError::UsizeFromU64(sz_u64))
            };

            #[cfg(not(unix))]
            let sz_res = Ok(1024*32); // If this is ever used in Windows, users can take this probably
                              // non-measurable performance hit.

            let mut bytes = Vec::with_capacity(sz_res?);

            self.file.read_to_end(&mut bytes)
                .await
                .map_err(|source| FsUtilError::ExistentReadableFileReadFailed{
                    path: self.path,
                    source,
                })?;

            Ok(bytes)
        }
    }
}
