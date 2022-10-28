use web_sys::HtmlInputElement;
use yew::{events::InputEvent, html, Properties};
use yew::prelude::*;

// INPUT 组件
#[derive(Properties, Clone, PartialEq)]
pub struct InputProps {
    #[prop_or_default]
    pub value: String,
    #[prop_or_default]
    pub style: &'static str,    //这里直接加入style 设置

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
    fn create(_ctx: &Context<Self>) -> Self {
        Self { text: _ctx.props().value.clone() }
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
    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
        <div class="el-input">
          <input disabled={_ctx.props().disabled}
            value={self.text.clone()} 
            oninput={_ctx.link().callback(|e:InputEvent| ElInputMsg::Edit(e))}
            class="el-input__inner"  
            style={_ctx.props().style}
            placeholder={_ctx.props().placeholder.clone()}/>

        </div>
    }
    }
}


 
 