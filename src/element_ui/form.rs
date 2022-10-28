use yew::{Children, html, Properties};
//-----------------------------------ELForm
use yew::function_component;
use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub struct ElFormItem {}

pub enum ElFormItemMsg {}

#[derive(Properties, PartialEq)]
pub struct ElFormItemProps {
    #[prop_or_default]
    pub label: String,
    #[prop_or("80px")]
    pub label_witdh: &'static str,
    #[prop_or_default]
    pub children: Children,
}

impl Component for ElFormItem {
    type Message = ElFormItemMsg;
    type Properties = ElFormItemProps;
    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }
    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }
    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            _ctx.props().children.iter().map(|child|{
                html!{
                    <div class="el-form-item">
                      <label class="el-form-item__label"
                        style={format!("width:{}",_ctx.props().label_witdh)}>
                          {_ctx.props().label.clone()}
                      </label>
                      <div class="el-form-item__content" style={format!("margin-left:{}",_ctx.props().label_witdh)}>
                          {child}
                      </div>

                    </div>
                }
            }).collect::<Html>()
    }
    }
}

#[derive(Properties, Clone, PartialEq)]
pub struct ElFormProps {
    #[prop_or_default]
    pub style: &'static str,
    #[prop_or_default]
    pub children: ChildrenWithProps<ElFormItem>,
}

#[function_component(ElForm)]
pub fn el_form(props: &ElFormProps) -> Html {
    html! {
         <form class="el-form" style={props.style.clone()}>
         {  props.children.iter().map(|child|{
            html!{
                {child}
            }
         }).collect::<Html>()
        }
        </form>
    }
}