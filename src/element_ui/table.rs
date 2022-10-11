use serde_json;
use serde_json::Value;
use yew::{html, Properties};
use yew::prelude::*;

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
    fn render_cell(&self, row: &Value, prop: String) -> Html {
        let empty_value = Value::String(String::new());
        html! {
            <td rowspan="1" colspan="1" class="el-table_2_column_6   el-table__cell">
            <div class="cell">
            {
                self.get_json_value(prop,row,&empty_value)
            }
            </div>
            </td>
        }
    }

    fn get_json_value(&self, query: String, data: &Value, empty: &Value) -> String {


        let query_list: Vec<&str> = query.split(".").collect();
        if query_list.len() <= 0 {
            "--".to_string();
        }
        let mut first: Option<&Value> = data.get(query_list.get(0).unwrap());
        query_list.iter().enumerate().for_each(|(i, key)| {
            if i > 0 {
                match key.parse::<usize>() {
                    Ok(k) => {
                        first = first.and_then(|v| v.get(k));
                    }
                    _ => {
                        first = first.and_then(|v| v.get(key));
                    }
                }
            }
        });
        return first.unwrap_or(empty).as_str().unwrap_or("--").to_string();
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
