use serde::{Deserialize, Serialize};
use yew::prelude::*;

use crate::element_ui::{ElButton, ElCheckBox, ElForm,
                        ElFormItem, ElInput, ElInputNumber, ElSwitch};
use crate::helper::js::log_str;
use crate::helper::message::error;

#[derive(Default, Serialize, Deserialize, Clone)]
struct FormData {
    name: String,
    age: i32,
    is_active: bool,
    is_admin: bool,
}

pub enum UpdateType {
    UpdateName,
    UpdateAge,
    UpdateActive,
    UpdateIsAdmin,
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
                    UpdateType::UpdateActive => {
                        self.data.is_active = text.parse::<bool>().unwrap_or(false);
                    }
                    UpdateType::UpdateIsAdmin => {
                        self.data.is_admin = text.parse::<bool>().unwrap_or(false);
                    }
                }
            }
            FormTestMsg::LogData => {
                let str = serde_json::to_string_pretty(&self.data).unwrap();
                log_str(str);
            }
        }
        true
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        html!(
          <div>
            <div>{"新增用户"}</div>
            <ElForm style={"margin-top:20px"}>
                <ElFormItem label="姓名">
                    <ElInput value={self.data.name.clone()} onchange={ctx.link().callback(|v| FormTestMsg::UpdateData(v,UpdateType::UpdateName))} />
                </ElFormItem>
                <ElFormItem label="年龄">
                    <ElInputNumber default={self.data.age}
                    onchange={ctx.link().callback(|v| FormTestMsg::UpdateData(v,UpdateType::UpdateAge))}/>
                </ElFormItem>
                <ElFormItem label="是否有效" label_witdh="80px">
                    <ElSwitch value={self.data.is_active}
                     onchange={ctx.link().callback(|v|
                        FormTestMsg::UpdateData(v,UpdateType::UpdateActive))}/>
                </ElFormItem>
                <ElFormItem label="角色">
                  <ElCheckBox label="管理员" value={self.data.is_admin}
                  onchange={ctx.link().callback(|v|
                    FormTestMsg::UpdateData(v,UpdateType::UpdateIsAdmin))}
                  />
               </ElFormItem>
            </ElForm>

            <ElButton value={"点我"} button_type={"primary"}
                onclick={ctx.link().callback(|_| FormTestMsg::LogData)}
                />
                <ElButton value={"消息框"} button_type={"default"}
                    onclick={Callback::from(|_|{error("错误消息 ")})}
                />

            </div>
        )
    }
}