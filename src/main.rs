extern crate core;

use helper::router::MyRoute;
use crate::helper::websocket::WebsocketService;

pub mod components;
mod apis;
mod element_ui;
mod helper;

fn main() {
     WebsocketService::new();
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<MyRoute>();
}
