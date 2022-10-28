use web_sys::HtmlInputElement;
use yew::{events::InputEvent, html, Properties};
use yew::prelude::*;

// 计数器组件
#[derive(Properties, Clone, PartialEq)]
pub struct InputNumProps {
    #[prop_or_default]
    pub default: i32,

    #[prop_or_default]
    pub style: &'static str,    //这里直接加入style 设置， 懒得 用width了

    #[prop_or(false)]
    pub disabled: bool,

    #[prop_or_default]
    pub min: i32,

    #[prop_or(100)]
    pub max: i32,

    pub onchange: Option<Callback<String>>,
}

pub enum ElInputNumMsg {
    Incr,
    Decr,
    Change(InputEvent),
}

pub struct ElInputNumber {
    pub value: i32,
}

impl Component for ElInputNumber {
    type Message = ElInputNumMsg;
    type Properties = InputNumProps;
    fn create(_ctx: &Context<Self>) -> Self {
        Self { value: _ctx.props().default }
    }
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            ElInputNumMsg::Incr => {
                if self.value < ctx.props().max {
                    self.value += 1;
                    self.change(ctx);
                }

                true
            }
            ElInputNumMsg::Decr => {
                if self.value > ctx.props().min {
                    self.value -= 1;
                    self.change(ctx);
                }
                true
            }
            ElInputNumMsg::Change(e) => {
                let input = e.target_unchecked_into::<HtmlInputElement>();
                let new_value = input.value().parse::<i32>().unwrap_or(self.value);
                if self.value != new_value {
                    self.value = new_value;
                    self.change(ctx)
                }

                true
            }
        }
    }
    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
      <div class="el-input-number" style={_ctx.props().style.clone()}>
        <span role="button" class="el-input-number__decrease" onclick={_ctx.link().callback(|_| ElInputNumMsg::Decr)}>
          <i class="el-icon-minus" >
          </i>
        </span>
        <span role="button" class="el-input-number__increase" onclick={_ctx.link().callback(|_| ElInputNumMsg::Incr)}>
          <i class="el-icon-plus" ></i>
        </span>
       <div class="el-input">
         <input   disabled={_ctx.props().disabled} type="text" 
            oninput={_ctx.link().callback(|e:InputEvent| ElInputNumMsg::Change(e))}
            autocomplete="off" value={self.value.to_string()}  class="el-input__inner" />
       </div>
    </div>
    }
    }
}

impl ElInputNumber {
    fn change(&self, ctx: &Context<Self>) {
        if let Some(call) = ctx.props().onchange.clone() {
            call.emit(self.value.to_string());
        }
    }
}
// INPUT number 组件 ----end --------------------------------------------------------------

 
 