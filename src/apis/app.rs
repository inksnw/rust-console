use std::future::Future;

use gloo_net::http::Request;

use crate::element_ui::ValueText;

pub enum AppMsg {
    TestClick,
    UpdateMyName(String),
    UpdateNs(ValueText),
    LoadPodsDone(String),
}


pub async fn load_pods(ns: Option<String>) -> Result<String, wasm_bindgen::JsValue> {
    let pod_list = Request::get(&super::pods_api(ns).as_str()).send().
        await.unwrap().text().await.unwrap();
    Ok(pod_list)
}

pub fn load_pods_future(ns: Option<String>) -> impl Future<Output=AppMsg> + 'static
{
    let f = async {
        match load_pods(ns).await {
            Ok(pods_list) => AppMsg::LoadPodsDone(pods_list),
            Err(_) => AppMsg::LoadPodsDone("[]".to_string())
        }
    };
    return f;
}
