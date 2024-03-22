use gpui::*;
use scraper::selectable::Selectable;
use scraper::Html;

mod item;
mod selector;

use super::State;
use crate::components::Scroll;
use crate::proxy::ProxyUrl;
use item::Item;
use selector::selectors;

#[derive(IntoElement, Clone)]
pub struct Idle {
    items: Model<Vec<Item>>,
    scroll: Scroll,
}

impl Idle {
    const BASE_URL: &'static str = "https://www.javbus.com";

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

    pub fn parse(doc: String) -> (i32, i32, Vec<Item>) {
        let html = Html::parse_document(&doc);

        let mut low = i32::MAX;
        let mut high = i32::MIN;
        html.select(&selectors().pages)
            .flat_map(|page| page.inner_html().parse::<i32>().ok())
            .for_each(|page| {
                low = low.min(page);
                high = high.max(page);
            });

        let mut items = Vec::new();
        html.select(&selectors().items).for_each(|item| {
            let Some(href) = item.attr("href").map(|href| href.to_string()) else {
                return;
            };

            let Some(cover) = item
                .select(&selectors().cover)
                .next()
                .and_then(|cover| cover.attr("src").map(|src| src.to_string()))
            else {
                return;
            };

            let Some(title) = item
                .select(&selectors().title)
                .next()
                .and_then(|cover| cover.attr("title").map(|title| title.to_string()))
            else {
                return;
            };

            let Some(id) = item
                .select(&selectors().id)
                .nth(0)
                .map(|id| id.inner_html())
            else {
                return;
            };

            let Some(date) = item
                .select(&selectors().date)
                .nth(1)
                .map(|date| date.inner_html())
            else {
                return;
            };

            items.push(Item::new(
                href,
                ProxyUrl::Home(format!("{}{}", Self::BASE_URL, cover)).to_string(),
                title,
                id,
                date,
            ));
        });

        (low, high, items)
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
