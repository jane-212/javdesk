use gpui::*;

use super::State;
use crate::theme::Theme;

const SIZE: Pixels = Pixels(40.0);
const PADDING: Pixels = Pixels(30.0);
const MARGIN: Pixels = Pixels(8.0);

#[derive(IntoElement, Clone)]
pub struct Page {
    current: i32,
    low: i32,
    high: i32,
    pages: Vec<PageItem>,
}

impl Page {
    pub fn new() -> Self {
        Self {
            current: 1,
            low: 1,
            high: 1,
            pages: Vec::new(),
        }
    }

    pub fn to(&mut self, page: i32) {
        self.current = page;
    }

    pub fn set(&mut self, low: i32, high: i32) {
        self.low = low;
        self.high = high;
        self.pages.clear();
        self.pages.append(&mut self.pages());
    }

    fn pages(&self) -> Vec<PageItem> {
        let mut pages = Vec::new();

        for i in self.low..=self.high {
            if i == self.current {
                pages.push(PageItem::Current(i));
                continue;
            }

            pages.push(PageItem::Normal(i));
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
    Current(i32),
    Normal(i32),
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
            PageItem::Current(page) => view.text_color(theme.main).child(page.to_string()),
            PageItem::Normal(page) => view
                .child(page.to_string())
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
