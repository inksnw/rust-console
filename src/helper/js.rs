#![allow(dead_code)]

use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::{Document, window};
use web_sys::HtmlElement;

// window.alert
pub fn alert(str: &str) {
    window().unwrap().alert_with_message(str).unwrap();
}


// 获取 document
pub fn get_document() -> Result<Document, JsValue> {
    let window = web_sys::window().expect("error window");
    let document = window.document().expect("error document");
    Ok(document)
}

// 好比是getElementByid
pub fn get_by_id(id: &str) -> Result<web_sys::HtmlElement, JsValue> {
    let document = get_document().unwrap();
    let ele = document.get_element_by_id(id).
        expect("cann't found element").dyn_into::<web_sys::HtmlElement>().expect("error element");
    Ok(ele)
}

// document.body
pub fn get_body() -> Result<HtmlElement, JsValue> {
    let document = get_document().unwrap();
    let body = document.body().expect("error body");
    Ok(body)
}

// document.createElement
pub fn create_element(ele: &str) -> Result<HtmlElement, JsValue> {
    let document = get_document().unwrap();
    let ret = document.create_element(ele).unwrap().
        dyn_into::<web_sys::HtmlElement>()?;
    Ok(ret)
}