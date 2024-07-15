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

use async_global_executor as a_exec;
use std::future::Future;

async fn chunk_run<const CHUNK_SZ: usize, const RETRIES: usize, T, I, R, E, Fu, F>(chunk_iter: I, f: F) -> Result<Vec<R>, (Vec<R>, Vec<crate::AllocPerfError>)>
    where I: Iterator<Item = T>,
          T: Send + Clone + std::fmt::Debug + 'static,
          Fu: Future<Output = Result<R, E>> + Send + 'static,
          F: Fn(T) -> Fu,
          R: Send + 'static,
          E: Into<crate::AllocPerfError> + Send + std::fmt::Display + 'static,
{
    let mut chunk_ret_vec = Vec::with_capacity(CHUNK_SZ);
    let mut errors = Vec::with_capacity((CHUNK_SZ/2).max(1));

    let tasks_results = chunk_iter
        .map(|t| (t.clone(), a_exec::spawn(f(t))))
        // collect() is important to consume the iterator
        .collect::<Vec<_>>();
    for (t, task) in tasks_results {
        if RETRIES > 0 {
            match task.await {
                Ok(v) => chunk_ret_vec.push(v),
                Err(mut e) => {
                    let tries = RETRIES + 1;
                    let mut next_try = 2;
                    'retry: loop {
                        if next_try > tries {
                            errors.push(e.into());
                            break 'retry;
                        }
                        tracing::warn!("try {}/{tries} failed with error: {e}", next_try - 1);
                        tracing::warn!("start try {next_try}/{tries} task for item {t:?}");
                        match a_exec::spawn(f(t.clone())).await {
                            Ok(v) => {
                                chunk_ret_vec.push(v);
                                break 'retry;
                            },
                            Err(retry_e) => {
                                e = retry_e;
                                next_try += 1;
                            },
                        }
                    }
                },
            }
        } else {
            match  task.await {
                Ok(v) => chunk_ret_vec.push(v),
                Err(e) => {
                    errors.push(e.into());
                },
            }
        }
    }

    if errors.is_empty() {
        Ok(chunk_ret_vec)
    } else {
        Err((chunk_ret_vec, errors))
    }
}

pub(crate) async fn chunked_spawn_runner_with_retries<
    const CHUNK_SZ: usize,
    const RETRIES: usize,
    const ALLOW_ERR: bool,
    T, I, R, E, Fu, F>(full_iter: I, f: F) -> Result<Vec<R>, (Vec<R>, Vec<crate::AllocPerfError>)>
    where I: Iterator<Item = T>,
          T: Send + Clone + std::fmt::Debug + 'static,
          Fu: Future<Output = Result<R, E>> + Send + 'static,
          F: Fn(T) -> Fu + Copy,
          R: Send + 'static,
          E: Into<crate::AllocPerfError> + Send + std::fmt::Display + 'static,
{
    let mut ret_vec = Vec::with_capacity(full_iter.size_hint().1.unwrap_or(CHUNK_SZ*2).max(1));
    let mut chunks = full_iter.array_chunks::<CHUNK_SZ>();
    let mut errors = Vec::with_capacity(CHUNK_SZ*2);

    let chunk_run = chunk_run::<CHUNK_SZ, RETRIES, _, _, _, _, _, _>;

    macro_rules! get_chunk {
        ($chunk_iter:expr) => {
            match chunk_run($chunk_iter, f).await {
                Ok(chunk_ret_vec) => ret_vec.extend(chunk_ret_vec),
                Err((partial_chunk_ret_vec, e)) => {
                    ret_vec.extend(partial_chunk_ret_vec);
                    errors.extend(e);
                    if !ALLOW_ERR {
                        return Err((ret_vec, errors));
                    }
                },
            }
        };
    }

    while let Some(spawn_control_chunk) = chunks.next() {
        get_chunk!(spawn_control_chunk.into_iter());
    }

    if let Some (spawn_control_rem_iter) = chunks.into_remainder() {
        get_chunk!(spawn_control_rem_iter);
    }

    if errors.is_empty() {
        Ok(ret_vec)
    } else {
        Err((ret_vec, errors))
    }
}

pub(crate) async fn chunked_spawn_runner<
    const CHUNK_SZ: usize,
    const ALLOW_ERR: bool,
    T, I, R, E, Fu, F>(full_iter: I, f: F)
    -> Result<Vec<R>, (Vec<R>, Vec<crate::AllocPerfError>)>
    where I: Iterator<Item = T>,
          T: Send + Clone + std::fmt::Debug + 'static,
          Fu: Future<Output = Result<R, E>> + Send + 'static,
          F: Fn(T) -> Fu + Copy,
          R: Send + 'static,
          E: Into<crate::AllocPerfError> + Send + std::fmt::Display + 'static,
{
    chunked_spawn_runner_with_retries::<CHUNK_SZ, 0, ALLOW_ERR, _, _, _, _, _, _>(full_iter, f).await
}
