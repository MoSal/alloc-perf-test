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
