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

use tracing_subscriber::EnvFilter;

use std::io;

fn main() -> alloc_perf_test::AllocPerfRes<()> {
    /*
    use io::Read;
    let mut buf = [0u8; 1];
    io::stdin().read_exact(&mut buf).unwrap();
    */

    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "alloc_perf_test=info");
    }
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_writer(|| io::stderr())
        .init();

    std::env::set_var("ASYNC_GLOBAL_EXECUTOR_THREADS", "16");
    async_global_executor::block_on(alloc_perf_test::cli::cli())
}
