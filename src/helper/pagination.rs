use gloo::console::log;
use serde::{Deserialize, Serialize};
use yew::prelude::*;
use yew_router::prelude::*;

use crate::helper::router::Route;

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct PageQuery {
    pub page: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, Properties)]
pub struct Props {
    pub page: u64,
    pub total_pages: u64,
    pub route_to_page: Route,
}

pub struct Pagination;

impl Component for Pagination {
    type Message = ();
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let Props {
            page,
            total_pages,
            route_to_page: to,
        } = ctx.props().clone();


        html! {
           <div class="el-pagination">
           <ul class="pagination-list">
             <Link<Route, PageQuery>
                    disabled={page==1}
                    query={Some(PageQuery{page: page - 1})}
                    to={to.clone()}
                >
            if page!=1{
                  <button type="button" class="btn-prev" disabled={page==1}>
                 <i class="el-icon el-icon-arrow-left"></i>
                 </button>
            }


            </Link<Route, PageQuery>>

           <ul class="el-pager">{ self.view_links(ctx.props()) }</ul>

            <Link<Route, PageQuery>
                    disabled={page==total_pages}
                    query={Some(PageQuery{page: page + 1})}
                    {to}
                >
                if page!=total_pages{
                  <button type="button" class="btn-next" disabled={page==total_pages}>
                <i class="el-icon el-icon-arrow-right"></i>
                </button>
                }


            </Link<Route, PageQuery>>

            </ul>
           </div>
        }
    }
}

impl Pagination {
    fn render_link(&self, to_page: u64, props: &Props) -> Html {
        let Props {
            page,
            route_to_page,
            ..
        } = props.clone();

        let is_current_class = if to_page == page { true } else { false };

        html! {
            <li>
                <Link<Route, PageQuery> to={route_to_page} query={Some(PageQuery{page: to_page})}>
                    if is_current_class{
                         <li class="number active" >  { to_page }</li>
                    }else{
                          <li class="number">  { to_page }</li>
                    }
                </Link<Route, PageQuery>>
            </li>
        }
    }

    fn render_links<P>(&self, mut pages: P, len: usize, max_links: usize, props: &Props) -> Html
        where
            P: Iterator<Item=u64> + DoubleEndedIterator,
    {
        if len > max_links {
            let last_link = self.render_link(pages.next_back().unwrap(), props);
            // remove 1 for the ellipsis and 1 for the last link
            let links = pages
                .take(max_links - 2)
                .map(|page| self.render_link(page, props));
            html! {
                <>
                    { for links }
                     <li class="el-icon more btn-quicknext el-icon-more"></li>
                    { last_link }
                </>
            }
        } else {
            html! { for pages.map(|page| self.render_link(page, props)) }
        }
    }

    fn view_links(&self, props: &Props) -> Html {
        const LINKS_PER_SIDE: usize = 3;

        let Props {
            page, total_pages, ..
        } = *props;

        let pages_prev = page.checked_sub(1).unwrap_or_default() as usize;
        let pages_next = (total_pages - page) as usize;

        let links_left = LINKS_PER_SIDE.min(pages_prev)
            // if there are less than `LINKS_PER_SIDE` to the right, we add some more on the left.
            + LINKS_PER_SIDE.checked_sub(pages_next).unwrap_or_default();
        let links_right = 2 * LINKS_PER_SIDE - links_left;

        html! {
            <>
                { self.render_links(1..page, pages_prev, links_left, props) }
                { self.render_link(page, props) }
                { self.render_links(page + 1..=total_pages, pages_next, links_right, props) }
            </>
        }
    }
}
