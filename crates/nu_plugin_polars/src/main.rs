use nu_plugin::{serve_plugin, MsgPackSerializer};
use nu_plugin_polars::PolarsPlugin;

#[cfg(not(target_env = "msvc"))]
use jemallocator::Jemalloc;

#[cfg(not(target_env = "msvc"))]
#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

fn main() {
    serve_plugin(&PolarsPlugin::default(), MsgPackSerializer {})
}
