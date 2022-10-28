use yew::{events::MouseEvent, html, Properties};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ElButtonProps {
    #[prop_or_default]
    pub value: &'static str,
    #[prop_or(false)]
    pub disabled: bool,
    #[prop_or("default")]
    pub button_type: &'static str,   // 和elementui 一样 ， primary / success / warning / danger / info / text

    #[prop_or("button")]
    pub native_type: &'static str,  // button / submit / reset  默认是button

    #[prop_or_default]
    pub onclick: Option<Callback<MouseEvent>>,
}

pub enum ElButtonMsg {
    Click(MouseEvent)
}

pub struct ElButton {}

impl Component for ElButton {
    type Message = ElButtonMsg;
    type Properties = ElButtonProps;
    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            ElButtonMsg::Click(e) => {
                if let Some(call) = ctx.props().onclick.clone() {
                    call.emit(e);
                }
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
        <button type="button" 
          onclick={ctx.link().callback(|e:MouseEvent| ElButtonMsg::Click(e))}
           class={format!("el-button el-button--{}",ctx.props().button_type)}>
          <span>{ctx.props().value}</span>
        </button>
        }
    }
}
 