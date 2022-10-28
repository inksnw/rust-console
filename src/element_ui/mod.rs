pub use button::*;
pub use checkbox::*;
pub use form::*;
pub use input::ElInput;
pub use inputnum::*;
pub use select::*;
pub use switch::*;
pub use table::*;

pub mod input;
pub mod select;
pub mod table;
pub mod form;
pub mod button;
pub mod inputnum;
pub mod switch;
pub mod checkbox;

#[derive(Debug, Clone, PartialEq)]
pub struct ValueText {
    pub value: String,
    pub text: String,
}