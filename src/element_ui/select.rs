use yew::{html, Properties};
use yew::prelude::*;

use crate::element_ui::ValueText;

pub struct ElSelect {
    vt: ValueText,
    show: bool,
}

pub enum Msg {
    Toggle,
    Select(String, String),
}


#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub data: Vec<ValueText>,
    pub onchange: Option<Callback<ValueText>>,
}

impl Component for ElSelect {
    type Message = Msg;
    type Properties = Props;
    fn create(_ctx: &Context<Self>) -> Self {
        ElSelect {
            show: false,
            vt: ValueText { value: String::new(), text: String::new() },
        }
    }
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Toggle => {
                self.show = !self.show;
            }
            Msg::Select(v, t) => {
                self.vt.value = v;
                self.vt.text = t;
                if let Some(call) = ctx.props().onchange.clone() {
                    call.emit(self.vt.clone());
                }
                ctx.link().send_message(Msg::Toggle);
            }
        };
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let arrow = if self.show { "el-icon-arrow-down" } else { "el-icon-arrow-up" };

        html! {
            <div class="el-select">
             <div class="el-input el-input--suffix ">
             <input type="text" readonly=true autocomplete="off"
               placeholder="全部名称空间" value={self.vt.text.clone()}

               onclick={ctx.link().callback(|_| Msg::Toggle)}
               class="el-input__inner"/>
                <span class="el-input__suffix">
                    <span class="el-input__suffix-inner">
                    <i class={format!("el-select__caret  el-input__icon {}",arrow)}
                     onclick={ctx.link().callback(|_| Msg::Toggle)}
                    ></i>
                    </span>
                </span>
              </div>


            <div class="el-scrollbar el-popper el-select-dropdown "
            style={format!("position: absolute !important; left:0; top:35px;z-index: 2000;width:100%;")+{
                 if self.show{""}else{"display:none"}
            }}>
              <div class="el-select-dropdown__wrap"  >
                <ul class="el-scrollbar__view el-select-dropdown__list">
                {
                    for ctx.props().data.iter().map(|vt|{
                        let v=format!("{}",&vt.value);
                        let t=format!("{}",&vt.text);
                        html!{
                            <li class="el-select-dropdown__item" onclick={ctx.link().callback(move |_| Msg::Select(v.clone(),t.clone()))}>
                                <span>{&vt.text}</span>
                            </li>
                        }
                    })
                }
                </ul>
             </div>
            </div>
            </div>
        }
    }
}
