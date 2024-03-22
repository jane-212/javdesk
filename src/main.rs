use env_logger::{Builder, Env};
use gpui::App;

mod app;
mod app_state;
mod assets;
mod components;
mod config;
mod db;
mod proxy;
mod theme;
mod views;

use app::run_app;

#[global_allocator]
static GLOBAL: jemallocator::Jemalloc = jemallocator::Jemalloc;

fn main() {
    #[cfg(debug_assertions)]
    Builder::from_env(Env::new().default_filter_or("info")).init();
    #[cfg(not(debug_assertions))]
    Builder::from_env(Env::new().default_filter_or("warn")).init();
    let app = App::new();
    run_app(app);
}
