use gpui::*;

use super::State;
use crate::theme::Theme;

const SIZE: Pixels = Pixels(40.0);
const PADDING: Pixels = Pixels(30.0);
const MARGIN: Pixels = Pixels(8.0);

#[derive(IntoElement, Clone)]
pub struct Page {
    current: i32,
    prev: bool,
    next: bool,
}

impl Page {
    pub fn new() -> Self {
        Self {
            current: 1,
            prev: false,
            next: false,
        }
    }

    pub fn to(&mut self, page: i32) {
        self.current = page;
    }

    pub fn set_prev(&mut self, prev: bool) {
        self.prev = prev;
    }

    pub fn set_next(&mut self, next: bool) {
        self.next = next;
    }

    fn pages(&self) -> Vec<PageItem> {
        let mut pages = Vec::new();

        if self.prev {
            pages.push(PageItem::Prev(self.current - 1));
        }

        if self.next {
            pages.push(PageItem::Next(self.current + 1));
        }

        pages
    }
}

impl RenderOnce for Page {
    fn render(self, _cx: &mut WindowContext) -> impl IntoElement {
        let pages = self.pages();

        div()
            .w(SIZE + PADDING * 2)
            .h_full()
            .flex()
            .items_center()
            .justify_center()
            .child(
                div()
                    .w(SIZE)
                    .h(pages.len() as f32 * (SIZE + MARGIN))
                    .children(pages),
            )
    }
}

#[derive(IntoElement)]
pub enum PageItem {
    Prev(i32),
    Next(i32),
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
            PageItem::Prev(page) => view
                .child("<")
                .hover(|s| s.bg(theme.hover_background))
                .on_mouse_down(MouseButton::Left, move |_event, cx| {
                    cx.update_global::<State, ()>(|state, cx| {
                        state.machine_mut().load_page(page);
                        cx.refresh();
                    });
                }),
            PageItem::Next(page) => view
                .child(">")
                .hover(|s| s.bg(theme.hover_background))
                .on_mouse_down(MouseButton::Left, move |_event, cx| {
                    cx.update_global::<State, ()>(|state, cx| {
                        state.machine_mut().load_page(page);
                        cx.refresh();
                    });
                }),
        }
    }
}
