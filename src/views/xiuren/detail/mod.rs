use gpui::*;
use scraper::Html;

mod info;
mod selector;

use crate::proxy::ProxyUrl;

use super::State;
use info::Info;
use selector::selectors;

#[derive(IntoElement, Clone)]
pub struct Detail {
    info: Option<Info>,
}

impl Detail {
    pub fn new() -> Self {
        Self { info: None }
    }

    pub fn parse(doc: String) -> Option<Info> {
        let html = Html::parse_document(&doc);

        let Some(title) = html
            .select(&selectors().title)
            .next()
            .map(|title| title.inner_html())
        else {
            return None;
        };

        let images = html
            .select(&selectors().image)
            .skip(1)
            .flat_map(|image| {
                image
                    .attr("src")
                    .map(|src| ProxyUrl::Webp(src.to_string()).to_string())
            })
            .collect();

        Some(Info::new(title, images))
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
