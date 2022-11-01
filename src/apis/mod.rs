pub mod app;
pub mod apiv1;
pub mod detail;

const BASE_URL: &str = "http://localhost:9090";

pub fn with_path(p: &str, page: Option<String>) -> String {
    match page {
        Some(s) => {
            format!("{}{}?limit=5&page={}", BASE_URL, p, s)
        }
        None => {
            format!("{}{}", BASE_URL, p)
        }
    }
}

pub fn api_v1(resource_type: String, page: Option<String>) -> String {
    with_path(&format!("/kapis/{}", resource_type), page)
}

pub fn core_v1(ns: Option<String>, ins: Option<String>, page: Option<String>, resource_type: String) -> String {
    match (ns, ins) {
        (None, None) => {
            with_path(&format!("/kapis/{}", resource_type), page)
        }
        (None, Some(s)) => {
            with_path(&format!("/kapis/namespaces/default/{}/{}", resource_type, s), page)
        }
        (Some(s), None) => {
            with_path(&format!("/kapis/namespaces/{}/{}", s, resource_type), page)
        }
        (Some(s), Some(s1)) => {
            with_path(
                &format!("/kapis/namespaces/{}/{}/{}", s, resource_type, s1), page,
            )
        }
    }
}

