use yew::{Component, Context, create_portal, Html, html};
use yew::Properties;

#[derive(PartialEq, Properties)]
pub struct ModalProperties {
    pub visible: bool,
    // pub onclose: Option<Callback<ValueText>>,
}

pub struct Modal {}

pub enum ModalMessage {
    Hide,
}

impl Component for Modal {
    type Message = ModalMessage;
    type Properties = ModalProperties;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }
    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            ModalMessage::Hide => {
                log::debug!("点击了关闭");
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let body = gloo_utils::document().body().unwrap();
        let host = gloo_utils::document().create_element("div").expect("can create inner wrapper");
        body.append_child(&host).unwrap();
        match ctx.props().visible {
            true => create_portal(self.create_box(ctx, "".to_string()), host),
            false => create_portal(self.create_box(ctx, "display:none;".to_string()), host)
        }
    }
}

impl Modal {
    fn create_box(&self, ctx: &Context<Self>, visible_style: String) -> Html {
        // display: none;
        html!(
            <div class="el-dialog__wrapper" style="z-index: 2254;        display: none;">
                <div role="dialog" aria-modal="true" aria-label="收货地址" class="el-dialog" style="margin-top: 15vh;">
                    <div class="el-dialog__header"><span class="el-dialog__title">{"收货地址"}</span>
                        <button type="button" aria-label="Close" class="el-dialog__headerbtn"><i
                                class="el-dialog__close el-icon el-icon-close"></i></button>
                    </div>
                    <div class="el-dialog__body">
                        <form class="el-form">
                            <div class="el-form-item"><label class="el-form-item__label" style="width: 120px;">{"活动名称"}</label>
                                <div class="el-form-item__content" style="margin-left: 120px;">
                                </div>
                            </div>

                        </form>
                    </div>
                    <div class="el-dialog__footer">
                        <div class="dialog-footer">
                            <button type="button" class="el-button el-button--default"><span>{"取 消"}</span></button>
                            <button type="button" class="el-button el-button--primary"><span>{"确 定"}</span></button>
                        </div>
                    </div>
                </div>
            </div>

    )
    }
}
