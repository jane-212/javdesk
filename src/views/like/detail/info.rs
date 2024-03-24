use gpui::*;

use super::State;
use crate::{
    app_state::AppState,
    components::{Icon, IconName, Scroll},
    db::DB,
    theme::Theme,
};

#[derive(IntoElement, Clone)]
pub struct Info {
    id: String,
    scroll: Scroll,
}

impl Info {
    const TITLE_HEIGHT: Pixels = Pixels(90.0);
    const INFO_HEIGHT: Pixels = Pixels(30.0);
    const COVER_HEIGHT: Pixels = Pixels(500.0);
    const SAMPLE_HEIGHT: Pixels = Pixels(200.0);
    const PADDING: Pixels = Pixels(10.0);
    const FOLD: usize = 4;

    pub fn new(
        title: String,
        id: String,
        cover: String,
        date: String,
        cost: i32,
        samples: Vec<String>,
    ) -> Self {
        let list_state = ListState::new(1, ListAlignment::Top, Pixels(0.0), {
            let id = id.clone();

            move |_i, cx| {
                let theme = cx.global::<Theme>();
                let cover = cover.clone();
                let mut new_samples = Vec::new();
                let mut child = Vec::new();
                for sample in samples.clone() {
                    child.push(
                        div().p_4().w_1_4().h(Self::SAMPLE_HEIGHT).child(
                            img(sample)
                                .size_full()
                                .rounded_md()
                                .overflow_hidden()
                                .border_1()
                                .border_color(theme.border)
                                .object_fit(ObjectFit::Cover),
                        ),
                    );

                    if child.len() == Self::FOLD {
                        new_samples.push(child);
                        child = Vec::new();
                    }
                }
                if !child.is_empty() {
                    new_samples.push(child);
                }

                div()
                    .h_full()
                    .w_full()
                    .flex()
                    .justify_center()
                    .items_center()
                    .child(
                        div()
                            .h_full()
                            .w_2_3()
                            .child(
                                div()
                                    .w_full()
                                    .h(Self::COVER_HEIGHT)
                                    .pt_2()
                                    .child(
                                        img(cover.clone())
                                            .size_full()
                                            .rounded_md()
                                            .overflow_hidden()
                                            .border_1()
                                            .border_color(theme.border)
                                            .object_fit(ObjectFit::Cover),
                                    )
                                    .on_mouse_down(MouseButton::Left, move |_event, cx| {
                                        let cover = cover.clone();
                                        cx.update_global::<AppState, ()>(move |app_state, cx| {
                                            app_state.open(cover);
                                            cx.refresh();
                                        });
                                    }),
                            )
                            .child(
                                div()
                                    .overflow_hidden()
                                    .w_full()
                                    .font_weight(FontWeight::BOLD)
                                    .h(Self::TITLE_HEIGHT)
                                    .p_2()
                                    .child(title.clone()),
                            )
                            .child(
                                div()
                                    .flex()
                                    .w_full()
                                    .h(Self::INFO_HEIGHT * 3)
                                    .child(
                                        div()
                                            .w_4_5()
                                            .h_full()
                                            .child(
                                                div()
                                                    .w_full()
                                                    .h_1_3()
                                                    .flex()
                                                    .p_2()
                                                    .child(
                                                        div()
                                                            .mr_2()
                                                            .size_6()
                                                            .child(Icon::new(IconName::ID, true)),
                                                    )
                                                    .child(id.clone()),
                                            )
                                            .child(
                                                div()
                                                    .w_full()
                                                    .h_1_3()
                                                    .flex()
                                                    .p_2()
                                                    .child(
                                                        div()
                                                            .mr_2()
                                                            .size_6()
                                                            .child(Icon::new(IconName::Date, true)),
                                                    )
                                                    .child(date.clone()),
                                            )
                                            .child(
                                                div()
                                                    .w_full()
                                                    .h_1_3()
                                                    .flex()
                                                    .p_2()
                                                    .child(
                                                        div()
                                                            .mr_2()
                                                            .size_6()
                                                            .child(Icon::new(IconName::Cost, true)),
                                                    )
                                                    .child(format!("{} minutes", cost)),
                                            ),
                                    )
                                    .child(
                                        div()
                                            .h_full()
                                            .w_1_5()
                                            .flex()
                                            .justify_center()
                                            .items_center()
                                            .child(
                                                div()
                                                    .size(Self::INFO_HEIGHT * 3 - Self::PADDING * 4)
                                                    .rounded_lg()
                                                    .p(Self::PADDING)
                                                    .hover(|this| this.bg(theme.hover_background))
                                                    .child(Icon::new(IconName::Delete, true))
                                                    .on_mouse_down(MouseButton::Left, {
                                                        let id = id.clone();
                                                        move |_, cx| {
                                                            cx.global::<DB>().unlike(&id);
                                                            cx.update_global::<State, ()>(
                                                                |state, _| {
                                                                    state
                                                                        .machine_mut()
                                                                        .load_page(1);
                                                                },
                                                            );
                                                            cx.refresh();
                                                        }
                                                    }),
                                            ),
                                    ),
                            )
                            .children(new_samples.into_iter().map(move |sample| {
                                div()
                                    .w_full()
                                    .h(Self::SAMPLE_HEIGHT)
                                    .flex()
                                    .children(sample)
                            })),
                    )
                    .into_any()
            }
        });

        Self {
            id,
            scroll: Scroll::new(list_state),
        }
    }

    pub fn scroll(&mut self, delta: Pixels) {
        self.scroll.scroll(delta);
    }

    pub fn title(&self) -> String {
        self.id.clone()
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

                if x < Pixels(-40.0) {
                    cx.open_url(&format!("https://missav.com/cn/search/{}", self.id));
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
