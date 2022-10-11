pub mod app;
pub mod namespace;

const BASE_URL: &str = "http://localhost:9090";

pub fn with_path(p: &str) -> String {
    format!("{}{}", BASE_URL, p)
}

pub fn name_space_api() -> String {
    with_path("/kapis/namespaces")
}

pub fn pods_api(ns: Option<String>) -> String {
    match ns {
        None => with_path("/kapis/namespaces/default/pods"),
        Some(s) => {
            with_path(
                &format!("/kapis/namespaces/{}/pods", s)
            )
        }
    }
}