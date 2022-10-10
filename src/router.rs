use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::app::Resource;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound,
}


fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html! { <Resource/> },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}


pub(crate) struct MyRoute {}

impl Component for MyRoute {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
        <BrowserRouter>
            <Switch<Route> render={Switch::render(switch)} />
        </BrowserRouter>
        }
    }
}