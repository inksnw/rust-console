use std::future::Future;

use gloo_net::http::Request;

pub trait UserTrait {
    fn get_name(self) -> String;
}

pub enum TestMsg {
    TestClick,
    LoadUser,
    LoadUserDone(String),
}

pub async fn get_user<T: serde::Serialize + serde::de::DeserializeOwned + Clone + PartialEq>() -> Result<T, wasm_bindgen::JsValue> {
    let obj: T = Request::get(super::with_path("/test").as_str()).send().
        await.unwrap().json().await.unwrap();
    Ok(obj)
}

pub fn get_user_future<T>() -> impl Future<Output=TestMsg> + 'static
    where
        T: serde::Serialize + serde::de::DeserializeOwned + Clone + PartialEq + UserTrait
{
    let f = async {
        match get_user::<T>().await {
            Ok(user) => TestMsg::LoadUserDone(user.get_name()),
            Err(_) => TestMsg::LoadUserDone("no data".to_string())
        }
    };
    return f;
}