pub mod app;
pub mod namespace;

const BASE_URL: &str = "http://localhost:9090";

pub fn with_path(p: &str) -> String {
    format!("{}{}", BASE_URL, p)
}

pub fn name_space_api() -> String {
    with_path("/kapis/namespaces")
}