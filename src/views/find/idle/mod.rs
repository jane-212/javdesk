use gpui::*;
use scraper::selectable::Selectable;
use scraper::Html;

mod item;
mod selector;

use super::State;
use crate::components::ScrollOwned;
use crate::proxy::ProxyUrl;
use item::Item;
use selector::selectors;

#[derive(IntoElement, Clone)]
pub struct Idle {
    items: Model<Vec<Item>>,
    scroll: ScrollOwned,
}

impl Idle {
    const BASE_URL: &'static str = "https://www.javbus.com";

    pub fn new(cx: &mut WindowContext) -> Self {
        let items: Model<Vec<Item>> = cx.new_model(|_| Vec::new());

        let cur_items = items.clone();
        let list_state = cx.new_model(|cx| {
            ListState::new(cur_items.read(cx).len(), ListAlignment::Top, Pixels(0.0), {
                move |i, cx| cur_items.read(cx)[i].clone().into_any_element()
            })
        });

        Self {
            items,
            scroll: ScrollOwned::new(list_state),
        }
    }

    fn scroll(&mut self, delta: Pixels, cx: &mut WindowContext) {
        self.scroll.scroll(delta, cx);
    }

    pub fn change_to(&mut self, mut cur_items: Vec<Item>, cx: &mut WindowContext) {
        cx.update_model(&self.items, |items, _cx| {
            items.clear();
            items.append(&mut cur_items);
        });
        let items = self.items.clone();
        cx.update_model(&self.scroll.state(), |state, cx| {
            let new_state =
                ListState::new(items.read(cx).len(), ListAlignment::Top, Pixels(0.0), {
                    move |i, cx| items.read(cx)[i].clone().into_any_element()
                });

            *state = new_state;
        });
    }

    pub fn parse(doc: String) -> (i32, Vec<Item>) {
        let html = Html::parse_document(&doc);

        let Some(high) = html.select(&selectors().page).next().and_then(|page| {
            page.attr("totalpage")
                .and_then(|attr| attr.parse::<i32>().ok())
        }) else {
            return (1, Vec::new());
        };

        let mut items = Vec::new();
        html.select(&selectors().items).for_each(|item| {
            let Some(id) = item.attr("id").and_then(|id| {
                if id.starts_with("normalthread") {
                    id.split_once('_')
                        .and_then(|(_, id)| id.parse::<i32>().ok())
                } else {
                    None
                }
            }) else {
                return;
            };

            let Some(avatar) = item
                .select(&selectors().avatar)
                .next()
                .and_then(|avatar| avatar.attr("src").map(|avatar| avatar.to_string()))
            else {
                return;
            };

            let Some(title) = item
                .select(&selectors().title)
                .next()
                .map(|title| title.inner_html())
            else {
                return;
            };

            let Some(name) = item
                .select(&selectors().name)
                .next()
                .map(|name| name.inner_html())
            else {
                return;
            };

            let Some(date) = item
                .select(&selectors().date)
                .next()
                .map(|origin| {
                    (
                        origin,
                        origin
                            .select(&selectors().span)
                            .next()
                            .and_then(|date| date.attr("title").map(|date| date.to_string())),
                    )
                })
                .map(|(origin, date)| match date {
                    Some(date) => date,
                    None => origin.inner_html(),
                })
            else {
                return;
            };

            let Some(view) = item
                .select(&selectors().view)
                .next()
                .and_then(|view| view.inner_html().parse::<i32>().ok())
            else {
                return;
            };

            let Some(reply) = item
                .select(&selectors().reply)
                .next()
                .and_then(|reply| reply.inner_html().parse::<i32>().ok())
            else {
                return;
            };

            let samples = item
                .select(&selectors().samples)
                .flat_map(|sample| {
                    sample.attr("src").map(|src| {
                        ProxyUrl::Home(format!("{}/forum/{}", Self::BASE_URL, src)).to_string()
                    })
                })
                .collect();

            items.push(Item::new(
                id,
                ProxyUrl::Avatar(avatar).to_string(),
                title,
                name,
                date,
                view,
                reply,
                samples,
            ));
        });

        (high, items)
    }
}

impl RenderOnce for Idle {
    fn render(self, _cx: &mut WindowContext) -> impl IntoElement {
        div()
            .h_full()
            .w_full()
            .flex()
            .justify_center()
            .items_center()
            .child(self.scroll)
            .on_scroll_wheel(move |event, cx| {
                let delta = event.delta.pixel_delta(Pixels(1.0)).y;
                cx.update_global::<State, ()>(|state, cx| {
                    state.idle_mut().scroll(delta, cx);
                    cx.refresh();
                });
            })
    }
}
