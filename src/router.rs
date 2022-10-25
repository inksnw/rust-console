use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::{deploy::Deploy, node::Nodes, pod::Pods};

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/Pod")]
    Pods,
    #[at("/Deployment")]
    Deploy,
    #[not_found]
    #[at("/404")]
    NotFound,
}


fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html! {
            <div style="margin-left: 200px;height: 100%;"><Nodes/></div>},
        Route::Pods => html! {
            <div style="margin-left: 200px;height: 100%;"><Pods/></div>},
        Route::Deploy => html! {
            <div style="margin-left: 200px;height: 100%;"><Deploy/></div>},
        Route::NotFound => html! {
             <div style="margin-left: 200px;height: 100%;"><h1>{ "404" }</h1></div>
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
    fn render_nav_item(&self, item: Route, name: String) -> Html {
        html! {
            <li role="menuitem" tabindex="-1" class="el-menu-item " style="padding-left: 20px;">
            <i class="el-icon-document"></i>
            <span>
            <Link<Route>  to={item}>{ name }</Link<Route>>
            </span>
            </li>
        }
    }


    fn view_nav(&self) -> Html {
        html! {
            <div>
            <div>
                <div aria-label="Breadcrumb" role="navigation" class="el-breadcrumb">
                    <span class="el-breadcrumb__item">
                        <span role="link" class="el-breadcrumb__inner is-link">{{"首页"}}</span>
                        <span role="presentation" class="el-breadcrumb__separator">{{"/"}}</span>
                    </span>
                    <span class="el-breadcrumb__item">
                        <span role="link" class="el-breadcrumb__inner"><a href="/">{{"节点列表"}}</a></span>
                    </span>
                </div>
            </div>
            <div style="position: absolute;top: 150;left: 0; width: 200px;height: 100%;">
            <ul role="menubar" class="el-menu-vertical-demo el-menu">
            {{ self.render_nav_item(Route::Home,"nodes".to_string())}}
            {{ self.render_nav_item(Route::Deploy,"deploy".to_string())}}
            {{ self.render_nav_item(Route::Pods,"pods".to_string())}}
            </ul>
            </div>
            </div>
        }
    }
}
