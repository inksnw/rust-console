use yew::html::Scope;
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

pub enum Msg {
    ToggleNavbar,
}

pub(crate) struct MyRoute {
    navbar_active: bool,
}

impl Component for MyRoute {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            navbar_active: false,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ToggleNavbar => {
                self.navbar_active = !self.navbar_active;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
        <BrowserRouter>
            { self.view_nav(ctx.link()) }
            <Switch<Route> render={Switch::render(switch)} />
        </BrowserRouter>
        }
    }
}

impl MyRoute {
    fn view_nav(&self, link: &Scope<Self>) -> Html {
        let Self { navbar_active, .. } = *self;

        let active_class = if !navbar_active { "is-active" } else { "" };

        html! {
            <nav class="navbar is-primary" role="navigation" aria-label="main navigation">
                <div class="navbar-brand">
                    <h1 class="navbar-item is-size-3">{ "Yew Blog" }</h1>
                    <button class={classes!("navbar-burger", "burger", active_class)}
                        aria-label="menu" aria-expanded="false"
                        onclick={link.callback(|_| Msg::ToggleNavbar)}
                    >
                        <span aria-hidden="true"></span>
                        <span aria-hidden="true"></span>
                        <span aria-hidden="true"></span>
                    </button>
                </div>
                <div class={classes!("navbar-menu", active_class)}>
                    <div class="navbar-start">
                        <Link<Route> classes={classes!("navbar-item")} to={Route::Home}>
                            { "Home" }
                        </Link<Route>>

                    </div>
                </div>
            </nav>
        }
    }
}
