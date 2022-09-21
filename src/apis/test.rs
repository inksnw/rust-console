use std::future::Future;

use gloo_net::http::Request;
use k8s_openapi::api::core::v1::Namespace;

pub enum TestMsg {
    TestClick,
    LoadNS,
    LoadNSDone(Vec<Namespace>),
}

#[derive(serde::Deserialize)]
// #[serde(rename(total_items = "totalItems"))]
struct ApiResult {
    total_items: u32,
    items: Vec<Namespace>,
}

pub async fn load_ns() -> Result<Vec<Namespace>, wasm_bindgen::JsValue> {
    let rv: ApiResult = Request::get(&super::name_space_api().as_str()).send().
        await.unwrap().json().await.unwrap();

    Ok(rv.items)
}

pub fn load_ns_future() -> impl Future<Output=TestMsg> + 'static
{
    let f = async {
        match load_ns().await {
            Ok(nslist) => TestMsg::LoadNSDone(nslist),
            Err(_) => TestMsg::LoadNSDone(vec![])
        }
    };
    return f;
}