use chrono::{Utc, NaiveDate};
use speedy::{Readable, Writable};
use regex::Regex;

use std::{collections::{BTreeMap, BTreeSet}, ops::RangeInclusive};
use std::sync::LazyLock;
use std::iter;

use crate::conf::SubFull;
use crate::booies_cache::BooiesDetailsCache;

use crate::storage_util::{IsSpeedyRwRd, StorageInfo};
use crate::deserialize_util::{YearOrYMD, MapOrSeq};
use crate::wrapper_types::NDWrapper;

use crate::booies::{Booies, BooiesExample, BooiesExampleInfo, BooiesExampleSadioInfo, BooiesExampleFigureInfo, BooiesDetails};

type CCI = CommonConfInfo;

use crate::AllocPerfRes;

#[derive(Readable, Writable)]
pub(crate) struct BooiesIndex {
    categories: BTreeMap<u64, String>,
    list: BTreeMap<u64, Booies>,
    category_booies_map: BTreeMap<u64, BTreeSet<u64>>,
    uncategoriezed_booies_nums: BTreeSet<u64>,
}

impl BooiesIndex {
    fn gen_random(sz: usize) -> Self {
        fn gen_unique_ids(range_val: RangeInclusive<u64>, range_count: RangeInclusive<usize>) -> impl Iterator<Item=u64> {
            iter::repeat_with(|| fastrand::u64(range_val.clone()))
                .take(2*{*range_count.end()})
                // dedup
                .collect::<BTreeSet<_>>()
                .into_iter()
                .take(fastrand::usize(range_count))
        }

        let uncategoriezed_booies_nums = BTreeSet::new();

        let lower = {sz*75/100}.max(1);
        let upper = {sz*125/100};

        let categories: BTreeMap<_,_> = gen_unique_ids(1000..=10000, lower..=upper)
            .map(|cat_id| (
                cat_id,
                itertools::join({2..=4}.map(|_| crate::rand_str(3..=32)), " "),
            )).collect();

        let list = gen_unique_ids(10001..=100000, lower*categories.len()..=upper*categories.len())
            .map(|booies_id| {
                let category_id = categories
                    .keys()
                    .nth(fastrand::usize(..categories.len()))
                    .copied()
                    .expect("impossible");
                Booies {
                    booies_id,
                    num: booies_id,
                    name: itertools::join({2..=6}.map(|_| crate::rand_str(3..=32)), " "),
                    last_modified: fastrand::i64(1700000000..=1720000000),
                    genre: crate::rand_str(8..=24),
                    release_date: NaiveDate::from_ymd_opt(
                        // allow out-of-range values that will turn to `None`s
                        fastrand::i32(2000..=2023),
                        fastrand::u32(1..=12),
                        fastrand::u32(1..=31),
                    ).map(|nd| YearOrYMD::YMD(NDWrapper(nd))),
                    category_id: Some(category_id),
                    category_ids: Some(vec![category_id]),
                    rating: Some(fastrand::f64()*10.0),
                }
            }).map(|boo| (boo.booies_id, boo)).collect::<BTreeMap<_,_>>();

        let category_booies_map = categories
            .keys()
            .copied()
            .map(|category_id| (
                category_id,
                list
                    .values()
                    .filter(|boo| boo.category_id == Some(category_id))
                    .map(|boo| boo.booies_id)
                    .collect(),
            )).collect();

        Self { categories, list, category_booies_map, uncategoriezed_booies_nums }
    }
}

#[derive(Readable, Writable)]
pub(crate) struct AllInfo {
    fetched_at: i64,
    booies_index: Option<BooiesIndex>,
}

impl AllInfo {
    pub(crate) fn gen_random(sz: usize) -> Self {
        Self {
            fetched_at: Utc::now().timestamp(),
            booies_index: Some(BooiesIndex::gen_random(sz)),
        }
    }

