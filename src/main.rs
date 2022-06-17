#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

mod app;

pub mod components;
pub mod pages;
pub mod webgl;

#[macro_use]
extern crate log;

use app::App;

fn main() {
    info!("Setting up application...");
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}

/*

Interface graphique
Colors:
    - red #6A041D
    - orange #F5B841
    - yellow #F4FF52
    - green #53FF45
    - blue #1E2EDE
    - background #131615
    - orange text #CA3C25
    - hunter green #2C5530

*/