use router::MyRoute;

pub mod components;
mod apis;
mod element_ui;
mod router;

fn main() {
    yew::start_app::<MyRoute>();
}
