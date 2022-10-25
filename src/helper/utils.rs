use serde_json::Value;

pub fn get_json_value(query: &str, data: &Value, empty: &Value) -> String {
    let query_list: Vec<&str> = query.split(".").collect();
    if query_list.len() <= 0 {
        "--".to_string();
    }
    let mut first: Option<&Value> = data.get(query_list.get(0).unwrap());
    query_list.iter().enumerate().for_each(|(i, key)| {
        if i > 0 {
            match key.parse::<usize>() {
                Ok(k) => {
                    first = first.and_then(|v| v.get(k));
                }
                _ => {
                    first = first.and_then(|v| v.get(key));
                }
            }
        }
    });
    return first.unwrap_or(empty).as_str().unwrap_or("--").to_string();
}