use clap::{ValueEnum, Parser};
use std::fmt::Debug;

use crate::conf::Subs;


use crate::AllocPerfRes;
#[derive(ValueEnum, Debug, Clone, Copy, PartialEq)]
#[clap(rename_all="snake_case")]
pub enum MediaSource {
    Booies,
}

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq)]
#[clap(rename_all="snake_case")]
pub enum BooiesDetailsCacheRefresh {
    Never,
    Auto,
    Forced,
}

#[derive(Parser, Debug)]
#[clap(rename_all="kebab-case")]
struct GeneralArgs {
    /// number of subs
    #[clap(short, default_value="8")]
    n: u8,
}

#[derive(Parser, Debug)]
#[clap(rename_all="kebab-case")]
enum Commands {
    TestAllocPerf {
        #[clap(flatten)]
        general: GeneralArgs,
    },
    GenData {
        #[clap(flatten)]
        general: GeneralArgs,
        /// rough size of generated data relative to the default (SZ/DEF)^2
        #[clap(short, default_value="100")]
        sz: usize,
    },
}

pub async fn cli() -> AllocPerfRes<()> {
    let mut commands = Commands::parse();
    tracing::debug!("{commands:#?}");

    match &mut commands {
        Commands::GenData { general, sz } => {
            Subs::gen_save_all(general.n, *sz).await?;
        },
        Commands::TestAllocPerf{ general } => {
            Subs::gen_subs(general.n)
                .print_booies_examples_list()
                .await?;
        },
    }
    Ok(())
}
