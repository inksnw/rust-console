pub use button::ElButton;
pub use form::ElForm;
pub use form::ElFormItem;
pub use input::ElInput;

pub mod select;
pub mod table;
pub mod input;
pub mod form;
pub mod button;

#[derive(Debug, Clone, PartialEq)]
pub struct ValueText {
    pub value: String,
    pub text: String,
}