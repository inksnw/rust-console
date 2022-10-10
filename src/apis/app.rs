use crate::element_ui::ValueText;

pub enum AppMsg {
    TestClick,
    UpdateMyName(String),
    UpdateNs(ValueText),
}
