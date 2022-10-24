pub mod app;
pub mod apiv1;

const BASE_URL: &str = "http://localhost:9090";

pub fn with_path(p: &str) -> String {
    format!("{}{}", BASE_URL, p)
}

pub fn api_v1(resource_type: String) -> String {
    with_path(&format!("/kapis/{}", resource_type))
}

pub fn core_v1(ns: Option<String>, resource_type: String) -> String {
    match ns {
        None => {
            with_path(&format!("/kapis/namespaces/default/{}", resource_type))
        }
        Some(s) => {
            with_path(
                &format!("/kapis/namespaces/{}/{}", s, resource_type)
            )
        }
    }
}

