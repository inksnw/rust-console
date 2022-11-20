use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::{
    detail::deploy::DeployDetail,
    detail::pod::PodDetail,
    form::FormTest,
};
use crate::components::list::{
    daemonsets::DaemonSets, deploy::Deploy,
    event::Event, jobs::Jobs, node::Nodes,
    pod::Pods, services::Services,
};
use crate::components::list::statefulsets::StateFulSets;

#[derive(Routable, PartialEq, Eq, Clone, Debug)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/Pod")]
    Pods,
    #[at("/Event/:ns/:id")]
    Event { ns: String, id: String },
    #[at("/Pod/:ns/:id")]
    PodDetail { ns: String, id: String },
    #[at("/Deployment")]
    Deploy,
    #[at("/Deployment/:ns/:id")]
    DeployDetail { ns: String, id: String },
    #[at("/DaemonSets")]
    DaemonSets,
    #[at("/StateFulSets")]
    StateFulSets,
    #[at("/Jobs")]
    Jobs,
    #[at("/Services")]
    Services,
    #[not_found]
    #[at("/404")]
    NotFound,
    #[at("/forms")]
    FormTest,
}


fn switch(routes: &Route) -> Html {
    let style = "margin-left: 200px;height: 100%;";
    match routes {
        Route::Home => html! {<div style={style}><Nodes/></div>},
        Route::Pods => html! {<div style={style}><Pods/></div>},
        Route::Deploy => html! {<div style={style}><Deploy/></div>},
        Route::DaemonSets => html! {<div style={style}><DaemonSets/></div>},
        Route::StateFulSets => html! {<div style={style}><StateFulSets/></div>},
        Route::Jobs => html! {<div style={style}><Jobs/></div>},
        Route::Services => html! {<div style={style}><Services/></div>},
        Route::NotFound => html! { <div style={style}><h1>{ "404" }</h1></div>        },
        Route::FormTest => html! {<div style={style}><FormTest/></div>},
        Route::PodDetail { ns, id } => html! {
            <div style={style}><PodDetail ns={ns.clone()} name={id.clone()}/></div>},
        Route::DeployDetail { ns, id } => html! {
            <div style={style}><DeployDetail ns={ns.clone()} name={id.clone()}/></div>},
        Route::Event { ns, id } => html! {
            <div style={style}><Event ns={ns.clone()} name={id.clone()} /></div>},
    }
}


#[function_component(MyRoute)]
pub fn root() -> Html {
    html!(
        <BrowserRouter>
            { view_nav() }
            <Switch<Route> render={Switch::render(switch)} />
        </BrowserRouter>
    )
}


fn render_nav_item(item: Route, name: String) -> Html {
    html!(
            <li role="menuitem" tabindex="-1" class="el-menu-item " style="padding-left: 20px;">
            <i class="el-icon-document"></i>
            <span>
            <Link<Route>  to={item}>{ name }</Link<Route>>
            </span>
            </li>
        )
}


fn view_nav() -> Html {
    html!(
            <div>
            <div style="position: absolute;top: 0;left: 0; width: 200px;height: 100%;">
            <ul role="menubar" class="el-menu-vertical-demo el-menu">
            {{ render_nav_item(Route::Home,"nodes".to_string())}}
            {{ render_nav_item(Route::Deploy,"工作负载".to_string())}}
            {{ render_nav_item(Route::Pods,"pods".to_string())}}
            {{ render_nav_item(Route::Jobs,"jobs".to_string())}}
            {{ render_nav_item(Route::Services,"Services".to_string())}}
            </ul>
            </div>
            </div>
        )
}
