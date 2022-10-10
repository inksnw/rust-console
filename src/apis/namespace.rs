use std::future::Future;

use gloo_net::http::Request;
use k8s_openapi::api::core::v1::Namespace;

use crate::element_ui::ValueText;

pub enum NamespaceMsg {
    LoadNS,
    LoadNSDone(Vec<Namespace>),
    Onchange(ValueText),
}

#[derive(serde::Deserialize)]
struct ApiResult {
    // total_items: u32,
    items: Vec<Namespace>,
}

pub async fn load_ns() -> Result<Vec<Namespace>, wasm_bindgen::JsValue> {
    let rv: ApiResult = Request::get(&super::name_space_api().as_str()).send().
        await.unwrap().json().await.unwrap();

    Ok(rv.items)
}

pub fn load_ns_future() -> impl Future<Output=NamespaceMsg> + 'static
{
    let f = async {
        match load_ns().await {
            Ok(ns_list) => NamespaceMsg::LoadNSDone(ns_list),
            Err(_) => NamespaceMsg::LoadNSDone(vec![])
        }
    };
    return f;
}
