use serde_json;
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
    pub data: Vec<serde_json::Value>,


}


impl Component for ElTable {
    type Message = TableMsg;
    type Properties = TableProps;

    fn create(_ctx: &Context<Self>) -> Self {
        ElTable {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {

    <div class="el-table--fit el-table--striped el-table--enable-row-hover el-table--enable-row-transition el-table el-table--layout-fixed is-scrolling-none"
         data-prefix="el"  style="width: 100%;">
        <div class="el-table__inner-wrapper">
            <div class="el-table__header-wrapper">
                <table class="el-table__header" border="0" cellpadding="0" cellspacing="0" style="width: 806px;">
                    <colgroup>
                      {
                        for ctx.props().children.iter().map(|item|{
                            html!{<col width={item.props.width.clone()}/>}
                            })
                      }

                    </colgroup>
                    <thead class="">
                    <tr class="">
                       {
                           for ctx.props().children.iter()
                       }
                    </tr>
                    </thead>
                </table>
            </div>
            <div class="el-table__body-wrapper">
                <div class="el-scrollbar">
                    <div class="el-scrollbar__wrap el-scrollbar__wrap--hidden-default">
                        <div class="el-scrollbar__view" style="display: inline-block; vertical-align: middle;">
                            <table class="el-table__body" cellspacing="0" cellpadding="0" border="0"
                                   style="table-layout: fixed; width: 806px;">
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
                    </div>
                    <div class="el-scrollbar__bar is-horizontal" style="display: none;">
                        <div class="el-scrollbar__thumb" style="transform: translateX(0%);"></div>
                    </div>
                    <div class="el-scrollbar__bar is-vertical" style="display: none;">
                        <div class="el-scrollbar__thumb" style="transform: translateY(0%);"></div>
                    </div>
                </div>
            </div>
        </div>
    </div>

        }
    }
}

impl ElTable {
    //遍历行
    pub fn render_row(&self, ctx: &Context<Self>) -> Html {
        ctx.props().data.as_slice().iter().map(|row: &serde_json::Value| {
            html! {
                    <tr class="el-table__row el-table__row--striped">
                    {
                        ctx.props().children.iter().enumerate().map(|(i,item)|{
                        self.render_cell(row,item.props.prop.clone())
                    }).collect::<Html>()
                    }
                    </tr>}
        }).collect::<Html>()
    }
    fn render_cell(&self, row: &serde_json::Value, prop: String) -> Html {
        let empty_value = serde_json::Value::String(String::new());
        html! {
            <td class="el-table_2_column_6 el-table__cell" rowspan="1" colspan="1">
            <div class="cell">
            {row.get(prop).unwrap_or(&empty_value).
            as_str().unwrap_or("--")
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
