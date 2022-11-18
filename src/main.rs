extern crate core;

use helper::router::MyRoute;

pub mod components;
mod apis;
mod element_ui;
mod helper;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<MyRoute>();
}
