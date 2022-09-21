pub mod test;

const BASE_URL: &str = "http://localhost:8081";

pub fn with_path(p: &str) -> String {
    format!("{}{}", BASE_URL, p)
}