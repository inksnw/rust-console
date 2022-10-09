use components::app::TestComp;

mod components;
mod helper;
mod apis;
mod element_ui;

fn main() {
    yew::start_app::<TestComp>();
}
