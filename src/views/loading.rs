use std::time::Duration;

use gpui::*;

use crate::{
    components::{Icon, IconName},
    theme::Theme,
};

#[derive(IntoElement)]
pub struct Loading;

const HEIGHT: Pixels = Pixels(200.0);

impl RenderOnce for Loading {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        let theme = cx.global::<Theme>();
        let height = cx.viewport_size().height - HEIGHT;

        div()
            .flex()
            .justify_center()
            .items_center()
            .size_full()
            .child(
                div()
                    .w_full()
                    .h(HEIGHT)
                    .absolute()
                    .top_0()
                    .left_0()
                    .border_l_1()
                    .border_color(theme.main)
                    .with_animation(
                        "loading-line",
                        Animation::new(Duration::from_secs(2))
                            .repeat()
                            .with_easing(bounce(ease_in_out)),
                        move |this, delta| this.top(delta * height),
                    ),
            )
            .child(
                div()
                    .size_12()
                    .child(Icon::new(IconName::Loading, true))
                    .with_animation(
                        "loading",
                        Animation::new(Duration::from_secs(1))
                            .repeat()
                            .with_easing(ease_in_out),
                        |this, delta| this.mt(Pixels((delta - 0.5).abs() * -100.0)),
                    ),
            )
    }
}
