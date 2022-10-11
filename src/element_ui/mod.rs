pub(crate) mod input;
pub(crate) mod select;
pub(crate) mod table;


#[derive(Debug, Clone, PartialEq)]
pub struct ValueText {
    pub value: String,
    pub text: String,
}