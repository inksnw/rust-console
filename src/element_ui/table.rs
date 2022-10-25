use serde_json;
use serde_json::Value;
use yew::{html, Properties};
use yew::prelude::*;

use crate::helper::utils::get_json_value;

pub struct ElTable {}

pub enum TableMsg {}

#[derive(PartialEq, Properties)]
pub struct TableProps {
    #[prop_or(String::from("100%"))]
    pub width: String,
    #[prop_or_default]
    pub children: ChildrenWithProps<ElTableColumn>,
    #[prop_or_default]
    pub data: Vec<Value>,

}


impl Component for ElTable {
    type Message = TableMsg;
    type Properties = TableProps;

    fn create(_ctx: &Context<Self>) -> Self {
        ElTable {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {


        <div class="el-table el-table--fit el-table--striped el-table--enable-row-hover el-table--enable-row-transition"
             style="width: 100%;">
            <div class="hidden-columns">
                <div></div>
                <div></div>
                <div></div>
            </div>
            <div class="el-table__header-wrapper">
                <table cellspacing="0" cellpadding="0" border="0" class="el-table__header" style="width: 497px;">
                    <colgroup>
                        {
                        for ctx.props().children.iter().map(|item|{
                        html!{<col width={item.props.width.clone()}/>}
                        })
                        }
                    </colgroup>
                    <thead class="has-gutter">
                    <tr class="">
                        {
                        for ctx.props().children.iter()
                        }
                    </tr>
                    </thead>
                </table>
            </div>
            <div class="el-table__body-wrapper is-scrolling-none">
                <table cellspacing="0" cellpadding="0" border="0" class="el-table__body" style="width: 497px;">
                    <colgroup>
                        {
                        for ctx.props().children.iter().map(|item|{
                        html!{<col width={item.props.width.clone()}/>}
                        })
                        }
                    </colgroup>
                    <tbody>

                    { self.render_row(ctx) }

                    </tbody>
                </table>
            </div>
            <div class="el-table__column-resize-proxy" style="display: none;"></div>
        </div>

        }
    }
}

impl ElTable {
    //遍历行
    pub fn render_row(&self, ctx: &Context<Self>) -> Html {
        ctx.props().data.as_slice().iter().map(|row: &Value| {
            html! {
                    <tr class="el-table__row">
                    {
                        ctx.props().children.iter().enumerate().map(|(_i,item)|{
                        self.render_cell(row,item.props.prop.clone())
                        }).collect::<Html>()
                    }
                    </tr>}
        }).collect::<Html>()
    }
    fn render_cell(&self, row: &Value, query: String) -> Html {
        let empty_value = Value::String(String::new());
        let value = get_json_value(&query, row, &empty_value);
        let namespace = get_json_value("metadata.namespace", row, &empty_value);
        let kind = get_json_value("kind", row, &empty_value);
        let url = if namespace == "" { format!("{}/{}", kind, value) } else { format!("{}/{}/{}", kind, namespace, value) };


        html! {
            <td rowspan="1" colspan="1" class="el-table_2_column_6   el-table__cell">
            <div class="cell">

             if query=="metadata.name"{
                     <a href={ url }> {value} </a>
                }else{
                    {value}
                }
            </div>
            </td>
        }
    }
}

pub struct ElTableColumn {}

#[derive(PartialEq, Properties)]
pub struct TableColumnProps {
    #[prop_or(String::from("100"))]
    pub width: String,
    #[prop_or_default]
    pub label: String,
    #[prop_or_default]
    pub prop: String,
}

impl Component for ElTableColumn {
    type Message = ();
    type Properties = TableColumnProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
             <th class="el-table_2_column_5 is-leaf el-table__cell" colspan="1" rowspan="1">
              <div class="cell">{ctx.props().label.clone()}</div>
             </th>
        }
    }
}
