use web_sys::HtmlInputElement;
use yew::{events::InputEvent, html, Properties};
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct InputProps {
    #[prop_or(String::new())]
    pub value: String,
    #[prop_or(false)]
    pub disabled: bool,
    #[prop_or(String::from("请输入内容..."))]
    pub placeholder: String,
    pub onchange: Option<Callback<String>>,
}

pub enum ElInputMsg {
    Edit(InputEvent)
}

pub struct ElInput {
    text: String,
}

impl Component for ElInput {
    type Message = ElInputMsg;
    type Properties = InputProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            text: ctx.props().value.clone()
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            ElInputMsg::Edit(e) => {
                let input = e.target_unchecked_into::<HtmlInputElement>();
                self.text = input.value();
                if let Some(call) = ctx.props().onchange.clone() {
                    call.emit(input.value());
                }
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
        <div class="el-input">
            <input disabled={ctx.props().disabled}
            value={self.text.clone()}
            oninput={ctx.link().callback(|e:InputEvent|ElInputMsg::Edit(e))}
            class="el-input__inner"
            placeholder={ctx.props().placeholder.clone()}/>
        </div>
        }
    }
}