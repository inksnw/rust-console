use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::{
    daemonsets::DaemonSets, deploy::Deploy,
    event::Event, form::FormTest,
    jobs::Jobs, node::Nodes,
    pod::Pods, pod_detail::PodDetail,
    services::Services,
    statefulsets::StateFulSets,
};

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
    match routes {
        Route::Home => html! {
            <div style="margin-left: 200px;height: 100%;"><Nodes/></div>},
        Route::Pods => html! {
            <div style="margin-left: 200px;height: 100%;"><Pods/></div>},
        Route::Deploy => html! {
            <div style="margin-left: 200px;height: 100%;"><Deploy/></div>},
        Route::DaemonSets => html! {
            <div style="margin-left: 200px;height: 100%;"><DaemonSets/></div>},
        Route::StateFulSets => html! {
            <div style="margin-left: 200px;height: 100%;"><StateFulSets/></div>},
        Route::Jobs => html! {
            <div style="margin-left: 200px;height: 100%;"><Jobs/></div>},
        Route::Services => html! {
            <div style="margin-left: 200px;height: 100%;"><Services/></div>},
        Route::NotFound => html! {
             <div style="margin-left: 200px;height: 100%;"><h1>{ "404" }</h1></div>
        },
        Route::PodDetail { ns, id } => html! {
            <div style="margin-left: 200px;height: 100%;"><PodDetail ns={ns.clone()} name={id.clone()}/></div>
        },
        Route::Event { ns, id } => html! {
            <div style="margin-left: 200px;height: 100%;"><Event ns={ns.clone()} name={id.clone()} /></div>
        },
        Route::FormTest => html! {
            <div style="margin-left: 200px;height: 100%;"><FormTest  /></div>
        }
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
