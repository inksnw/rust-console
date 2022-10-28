#![allow(dead_code)]

use uuid::Uuid;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;

use super::js::{create_element, get_body, get_by_id};

// 大概的样式如下
// <div class={format!("el-message is-closable el-message--{}",_ctx.props().msg_type)} 
//        style="top: 20px; z-index: 2020;transform: translateY(100%)">
//           <i class="el-message__icon el-icon-success"></i>
//           <p class="el-message__content">这里填的是text</p>
//            <i class="el-message__closeBtn el-icon-close"></i>
//         </div>
// 弹出成功消息框
fn message(msg_type: &str, msg: &str) {
    let body = get_body().unwrap();
    // msg_type info 、success、warning、error 四种
    let div = create_element("div").expect("create div error");
    let div_class_name = format!("el-message is-closable el-message--{}", msg_type);
    let div_id = Uuid::new_v4();
    div.set_id(div_id.to_string().as_str());
    div.set_class_name(div_class_name.as_str());
    div.set_attribute("style", "top: 20px; z-index: 2020;").expect("style set error");

    let i1 = create_element("i").expect("create i error");
    let i1_class_name = format!("el-message__icon el-icon-{}", msg_type);
    i1.set_class_name(i1_class_name.as_str());

    let text = create_element("p").expect("create p error");
    text.set_class_name("el-message__content");
    text.set_inner_text(msg);

    let i2 = create_element("i").expect("create i error");
    i2.set_class_name("el-message__closeBtn el-icon-close");  //这个 i 可以设置关闭事件 
    i2.set_attribute("role", div_id.to_string().as_str()).expect("i2 set role error");

    let close_func = Closure::wrap(Box::new(|e: web_sys::Event| {
        let get_id = e
            .current_target()
            .unwrap()
            .dyn_into::<web_sys::HtmlElement>()
            .unwrap().get_attribute("role").unwrap();

        let get_child = get_by_id(&get_id).unwrap();

        get_body().unwrap().remove_child(&get_child.dyn_into::<web_sys::Node>().
            unwrap()).unwrap();
    }) as Box<dyn FnMut(_)>);


    i2.add_event_listener_with_callback("click", &close_func.as_ref().unchecked_ref()).expect("add click error");
    close_func.forget();//这句一定要加


    div.append_child(&i1).expect("append error i1");
    div.append_child(&text).expect("append error text");
    div.append_child(&i2).expect("append error i2");

    body.append_child(&div).unwrap();
}

pub fn success(msg: &str) {
    message("success", msg);
}

pub fn info(msg: &str) {
    message("info", msg);
}

pub fn error(msg: &str) {
    message("error", msg);
}

pub fn warning(msg: &str) {
    message("warning", msg);
}