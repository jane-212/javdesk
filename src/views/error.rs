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
    }
}
