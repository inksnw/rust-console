pub(crate) mod el_input;
pub(crate) mod el_select;


#[derive(Debug, Clone, PartialEq)]
pub struct ValueText {
    pub value: String,
    pub text: String,
}