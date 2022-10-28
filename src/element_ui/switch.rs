use yew::{html, Properties};
use yew::prelude::*;

// INPUT 组件
#[derive(Properties, Clone, PartialEq)]
pub struct SwitchProps {
    #[prop_or_default]
    pub value: bool,
    #[prop_or_default]
    pub style: &'static str,    //这里直接加入style 设置， 懒得 用width了

    #[prop_or(false)]
    pub disabled: bool,

    pub onchange: Option<Callback<String>>,
}

pub enum SwitchMsg {
    Edit
}

pub struct ElSwitch {
    value: bool,
}

impl Component for ElSwitch {
    type Message = SwitchMsg;
    type Properties = SwitchProps;
    fn create(_ctx: &Context<Self>) -> Self {
        Self { value: _ctx.props().value }
    }
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            SwitchMsg::Edit => {
                self.value = !self.value;
                if let Some(call) = ctx.props().onchange.clone() {
                    call.emit(self.value.to_string());
                }
                true
            }
        }
    }
    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
        <div class={self.get_checkclass_div(_ctx)} onclick={_ctx.link().callback(|_| SwitchMsg::Edit)}>
           <input type="checkbox" disabled={_ctx.props().disabled} class="el-switch__input"/>
           <span class="el-switch__core" style={self.get_swichclass(_ctx)}>
        </span>
        </div>
    }
    }
}

impl ElSwitch {
    fn get_checkclass_div(&self, _ctx: &Context<Self>) -> String {
        let mut class = String::from("el-switch");
        if self.value {
            class.push_str(" is-checked");
        }
        if _ctx.props().disabled {
            class.push_str(" is-disabled");
        }
        class
    }
    fn get_swichclass(&self, _ctx: &Context<Self>) -> String {
        if self.value {
            "width: 40px; border-color: rgb(19, 206, 102); background-color: rgb(19, 206, 102);".to_string()
        } else {
            "width: 40px;".to_string()
        }
    }
}
// INPUT 组件 ----end --------------------------------------------------------------

 
 