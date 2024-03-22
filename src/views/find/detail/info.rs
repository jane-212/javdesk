use gpui::*;

use super::article::Article;
use super::page::Page;
use super::State;
use crate::components::Scroll;

#[derive(Clone, IntoElement)]
pub struct Info {
    title: String,
    page: Option<Page>,
    scroll: Scroll,
}

impl Info {
    pub fn new(title: String, page: Option<Page>, articles: Vec<Article>) -> Self {
        let list_state = ListState::new(articles.len(), ListAlignment::Top, Pixels(0.0), {
            move |i, _cx| articles[i].clone().into_any_element()
        });

        Self {
            title,
            page,
            scroll: Scroll::new(list_state),
        }
    }

    pub fn scroll(&mut self, delta: Pixels) {
        self.scroll.scroll(delta);
    }

    pub fn title(&self) -> String {
        self.title.clone()
    }
}

impl RenderOnce for Info {
    fn render(self, _cx: &mut WindowContext) -> impl IntoElement {
        div()
            .h_full()
            .w_full()
            .flex()
            .child(
                div()
                    .h_full()
                    .w_full()
                    .child(self.scroll)
                    .on_scroll_wheel(move |event, cx| {
                        let delta = event.delta.pixel_delta(Pixels(1.0));
                        let x = delta.x;

                        if x > Pixels(40.0) {
                            cx.update_global::<State, ()>(|state, cx| {
                                state.machine_mut().idle();
                                cx.refresh();
                            });
                            return;
                        }

                        let y = delta.y;

                        cx.update_global::<State, ()>(|state, cx| {
                            state.detail_mut().scroll(y);
                            cx.refresh();
                        });
                    }),
            )
            .child(match self.page {
                Some(page) => div().child(page),
                None => div(),
            })
    }
}
