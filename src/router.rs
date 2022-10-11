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
        Route::Home => html! {
            <div style="margin-left: 200px;height: 100%;" >
            <Resource/>
            </div>
            },
        Route::NotFound => html! {
             <div style="margin-left: 200px;height: 100%;">
             <h1>{ "404" }</h1>
             </div>
            }
    }
}

pub enum Msg {}

pub(crate) struct MyRoute {}

impl Component for MyRoute {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }


    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
        <BrowserRouter>
            { self.view_nav() }
            <Switch<Route> render={Switch::render(switch)} />
        </BrowserRouter>
        }
    }
}

impl MyRoute {
    fn view_nav(&self) -> Html {
        html! {
            <div style="position: absolute;top: 0;left: 0; width: 200px;height: 100%;">
            <ul role="menubar" class="el-menu-vertical-demo el-menu">
            <li role="menuitem" tabindex="-1" class="el-menu-item" style="padding-left: 20px;">
            <span>
              <Link<Route> classes={classes!("navbar-item")} to={Route::Home}>{ "pod" }</Link<Route>>
            </span></li>
            <li role="menuitem" tabindex="-2" class="el-menu-item" style="padding-left: 20px;">
            <span>
              <Link<Route> classes={classes!("navbar-item")} to={Route::NotFound}>{ "deploy" }</Link<Route>>
            </span></li>
            </ul>
            </div>
        }
    }
}
