use gpui::*;

use super::State;
use crate::components::{Icon, IconName};
use crate::theme::Theme;

#[derive(IntoElement, Clone)]
pub struct Item {
    id: i32,
    #[cfg(feature = "avatar")]
    avatar: String,
    title: String,
    name: String,
    date: String,
    view: i32,
    reply: i32,
    samples: Vec<String>,
}

impl Item {
    const HEIGHT: Pixels = Pixels(200.0);

    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: i32,
        #[cfg(feature = "avatar")] avatar: String,
        title: String,
        name: String,
        date: String,
        view: i32,
        reply: i32,
        samples: Vec<String>,
    ) -> Self {
        Self {
            id,
            #[cfg(feature = "avatar")]
            avatar,
            title,
            name,
            date,
            view,
            reply,
            samples,
        }
    }
}

impl RenderOnce for Item {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        div()
            .w_full()
            .h(Self::HEIGHT)
            .flex()
            .justify_center()
            .items_center()
            .child(
                div()
                    .flex()
                    .p_4()
                    .w_2_3()
                    .h_full()
                    .rounded_lg()
                    .child(
                        #[cfg(feature = "avatar")]
                        div()
                            .h_full()
                            .w_1_3()
                            .child(
                                div()
                                    .w_full()
                                    .h_2_5()
                                    .flex()
                                    .justify_center()
                                    .items_center()
                                    .child(
                                        img(self.avatar)
                                            .object_fit(ObjectFit::Fill)
                                            .size_16()
                                            .rounded_full()
                                            .overflow_hidden(),
                                    ),
                            )
                            .child(
                                div()
                                    .h_3_5()
                                    .p_2()
                                    .w_full()
                                    .child(
                                        div()
                                            .w_full()
                                            .h_1_2()
                                            .flex()
                                            .justify_center()
                                            .items_center()
                                            .text_color(theme.name)
                                            .child(self.name),
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .items_center()
                                            .justify_center()
                                            .w_full()
                                            .h_1_2()
                                            .child(
                                                div()
                                                    .mr_2()
                                                    .size_6()
                                                    .child(Icon::new(IconName::Date, true)),
                                            )
                                            .child(self.date),
                                    ),
                            ),
                        #[cfg(not(feature = "avatar"))]
                        div().h_full().w_1_3().child(
                            div()
                                .h_1_2()
                                .p_2()
                                .w_full()
                                .child(
                                    div()
                                        .flex()
                                        .items_center()
                                        .justify_center()
                                        .w_full()
                                        .h_1_2()
                                        .text_color(theme.name)
                                        .child(self.name),
                                )
                                .child(
                                    div()
                                        .flex()
                                        .items_center()
                                        .justify_center()
                                        .w_full()
                                        .h_1_2()
                                        .child(
                                            div()
                                                .mr_2()
                                                .size_6()
                                                .child(Icon::new(IconName::Date, true)),
                                        )
                                        .child(self.date),
                                ),
                        ),
                    )
                    .child(
                        div()
                            .h_full()
                            .w_2_3()
                            .p_2()
                            .child(
                                div()
                                    .font_weight(FontWeight::BOLD)
                                    .h_1_5()
                                    .w_full()
                                    .overflow_hidden()
                                    .child(
                                        self.title
                                            .lines()
                                            .flat_map(|line| line.chars())
                                            .collect::<String>(),
                                    ),
                            )
                            .child(
                                div()
                                    .h_1_5()
                                    .w_full()
                                    .flex()
                                    .child(
                                        div()
                                            .flex()
                                            .w_1_2()
                                            .h_full()
                                            .child(
                                                div()
                                                    .mr_2()
                                                    .size_6()
                                                    .child(Icon::new(IconName::View, true)),
                                            )
                                            .child(self.view.to_string()),
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .w_1_2()
                                            .h_full()
                                            .child(
                                                div()
                                                    .mr_2()
                                                    .size_6()
                                                    .child(Icon::new(IconName::Reply, true)),
                                            )
                                            .child(self.reply.to_string()),
                                    ),
                            )
                            .child(div().h_3_5().w_full().flex().children(
                                self.samples.into_iter().map(|sample| {
                                    div().w_1_3().h_full().child(
                                        img(sample).size_full().rounded_md().overflow_hidden(),
                                    )
                                }),
                            )),
                    )
                    .hover(|s| s.bg(theme.hover_background))
                    .on_mouse_down(MouseButton::Left, {
                        move |_event, cx| {
                            cx.update_global::<State, ()>(|state, cx| {
                                state.machine_mut().load_detail(self.id, 1);
                                cx.refresh();
                            });
                        }
                    }),
            )
    }
}
