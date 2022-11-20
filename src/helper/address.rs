use std::collections::HashMap;

use crate::helper::router::Route;

pub fn get_route(kind: String, name: String, ns: String) -> Route {
    let mut scores = HashMap::new();
    scores.insert(String::from("deploy"), Route::Deploy);
    scores.insert(String::from("Pod"), Route::PodDetail { ns, id: name });

    match scores.get(&kind) {
        Some(t) => { t.clone() }
        _ => { Route::NotFound }
    }
}