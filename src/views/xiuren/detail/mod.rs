use gpui::*;
use scraper::Html;

mod info;
mod selector;

use super::State;
use crate::proxy::ProxyUrl;
use info::Info;
use selector::selectors;

#[derive(IntoElement, Clone)]
pub struct Detail {
    info: Option<Info>,
}

impl Detail {
    const BASE_URL: &'static str = "https://www.javbus.com";

    pub fn new() -> Self {
        Self { info: None }
    }

    pub fn parse(doc: String, href: String) -> Option<Info> {
        let html = Html::parse_document(&doc);

        let Some(title) = html
            .select(&selectors().title)
            .next()
            .and_then(|title| title.attr("title").map(|attr| attr.to_string()))
        else {
            return None;
        };

        let Some(cover) = html.select(&selectors().cover).next().and_then(|cover| {
            cover
                .attr("src")
                .map(|attr| ProxyUrl::Home(format!("{}{}", Self::BASE_URL, attr)).to_string())
        }) else {
            return None;
        };

        let Some(id) = html
            .select(&selectors().id)
            .nth(1)
            .map(|id| id.inner_html())
        else {
            return None;
        };

        let Some(date) = html
            .select(&selectors().date)
            .nth(1)
            .map(|date| date.inner_html())
            .map(|origin| {
                let mut date = String::new();
                let mut is_begin = false;
                for c in origin.chars() {
                    if !is_begin && c.is_numeric() {
                        date.push(c);
                        is_begin = true;
                        continue;
                    }

                    if !is_begin {
                        continue;
                    }

                    if !c.is_numeric() && c != '-' {
                        break;
                    }

                    date.push(c);
                }
                date.trim().to_string()
            })
        else {
            return None;
        };

        let Some(cost) = html
            .select(&selectors().cost)
            .nth(2)
            .map(|cost| cost.inner_html())
            .map(|origin| {
                let mut cost = String::new();
                let mut is_begin = false;
                for c in origin.chars() {
                    if !is_begin && c.is_numeric() {
                        cost.push(c);
                        is_begin = true;
                        continue;
                    }

                    if !is_begin {
                        continue;
                    }

                    if !c.is_numeric() {
                        break;
                    }

                    cost.push(c);
                }
                cost.trim().parse::<i32>().unwrap_or_default()
            })
        else {
            return None;
        };

        let samples = html
            .select(&selectors().samples)
            .flat_map(|sample| {
                sample
                    .attr("src")
                    .map(|attr| ProxyUrl::Home(format!("{}{}", Self::BASE_URL, attr)).to_string())
            })
            .collect();

        Some(Info::new(title, id, cover, date, cost, samples, href))
    }

    pub fn show(&mut self, info: Info) {
        self.info = Some(info);
    }

    fn scroll(&mut self, delta: Pixels) {
        if let Some(info) = &mut self.info {
            info.scroll(delta);
        }
    }

    pub fn title(&self) -> String {
        self.info
            .clone()
            .map(|info| info.title())
            .unwrap_or("Detail".to_string())
    }
}

impl RenderOnce for Detail {
    fn render(self, _cx: &mut WindowContext) -> impl IntoElement {
        match self.info {
            Some(info) => div()
                .size_full()
                .flex()
                .justify_center()
                .items_center()
                .child(info),
            None => div(),
        }
    }
}
