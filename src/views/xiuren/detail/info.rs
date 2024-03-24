use gpui::*;

use super::State;
use crate::{app_state::AppState, components::Scroll};

#[derive(IntoElement, Clone)]
pub struct Info {
    title: String,
    scroll: Scroll,
}

impl Info {
    const HEIGHT: Pixels = Pixels(1200.0);

    pub fn new(title: String, images: Vec<String>) -> Self {
        let list_state = ListState::new(images.len(), ListAlignment::Top, Pixels(0.0), {
            move |i, _| {
                let src = images[i].to_string();

                div()
                    .w_full()
                    .h(Self::HEIGHT)
                    .flex()
                    .justify_center()
                    .items_center()
                    .p_2()
                    .child(
                        div()
                            .h_full()
                            .w_2_3()
                            .child(
                                img(src.clone())
                                    .size_full()
                                    .rounded_md()
                                    .overflow_hidden()
                                    .object_fit(ObjectFit::Contain),
                            )
                            .on_mouse_down(MouseButton::Left, {
                                move |_event, cx| {
                                    let src = src.clone();
                                    cx.update_global::<AppState, ()>(move |app_state, cx| {
                                        app_state.open(src);
                                        cx.refresh();
                                    });
                                }
                            }),
                    )
                    .into_any()
            }
        });

        Self {
            title,
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
            .size_full()
            .flex()
            .justify_center()
            .items_center()
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
            })
    }
}
