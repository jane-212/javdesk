use gpui::*;
use scraper::Html;

mod article;
mod info;
mod page;
mod selector;

use super::State;
#[cfg(feature = "avatar")]
use crate::proxy::ProxyUrl;
use article::{Article, Reply};
use info::Info;
use page::Page;
use selector::selectors;

#[derive(IntoElement, Clone)]
pub struct Detail {
    info: Option<Info>,
}

impl Detail {
    pub fn new() -> Self {
        Self { info: None }
    }

    pub fn parse(doc: String, id: i32, current: i32) -> Option<Info> {
        let html = Html::parse_document(&doc);

        let Some(title) = html.select(&selectors().title).next().map(|origin| {
            let mut title = String::new();
            for c in origin.inner_html().chars() {
                if c == '<' {
                    break;
                }

                title.push(c);
            }
            title.trim().to_string()
        }) else {
            return None;
        };

        #[cfg(feature = "avatar")]
        let post_avatar = html
            .select(&selectors().post_avatar)
            .next()
            .and_then(|avatar| avatar.attr("src"));
        let post_name = html
            .select(&selectors().post_name)
            .next()
            .map(|name| name.inner_html());

        let high = html.select(&selectors().page).next().and_then(|page| {
            page.inner_html()
                .chars()
                .filter(|c| c.is_numeric())
                .collect::<String>()
                .parse::<i32>()
                .ok()
        });

        let articles = html
            .select(&selectors().items)
            .fold(Vec::new(), |mut articles, item| {
                let Some(item) = item.select(&selectors().tr).nth(0) else {
                    return articles;
                };

                #[cfg(feature = "avatar")]
                let avatar = item
                    .select(&selectors().avatar)
                    .next()
                    .and_then(|avatar| avatar.attr("src"));
                #[cfg(feature = "avatar")]
                let avatar = match (avatar, post_avatar) {
                    (Some(avatar), None) | (Some(avatar), Some(_)) | (None, Some(avatar)) => {
                        ProxyUrl::Avatar(avatar.to_string()).to_string()
                    }
                    (None, None) => return articles,
                };

                let name = item
                    .select(&selectors().name)
                    .next()
                    .map(|name| name.inner_html());
                let name = match (&name, &post_name) {
                    (Some(name), None) | (Some(name), Some(_)) | (None, Some(name)) => {
                        name.to_string()
                    }
                    (None, None) => return articles,
                };

                let replys =
                    item.select(&selectors().replys)
                        .fold(Vec::new(), |mut replys, reply| {
                            #[cfg(feature = "avatar")]
                            let Some(avatar) = reply
                                .select(&selectors().reply_avatar)
                                .next()
                                .and_then(|avatar| {
                                    avatar.attr("src").map(|avatar| {
                                        ProxyUrl::Avatar(avatar.to_string()).to_string()
                                    })
                                })
                            else {
                                return replys;
                            };

                            let Some(name) = reply
                                .select(&selectors().reply_name)
                                .next()
                                .map(|name| name.inner_html())
                            else {
                                return replys;
                            };

                            let Some(content) = reply
                                .select(&selectors().reply_content)
                                .next()
                                .map(|origin| {
                                    let mut content = String::new();
                                    for c in origin.inner_html().chars() {
                                        if c == '<' {
                                            break;
                                        }

                                        content.push(c);
                                    }

                                    content
                                        .trim()
                                        .chars()
                                        .rev()
                                        .skip(6)
                                        .collect::<String>()
                                        .chars()
                                        .rev()
                                        .collect()
                                })
                            else {
                                return replys;
                            };

                            #[cfg(feature = "avatar")]
                            replys.push(Reply::new(avatar, name, content));
                            #[cfg(not(feature = "avatar"))]
                            replys.push(Reply::new(name, content));
                            replys
                        });

                let Some(content) = item
                    .select(&selectors().content)
                    .next()
                    .map(|content| content.inner_html())
                else {
                    return articles;
                };

                let quote = item.select(&selectors().quote).next().map(|quote| {
                    quote
                        .text()
                        .flat_map(|text| text.chars())
                        .collect::<String>()
                });

                articles.push(
                    #[cfg(feature = "avatar")]
                    Article::new(avatar, name)
                        .parse(content)
                        .set_replys(replys)
                        .set_quote(quote),
                    #[cfg(not(feature = "avatar"))]
                    Article::new(name)
                        .parse(content)
                        .set_replys(replys)
                        .set_quote(quote),
                );
                articles
            });

        Some(Info::new(
            title,
            high.map(|high| {
                let mut page = Page::new(id);
                page.to(current);
                page.set_high(high);
                page
            }),
            articles,
        ))
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
