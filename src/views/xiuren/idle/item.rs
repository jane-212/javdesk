use gpui::*;

use super::State;
use crate::components::{Icon, IconName};
use crate::theme::Theme;

#[derive(IntoElement, Clone)]
pub struct Item {
    href: String,
    cover: String,
    title: String,
    view: i32,
    date: String,
}

impl Item {
    const HEIGHT: Pixels = Pixels(400.0);

    pub fn new(href: String, cover: String, title: String, view: i32, date: String) -> Self {
        Self {
            href,
            cover,
            title,
            view,
            date,
        }
    }
}

impl RenderOnce for Item {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        div()
            .flex()
            .justify_center()
            .items_center()
            .w_full()
            .h(Self::HEIGHT)
            .child(
                div()
                    .p_6()
                    .w_1_2()
                    .h_full()
                    .rounded_lg()
                    .child(
                        div().absolute().top_0().left_0().size_full().p_4().child(
                            div().size_full().child(
                                img(self.cover)
                                    .rounded_md()
                                    .overflow_hidden()
                                    .size_full()
                                    .object_fit(ObjectFit::Fill),
                            ),
                        ),
                    )
                    .child(
                        div().absolute().top_0().left_0().size_full().p_4().child(
                            div()
                                .size_full()
                                .rounded_md()
                                .overflow_hidden()
                                .bg(theme.overlay),
                        ),
                    )
                    .child(
                        div()
                            .font_weight(FontWeight::BOLD)
                            .h_1_5()
                            .w_full()
                            .overflow_hidden()
                            .child(self.title),
                    )
                    .child(
                        div()
                            .h_1_5()
                            .w_full()
                            .flex()
                            .justify_center()
                            .items_center()
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
                                            .child(Icon::new(IconName::Date, true)),
                                    )
                                    .child(self.date),
                            ),
                    )
                    .hover(|s| s.bg(theme.hover_background))
                    .on_mouse_down(MouseButton::Left, {
                        let href = self.href;
                        move |_event, cx| {
                            cx.update_global::<State, ()>(|state, cx| {
                                state.machine_mut().load_detail(href.clone());
                                cx.refresh();
                            });
                        }
                    }),
            )
    }
}
