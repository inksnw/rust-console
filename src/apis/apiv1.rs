use std::future::Future;

use gloo_net::http::Request;

use crate::element_ui::ValueText;

pub enum NamespaceMsg {
    LoadNS,
    LoadNSDone(String),
    Onchange(ValueText),

}


pub async fn load_ns(resource_type: String) -> Result<String, wasm_bindgen::JsValue> {
    let rv = Request::get(&super::api_v1(resource_type).as_str()).send().
        await.unwrap().text().await.unwrap();
    Ok(rv)
}

pub fn load_ns_future(resource_type: String) -> impl Future<Output=NamespaceMsg> + 'static
{
    let f = async {
        match load_ns(resource_type).await {
            Ok(ns_list) => NamespaceMsg::LoadNSDone(ns_list),
            Err(_) => NamespaceMsg::LoadNSDone("[]".to_string())
        }
    };
    return f;
}
