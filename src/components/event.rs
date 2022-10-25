use yew::{Context, Html, html};
use yew::prelude::Component;
use yew_router::prelude::*;

use crate::apis::app::{AppMsg, load_pods_future};
use crate::helper::router::Route;

pub struct Event {
    pods: Vec<serde_json::Value>,
}

impl Component for Event {
    type Message = AppMsg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_future(load_pods_future(None, None, "events".to_string()));
        Self {
            pods: vec![],
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            AppMsg::UpdateNs(_) => {}
            AppMsg::LoadPodsDone(pods_str) => {
                self.pods = serde_json::from_str(pods_str.as_str()).unwrap();
            }
        }
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div>

            <ul role="menubar" class="el-menu-demo el-menu--horizontal el-menu">
            <li role="menuitem" tabindex="0" class="el-menu-item">
             <Link<Route>  to={Route::PodDetail { ns: "kubesphere-system".to_string(), id: "minio-make-bucket-job-zfd2q".to_string() } }>{ {"资源状态"} }</Link<Route>>
            </li>
            <li role="menuitem" tabindex="0" class="el-menu-item is-active">
              <Link<Route>  to={Route::Event}>{ {"事件"} }</Link<Route>>
            </li>
            </ul>
           {{"这里是事件"}}
            </div>
        }
    }
}



