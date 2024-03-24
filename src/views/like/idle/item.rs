use gpui::*;

use super::State;
use crate::components::{Icon, IconName};
use crate::theme::Theme;

#[derive(IntoElement, Clone)]
pub struct Item {
    href: String,
    cover: String,
    title: String,
    id: String,
    date: String,
}

impl Item {
    const HEIGHT: Pixels = Pixels(200.0);
}

impl From<(String, String, String, String, String)> for Item {
    fn from(value: (String, String, String, String, String)) -> Self {
        Self {
            id: value.0,
            href: value.1,
            title: value.2,
            cover: value.3,
            date: value.4,
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
                    .flex()
                    .p_4()
                    .w_2_3()
                    .h_full()
                    .rounded_lg()
                    .child(
                        div().h_full().w_1_5().child(
                            img(self.cover)
                                .size_full()
                                .rounded_md()
                                .overflow_hidden()
                                .border_1()
                                .border_color(theme.border)
                                .object_fit(ObjectFit::Cover),
                        ),
                    )
                    .child(
                        div()
                            .h_full()
                            .w_4_5()
                            .p_2()
                            .child(
                                div()
                                    .font_weight(FontWeight::BOLD)
                                    .h_4_5()
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
                                                    .child(Icon::new(IconName::ID, true)),
                                            )
                                            .child(self.id.clone()),
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
                            ),
                    )
                    .hover(|s| s.bg(theme.hover_background))
                    .on_mouse_down(MouseButton::Left, {
                        let href = self.href.clone();
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