    pub(crate) fn gen_booies_details(&self) -> BooiesDetailsCache {
        let mut cache = BooiesDetailsCache::new();
        let mut id_iter = 500_000_u64..;
        for boo in self.booies_index
            .as_ref()
            .iter()
            .map(|booies_index| booies_index.list.values())
            .flatten()
        {
            let mut examples_chapters = Vec::with_capacity(12);
            for chapter in 1..=fastrand::u64(1..=12) {
                let mut examples = Vec::with_capacity(24);
                for example_num in 1..=fastrand::u64(6..=24) {
                    let duration_secs = fastrand::u64(600..=5000);
                    let example = BooiesExample {
                        id: id_iter.next().expect("impossible"),
                        chapter: Some(chapter),
                        example_num: Some(example_num),
                        title: itertools::join({2..=6}.map(|_| crate::rand_str(3..=32)), " "),
                        container_extension: crate::rand_str(2..=4),
                        added: 1720000000,
                        info: Some(BooiesExampleInfo {
                            duration_secs: Some(duration_secs),
                            duration: Some(format!("{:02}:{:02}:{:02}",
                                    duration_secs/3600,
                                    (duration_secs%3600)/60,
                                    duration_secs%60)),
                                    bitrate: Some(fastrand::u64(1000..=15000)),
                                    sadio: Some(BooiesExampleSadioInfo{
                                        boec_name: Some("SAD".into()),
                                        bad_rate: 20000,
                                        channels: 2,
                                    }),
                                    figure: Some(BooiesExampleFigureInfo {
                                        boec_name: Some("FIG".into()),
                                        wigth: 5000,
                                        feight: 1000,
                                    }),
                        }),
                    };
                    examples.push(example);
                }
                examples_chapters.push(examples);
            }
            let boo_details = BooiesDetails{ examples: Some(MapOrSeq::Seq(examples_chapters)) };
            cache.insert(boo.num, boo_details);
        }
        cache
    }

    fn mk_cci(&self, sub: &SubFull) -> CCI {
        let full_server_url = sub.domain.clone();
        let username = sub.username.clone();
        let password = sub.password.clone();
        let sub = sub.clone();
        CommonConfInfo{full_server_url, username, password, sub}
    }
}

impl StorageInfo for AllInfo {
    const DESC: &'static str = "all info";
}

impl IsSpeedyRwRd for AllInfo {
    const FILE_NAME: &'static str = "ALL";
}

pub(crate) struct CommonConfInfo {
    full_server_url: String,
    username: String,
    password: String,
    sub: SubFull,
}

pub(crate) struct ExtractedBooiesExampleStweem {
    name: String,
    url: String,
}

pub(crate) struct EBESMap(BTreeMap<u64, Vec<ExtractedBooiesExampleStweem>>);

impl EBESMap {
    fn _final_filtered_list<'a>(booies_index: &'a BooiesIndex) -> impl Iterator<Item=&'a Booies> + 'a {
        booies_index.list.values()
    }

    async fn mk(booies_index: &BooiesIndex, cci: &CCI) -> AllocPerfRes<Self> {
        let filtered_list = Self::_final_filtered_list(booies_index);

        let cache = BooiesDetailsCache::get_local_or_new(&cci.sub).await?;

        let inner = filtered_list
            .map(|boo| boo.num)
            .filter_map(|num| {
                let boo_details = cache.get_boo_details(num);

                if boo_details.is_none() {
                    tracing::error!("details of booies '{}' not in cache", booies_index.list[&num].name);
                }
                boo_details.and_then(|boo_det| boo_det.examples.as_ref().map(|examples| (num, examples)))
            })
            .map(|(num, examples)| {
                Ok((
                    num,
                    examples.values()
                        .flatten()
                        .into_iter()
                        .map(|example| Self::mk_extracted_st(&cci, &booies_index.list[&num], &example))
                        .collect::<AllocPerfRes<Vec<_>>>()?,
                 ))
            })
            .collect::<AllocPerfRes<BTreeMap<_, _>>>()?;
        Ok(Self(inner))
    }
}

