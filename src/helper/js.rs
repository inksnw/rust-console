use web_sys::window;

pub fn alert(str: &str) {
    window().unwrap().alert_with_message(str).unwrap();
}

