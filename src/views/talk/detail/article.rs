use gpui::prelude::FluentBuilder;
use gpui::*;
use scraper::Html;

use crate::app_state::AppState;
use crate::proxy::ProxyUrl;
use crate::theme::Theme;

#[derive(Clone, IntoElement)]
pub struct Article {
    #[cfg(feature = "avatar")]
    avatar: String,
    name: String,
    content: Vec<Line>,
    replys: Vec<Reply>,
}

impl Article {
    const USER_HEIGHT: Pixels = Pixels(50.0);
    const PADDING: Pixels = Pixels(20.0);

    #[cfg(feature = "avatar")]
    pub fn new(avatar: String, name: String) -> Self {
        Self {
            avatar,
            name,
            content: Vec::new(),
            replys: Vec::new(),
        }
    }
    #[cfg(not(feature = "avatar"))]
    pub fn new(name: String) -> Self {
        Self {
            name,
            content: Vec::new(),
            replys: Vec::new(),
        }
    }

    pub fn parse(mut self, content: String) -> Self {
        enum Element {
            Text(String),
            Tag(String),
        }

        let mut elements = Vec::new();
        let mut line = String::new();
        let mut begin = true;
        for c in content.chars() {
            if c == '<' {
                let text = line.trim();
                if !text.is_empty() {
                    elements.push(Element::Text(text.to_string()));
                }
                line.clear();
                line.push(c);
                begin = false;
                continue;
            }

            if c == '>' {
                line.push(c);
                elements.push(Element::Tag(line));
                line = String::new();
                begin = true;
                continue;
            }

            line.push(c);
        }
        if !line.is_empty() {
            if begin {
                elements.push(Element::Text(line));
            } else {
                elements.push(Element::Tag(line));
            }
        }

        let mut lines = Vec::new();
        let mut line = String::new();
        for element in elements.into_iter() {
            match element {
                Element::Text(text) => {
                    line.push_str(&text);
                }
                Element::Tag(tag) => {
                    let html = Html::parse_fragment(&tag);
                    let Some(value) = html.root_element().child_elements().next() else {
                        continue;
                    };

                    match value.value().name() {
                        "img" => {
                            if let Some(src) = value.attr("src") {
                                if !src.starts_with("static") {
                                    lines.push(Line::Image(
                                        ProxyUrl::Talk(src.to_string()).to_string(),
                                    ));
                                }
                            }
                        }
                        "br" => {
                            let new_line = line.trim().replace("&nbsp;", " ").to_string();
                            if !line.is_empty() {
                                lines.push(Line::Text(new_line));
                                line.clear();
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
        let line = line.trim().replace("&nbsp;", " ").to_string();
        if !line.is_empty() {
            lines.push(Line::Text(line));
        }

        self.content = lines;

        self
    }

    pub fn set_replys(mut self, mut replys: Vec<Reply>) -> Self {
        self.replys.clear();
        self.replys.append(&mut replys);

        self
    }

    pub fn set_quote(mut self, quote: Option<String>) -> Self {
        if let Some(text) = quote.clone() {
            self.content = self
                .content
                .into_iter()
                .skip(text.lines().count())
                .rev()
                .chain(quote.map(Line::Quote))
                .rev()
                .collect();
        }

        self
    }
}

impl RenderOnce for Article {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        let reply_height = if !self.replys.is_empty() {
            self.replys
                .iter()
                .fold(Pixels::ZERO, |height, reply| height + reply.height())
                + Self::PADDING
        } else {
            Pixels::ZERO
        };

        div()
            .size_full()
            .flex()
            .justify_center()
            .items_center()
            .child(
                div()
                    .h_full()
                    .w_2_3()
                    .border_b_1()
                    .border_color(theme.border)
                    .pb(Self::PADDING)
                    .child(
                        div()
                            .h(Self::USER_HEIGHT + Self::PADDING)
                            .w_full()
                            .flex()
                            .items_center()
                            .child(
                                #[cfg(feature = "avatar")]
                                img(self.avatar)
                                    .size(Self::USER_HEIGHT)
                                    .rounded_full()
                                    .object_fit(ObjectFit::Fill)
                                    .border_1()
                                    .border_color(theme.border)
                                    .overflow_hidden(),
                                #[cfg(not(feature = "avatar"))]
                                div(),
                            )
                            .child(
                                div()
                                    .size_full()
                                    .flex()
                                    .items_center()
                                    .pl_2()
                                    .text_color(theme.name)
                                    .child(self.name),
                            ),
                    )
                    .child(
                        div()
                            .w_full()
                            .border_l_1()
                            .border_color(theme.main)
                            .pl_2()
                            .children(self.content),
                    )
                    .child(
                        div()
                            .pl_2()
                            .h(reply_height)
                            .w_full()
                            .when_else(
                                self.replys.is_empty(),
                                |this| this.pt(Pixels::ZERO),
                                |this| this.pt(Self::PADDING),
                            )
                            .children(self.replys),
                    ),
            )
    }
}

#[derive(Clone, IntoElement, Debug)]
enum Line {
    Image(String),
    Text(String),
    Quote(String),
}

impl Line {
    const HEIGHT: Pixels = Pixels(400.0);
}

impl RenderOnce for Line {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        match self {
            Line::Image(src) => div()
                .p_2()
                .w_full()
                .h(Self::HEIGHT)
                .child(img(src.clone()).size_full().rounded_md().overflow_hidden())
                .on_mouse_down(MouseButton::Left, move |_event, cx| {
                    let src = src.clone();
                    cx.update_global::<AppState, ()>(move |app_state, cx| {
                        app_state.open(src);
                        cx.refresh();
                    });
                }),
            Line::Text(text) => div().w_full().child(text.clone()).on_mouse_down(
                MouseButton::Left,
                move |_event, cx| {
                    let mut key = String::new();
                    let mut begin = false;
                    for c in text.chars() {
                        if c.is_ascii_alphanumeric() || c == '-' || c == ' ' {
                            begin = true;
                            key.push(c);
                            continue;
                        }

                        if begin {
                            break;
                        }
                    }

                    cx.open_url(&format!("https://missav.com/cn/search/{}", key.trim()));
                },
            ),
            Line::Quote(text) => div()
                .w_full()
                .child(text)
                .pl_2()
                .border_l_8()
                .border_color(theme.main),
        }
    }
}

#[derive(Clone, IntoElement)]
pub struct Reply {
    #[cfg(feature = "avatar")]
    avatar: String,
    name: String,
    content: String,
}

impl Reply {
    const HEIGHT: Pixels = Pixels(40.0);

    #[cfg(feature = "avatar")]
    pub fn new(avatar: String, name: String, content: String) -> Self {
        Self {
            avatar,
            name,
            content,
        }
    }
    #[cfg(not(feature = "avatar"))]
    pub fn new(name: String, content: String) -> Self {
        Self { name, content }
    }

    fn height(&self) -> Pixels {
        Self::HEIGHT
    }
}

impl RenderOnce for Reply {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        div()
            .w_full()
            .h(self.height())
            .flex()
            .items_center()
            .child(
                #[cfg(feature = "avatar")]
                img(self.avatar)
                    .size_6()
                    .min_w_6()
                    .rounded_full()
                    .overflow_hidden()
                    .object_fit(ObjectFit::Fill)
                    .border_1()
                    .border_color(theme.border),
                #[cfg(not(feature = "avatar"))]
                div(),
            )
            .child(div().text_color(theme.name).pl_2().child(self.name))
            .child(div().overflow_hidden().pl_2().child(self.content))
    }
}