impl EBESMap {
    pub(crate) async fn mk_from_all(all: &AllInfo, sub: &SubFull) -> AllocPerfRes<Option<Self>> {
        let cci = all.mk_cci(&sub);
        match all.booies_index.as_ref() {
            None => Ok(None),
            Some(booies_index) => {
                Ok(Some(Self::mk(&booies_index, &cci).await?))
            },
        }
    }

    fn mk_extracted_st(cci: &CCI, booies: &Booies, example: &BooiesExample) -> AllocPerfRes<ExtractedBooiesExampleStweem> {
        let server = &*cci.full_server_url;

        let user = &cci.username;
        let pass = &cci.password;

        let id = example.id;
        let ext = &example.container_extension;
        let url = format!("booies|{server}|{user}|{pass}|{id}.{ext}");

        let name_extra = example
            .info.as_ref().map(|example_info| {
                let (v_boec, w_h) = match example_info.figure.as_ref() {
                    Some(figure) => {
                        let v_boec = figure.boec_name
                            .as_deref()
                            .map(str::to_uppercase);
                        let w = figure.wigth;
                        let f = figure.feight;
                        (v_boec, Some(format!("{w}^{f}")))
                    },
                    None => (None, None),
                };

                let (a_boec, ch) = match example_info.sadio.as_ref() {
                    Some(sadio) => {
                        let a_boec = sadio.boec_name
                            .as_deref()
                            .map(str::to_uppercase);

                        let ch = match sadio.channels {
                            0 => "unknown",
                            1 => "uni",
                            2 => "bidi",
                            _ => "multi",
                        }.to_owned();
                        (a_boec, Some(ch))
                    },
                    None => (None, None),
                };

                let b_kibs = example_info.bitrate.as_ref().map(|b| format!("{b} kibs"));

                let dur_hms = example_info.duration.as_ref()
                    .map(Clone::clone)
                    .or_else(|| example_info.duration_secs.map(|dur_secs| {
                        let h = dur_secs / 3600;
                        let m = (dur_secs % 3600) / 60;
                        let s = dur_secs % 60;
                        format!("{h:02}:{m:02}:{s:02}")
                    }));

                let full = [w_h, ch, dur_hms, b_kibs, v_boec, a_boec]
                    .into_iter()
                    .filter_map(|s| s)
                    .collect::<Vec<_>>()
                    .join(" / ");

                full.is_empty()
                    .then_some("".to_owned())
                    .unwrap_or(" / ".to_owned() + &full)
            }).unwrap_or_default();

        static C_N_CAP: LazyLock<Regex> = LazyLock::new(|| Regex::new(r".*\bC(\d+)(\s+)?N(\d+).*").expect("valid regex"));

        let name = match (example.chapter, example.example_num) {
            (Some(c), Some(n)) => {
                format!("{} C{c:02}N{n:02}{name_extra}", booies.name)
            },
            _ => {
                let cap_opt = C_N_CAP.captures(&example.title);
                if let Some(cap) = cap_opt &&
                    let (Some(Ok(c)), Some(Ok(n))) = (cap.get(1).map(|c| c.as_str().parse::<u64>()), cap.get(3).map(|n| n.as_str().parse::<u64>()))
                {
                    tracing::warn!("chapter and example_num info missing from example, guessing from example title: '{}'", example.title);
                    tracing::warn!("Guessed example C/N: C{c:02}N{n:02}");
                    format!("{} C{c:02}N{n:02}{name_extra}", booies.name)
                } else {
                    tracing::warn!("chapter and example_num info missing, using example title");
                    format!("{} {}{name_extra}", example.title, booies.name)
                }
            },
        };

        Ok(ExtractedBooiesExampleStweem { name, url })

    }
}
