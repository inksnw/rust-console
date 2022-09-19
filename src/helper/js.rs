use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::console;
use web_sys::window;

pub fn alert(str: &str) {
    window().unwrap().alert_with_message(str).unwrap();
}

pub fn log(obj: &wasm_bindgen::JsValue) {
    console::log_1(obj);
}

pub fn log_str(str: String) {
    log(&wasm_bindgen::JsValue::from_str(str.as_str()));
}