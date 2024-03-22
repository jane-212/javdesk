use gpui::*;

use super::State;
use crate::theme::Theme;

const SIZE: Pixels = Pixels(40.0);
const PADDING: Pixels = Pixels(30.0);
const MARGIN: Pixels = Pixels(8.0);

#[derive(IntoElement, Clone)]
pub struct Page {
    id: i32,
    current: i32,
    high: i32,
    pages: Vec<PageItem>,
}

impl Page {
    const RANGE: i32 = 5;

    pub fn new(id: i32) -> Self {
        Self {
            id,
            current: 1,
            high: 1,
            pages: Vec::new(),
        }
    }

    pub fn to(&mut self, page: i32) {
        self.current = page;
    }

    pub fn set_high(&mut self, high: i32) {
        self.high = high;
        self.pages.clear();
        self.pages.append(&mut self.pages(self.id));
    }

    fn pages(&self, id: i32) -> Vec<PageItem> {
        let mut pages = Vec::new();

        for i in 1..=self.high {
            if i == self.current {
                pages.push(PageItem::Current(id, i));
                continue;
            }

            if i == 1 {
                pages.push(PageItem::Normal(id, i));
                if self.current - 1 > Self::RANGE {
                    pages.push(PageItem::Dot);
                }
                continue;
            }

            if i == self.high {
                if self.high - self.current > Self::RANGE {
                    pages.push(PageItem::Dot);
                }
                pages.push(PageItem::Normal(id, i));
                continue;
            }

            if (self.current - i).abs() < Self::RANGE {
                pages.push(PageItem::Normal(id, i));
            }
        }

        pages
    }
}

impl RenderOnce for Page {
    fn render(self, _cx: &mut WindowContext) -> impl IntoElement {
        div()
            .w(SIZE + PADDING * 2)
            .h_full()
            .flex()
            .items_center()
            .justify_center()
            .child(
                div()
                    .w(SIZE)
                    .h(self.pages.len() as f32 * (SIZE + MARGIN))
                    .children(self.pages),
            )
    }
}

#[derive(IntoElement, Clone)]
pub enum PageItem {
    Current(i32, i32),
    Dot,
    Normal(i32, i32),
}

impl RenderOnce for PageItem {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        let view = div()
            .size(SIZE)
            .flex()
            .justify_center()
            .items_center()
            .rounded_lg()
            .mt(MARGIN);
        match self {
            PageItem::Current(_, page) => view.text_color(theme.main).child(page.to_string()),
            PageItem::Normal(id, page) => view
                .child(page.to_string())
                .hover(|s| s.bg(theme.hover_background))
                .on_mouse_down(MouseButton::Left, move |_event, cx| {
                    cx.update_global::<State, ()>(|state, cx| {
                        state.machine_mut().load_detail(id, page);
                        cx.refresh();
                    });
                }),
            PageItem::Dot => view.child("..."),
        }
    }
}
