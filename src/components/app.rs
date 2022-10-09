use k8s_openapi::api::core::v1::Namespace;
use yew::prelude::*;

use crate::apis::test::TestMsg;
use crate::element_ui::base::ElInput;
use crate::helper::js;

use super::selectns::NameSpaceSelect;

#[allow(dead_code)]
pub struct TestComp {
    ns_list: Vec<Namespace>,
}


impl Component for TestComp {
    type Message = TestMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            ns_list: vec![]
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            TestMsg::TestClick => {
                js::alert("按钮点击");
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
            <NameSpaceSelect />
            <ElInput value="abc"/>
            <button onclick={ctx.link().callback(|_| TestMsg::TestClick)}>{"点我"}  </button>
            </div>
            }
    }
}