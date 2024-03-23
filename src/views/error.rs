use std::time::Duration;

use gpui::*;

use crate::components::{Icon, IconName};

#[derive(IntoElement)]
pub struct Error;

impl RenderOnce for Error {
    fn render(self, _cx: &mut WindowContext) -> impl IntoElement {
        div()
            .flex()
            .justify_center()
            .items_center()
            .size_full()
            .child(div().size_12().child(Icon::new(IconName::Error, true)))
            .with_animation(
                "error",
                Animation::new(Duration::from_secs(1))
                    .repeat()
                    .with_easing(ease_in_out),
                |this, delta| this.mt(Pixels((delta - 0.5).abs() * -100.0)),
            )
    }
}
