use yew::{html, Properties};
use yew::prelude::*;

// INPUT 组件
#[derive(Properties, Clone, PartialEq)]
pub struct CheckBoxProps {
    #[prop_or_default]
    pub value: bool,

    #[prop_or_default]
    pub style: &'static str,    //这里直接加入style 设置， 懒得 用width了

    #[prop_or(false)]
    pub disabled: bool,

    #[prop_or_default]
    pub label: &'static str,

    pub onchange: Option<Callback<String>>,
}

pub enum ElCheckBoxMsg {
    Edit
}

pub struct ElCheckBox {
    value: bool,
}

impl Component for ElCheckBox {
    type Message = ElCheckBoxMsg;
    type Properties = CheckBoxProps;
    fn create(_ctx: &Context<Self>) -> Self {
        Self { value: _ctx.props().value }
    }
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            ElCheckBoxMsg::Edit => {
                self.value = !self.value;

                if let Some(call) = ctx.props().onchange.clone() {
                    call.emit(self.value.to_string());
                }
                true
            }
        }
    }
    fn view(&self, _ctx: &Context<Self>) -> Html {
        html!(
        <div class="el-input">
        <label class={self.get_checkclass("el-checkbox")}>
          <span class={self.get_checkclass("el-checkbox__input")}>
        <span class="el-checkbox__inner"></span>
           <input type="checkbox" checked={self.value}  class="el-checkbox__original" 
             onclick={_ctx.link().callback(|_| ElCheckBoxMsg::Edit)} />
        </span>
         <span class="el-checkbox__label">{_ctx.props().label}</span>
         </label>
        </div>
    )
    }
}

impl ElCheckBox {
    fn get_checkclass(&self, initstr: &str) -> String {
        let mut class = String::from(initstr);
        if self.value {
            class.push_str(" is-checked");
        }
        class
    }
}
// checkbox 组件 ----end --------------------------------------------------------------

 
 