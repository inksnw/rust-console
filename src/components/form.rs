use gloo::console::log;
use serde::{Deserialize, Serialize};
use yew::prelude::*;

use crate::element_ui::{ElButton, ElForm, ElFormItem, ElInput};

#[derive(Default, Serialize, Deserialize, Clone)]
struct FormData {
    name: String,
    age: i32,
}

pub enum UpdateType {
    UpdateName,
    UpdateAge,
}

// 组件对象
#[derive(Default, Clone)]
pub struct FormTest {
    data: FormData,
}

pub enum FormTestMsg {
    UpdateData(String, UpdateType),
    LogData,
}


impl Component for FormTest {
    type Message = FormTestMsg;
    type Properties = ();
    fn create(_ctx: &Context<Self>) -> Self {
        Self { ..Default::default() }
    }
    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            FormTestMsg::UpdateData(text, t) => {
                match t {
                    UpdateType::UpdateName => {
                        self.data.name = text;
                    }
                    UpdateType::UpdateAge => {
                        self.data.age = text.parse::<i32>().unwrap_or(0);
                    }
                }
            }
            FormTestMsg::LogData => {
                let str = serde_json::to_string_pretty(&self.data).unwrap();
                log!(str);
            }
        }
        true
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
          <div>
            <ElForm>
                <ElFormItem label="姓名">
                    <ElInput onchange={ctx.link().callback(|v| FormTestMsg::UpdateData(v,UpdateType::UpdateName))} />
                </ElFormItem>
                <ElFormItem label="年龄">
                    <ElInput onchange={ctx.link().callback(|v| FormTestMsg::UpdateData(v,UpdateType::UpdateAge))}/>
                </ElFormItem>
            </ElForm>
            <ElButton value={"点我"} button_type={"primary"} 
                onclick={ctx.link().callback(|_| FormTestMsg::LogData)}
                />
            </div>
        }
    }
}