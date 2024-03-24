use gpui::prelude::FluentBuilder;
use gpui::*;
use scraper::selectable::Selectable;
use scraper::Html;
use serde::Deserialize;

mod item;
mod selector;

use super::State;
use crate::components::Scroll;
use item::Item;
use selector::selectors;

#[derive(IntoElement, Clone)]
pub struct Idle {
    items: Model<Vec<Item>>,
    scroll: Scroll,
}

impl Idle {
    pub fn new(cx: &mut WindowContext) -> Self {
        let items: Model<Vec<Item>> = cx.new_model(|_| Vec::new());

        let list_state = ListState::new(items.read(cx).len(), ListAlignment::Top, Pixels(0.0), {
            let items = items.clone();
            move |i, cx| items.read(cx)[i].clone().into_any_element()
        });

        Self {
            items,
            scroll: Scroll::new(list_state),
        }
    }

    fn scroll(&mut self, delta: Pixels) {
        self.scroll.scroll(delta);
    }

    pub fn change_to(&mut self, mut cur_items: Vec<Item>, cx: &mut WindowContext) {
        cx.update_model(&self.items, |items, _cx| {
            items.clear();
            items.append(&mut cur_items);
        });
        let items = self.items.clone();
        self.scroll = Scroll::new(ListState::new(
            items.read(cx).len(),
            ListAlignment::Top,
            Pixels(0.0),
            move |i, cx| items.read(cx)[i].clone().into_any_element(),
        ));
    }

    pub fn parse(res: Response) -> (bool, Vec<Item>) {
        let html = Html::parse_document(&res.content.replace("\\\"", "\""));

        let mut items = Vec::new();
        html.select(&selectors().items).for_each(|item| {
            let Some(href) = item
                .select(&selectors().href)
                .next()
                .and_then(|href| href.attr("href").map(|href| href.to_string()))
            else {
                return;
            };

            let Some(cover) = item
                .select(&selectors().cover)
                .next()
                .and_then(|cover| cover.attr("data-src").map(|src| src.to_string()))
            else {
                return;
            };

            let Some(title) = item
                .select(&selectors().title)
                .next()
                .and_then(|title| title.attr("alt").map(|title| title.to_string()))
            else {
                return;
            };

            let Some(view) = item.select(&selectors().view).next().map(|view| {
                view.text()
                    .fold(String::new(), |mut view, text| {
                        let text = text.trim();
                        if !text.is_empty() {
                            view.push_str(text);
                        }
                        view
                    })
                    .map(|view| view.trim().to_string())
            }) else {
                return;
            };

            let Some(date) = item.select(&selectors().date).next().map(|date| {
                date.text()
                    .fold(String::new(), |mut date, text| {
                        let text = text.trim();
                        if !text.is_empty() {
                            date.push_str(text);
                        }
                        date
                    })
                    .map(|date| date.trim().to_string())
            }) else {
                return;
            };

            items.push(Item::new(href, cover, title, view, date));
        });

        (res.next, items)
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
                    state.idle_mut().scroll(delta);
                    cx.refresh();
                });
            })
    }
}

#[derive(Deserialize, Debug)]
pub struct Response {
    content: String,
    prev: bool,
    next: bool,
}
