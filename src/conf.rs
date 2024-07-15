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

use std::path::PathBuf;

use crate::all::{AllInfo, EBESMap};
use crate::booies_cache::BooiesDetailsCache;
use crate::fs_util;
use crate::storage_util::StorageOpsSpeedy;
use crate::spawn_util;
use crate::AllocPerfError;

use crate::AllocPerfRes;

#[derive(Clone, Debug)]
pub(crate) struct SubFull {
    pub(crate) idx: u8,
    pub(crate) domain: String,
    pub(crate) username: String,
    pub(crate) password: String,
}

impl SubFull {
    pub(crate) async fn mk_sub_dir_path(&self) -> AllocPerfRes<PathBuf> {
        let path = PathBuf::from(self.idx.to_string());
        fs_util::dir::exists_or_create(&path).await?;
        Ok(path)
    }
}

#[derive(Debug)]
pub struct Subs(Vec<SubFull>);

impl Subs {
    pub fn gen_subs(n: u8) -> Self {
        let subs = {1..=n}.map(|idx| {
            SubFull {
                idx,
                domain: format!("https://{}.com", crate::rand_str(16..=16).to_ascii_lowercase()),
                username: crate::rand_str(16..=16),
                password: crate::rand_str(16..=16),
            }
        }).collect();
        Self(subs)
    }
    pub async fn gen_save_all(n: u8, sz: usize) -> AllocPerfRes<()> {
        let subs = Self::gen_subs(n);
        for n in 1..=n {
            tracing::info!("gen and save all for {n}");
            let path = AllInfo::get_path(&subs.0[(n-1) as usize]).await?;
            let (all, _) = AllInfo::gen_random(sz).with_updated_binz_file(&path).await?;
            tracing::info!("gen and save boo_cache for {n}");
            let path = BooiesDetailsCache::get_path(&subs.0[(n-1) as usize]).await?;
            let _ = all.gen_booies_details().with_updated_binz_file(&path).await?;
        }
        Ok(())
    }
    async fn sub_to_all_info(sub: &SubFull) -> AllocPerfRes<(u8, String, AllocPerfRes<AllInfo>)> {
        let sub_idx = sub.idx;
        let sub_dom = sub.domain.clone();
        let sub_all_info_res = try {
            AllInfo::from_local(sub).await?
        };
        Ok((sub_idx, sub_dom, sub_all_info_res))
    }
}

impl Subs {
    pub async fn print_booies_examples_list(&self) -> AllocPerfRes<()> {
        async fn get_sub_e_map(sub: SubFull) -> AllocPerfRes<(u8, String, AllocPerfRes<AllocPerfRes<Option<EBESMap>>>)> {
            let (sub_idx, sub_dom, all_info_res) = Subs::sub_to_all_info(&sub).await?;
            match all_info_res {
                Err(e) => Ok((sub_idx, sub_dom, Err(e))),
                Ok(all_info) => {
                    let e_map_res = EBESMap::mk_from_all(&all_info, &sub).await; 
                    Ok((sub_idx, sub_dom, Ok(Ok(e_map_res?))))
                },
            }
        }

        let runner_args = self.0
            .iter()
            .cloned();

        let runner = spawn_util::chunked_spawn_runner::<{crate::SPAWN_CHUNK_SZ}, false, _, _, _, _, _, _>;
        let e_maps_info = runner(runner_args, get_sub_e_map).await
            .map_err(|(_, errors)| AllocPerfError::Multi(errors))?;

        let mut e_maps = Vec::with_capacity(e_maps_info.len());

        for (sub_idx, _sub_dom, sub_e_map_res_res) in e_maps_info {
            match sub_e_map_res_res {
                Err(e) => tracing::error!("failed to get {} info for subscription {sub_idx}: {e}", "booies example"),
                Ok(Err(e)) => tracing::error!("failed to get extracted {} map for subscription {sub_idx}: {e}", "booies example"),
                Ok(Ok(e_map_opt)) => e_maps.push(e_map_opt),
            }
        }
        Ok(())
    }
}
