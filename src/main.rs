mod app;

pub mod components;
pub mod pages;
pub mod webgl;

use app::App;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());

    yew::start_app::<App>();
}
