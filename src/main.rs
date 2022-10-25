use helper::router::MyRoute;

pub mod components;
mod apis;
mod element_ui;
mod helper;

fn main() {
    yew::start_app::<MyRoute>();
}
