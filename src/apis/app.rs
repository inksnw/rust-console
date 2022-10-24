use std::future::Future;

use gloo_net::http::Request;

use crate::element_ui::ValueText;

pub enum AppMsg {
    UpdateNs(ValueText),
    LoadPodsDone(String),
}


pub async fn load_resource(ns: Option<String>, resource_type: String) -> Result<String, wasm_bindgen::JsValue> {
    let pod_list = Request::get(&super::core_v1(ns, resource_type).as_str()).send().
        await.unwrap().text().await.unwrap();
    Ok(pod_list)
}

pub fn load_pods_future(ns: Option<String>, resource_type: String) -> impl Future<Output=AppMsg> + 'static
{
    let f = async {
        match load_resource(ns, resource_type).await {
            Ok(pods_list) => AppMsg::LoadPodsDone(pods_list),
            Err(_) => AppMsg::LoadPodsDone("[]".to_string())
        }
    };
    return f;
}
