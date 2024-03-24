use gpui::*;

use crate::{
    components::{Icon, IconName},
    theme::Theme,
};

#[derive(Clone)]
pub struct Tab {
    selected: Model<TabItem>,
}

impl Tab {
    const WIDTH: Pixels = Pixels(70.0);
    const PADDING: Pixels = Pixels(15.0);

    pub fn new(selected: Model<TabItem>) -> Self {
        Self { selected }
    }

    fn tabs(selected: Model<TabItem>, cx: &mut WindowContext) -> Vec<AnyElement> {
        let theme = cx.global::<Theme>();

        vec![
            TabItem::Home,
            TabItem::Talk,
            TabItem::Find,
            TabItem::Like,
            TabItem::Xiuren,
        ]
        .into_iter()
        .map(|item| {
            let active = selected.read(cx) == &item;

            let view = div()
                .size(Self::WIDTH - Self::PADDING * 2)
                .p_2()
                .mt_2()
                .mb_2()
                .rounded_lg()
                .hover(|s| {
                    s.bg(if active {
                        theme.background
                    } else {
                        theme.hover_background
                    })
                })
                .on_mouse_down(MouseButton::Left, {
                    let selected = selected.clone();
                    let item = item.clone();
                    move |_event, cx| {
                        selected.update(cx, |selected, cx| {
                            *selected = item.clone();
                            cx.refresh();
                        });
                    }
                });

            match item {
                TabItem::Home => view.child(Icon::new(IconName::Home, active)),
                TabItem::Talk => view.child(Icon::new(IconName::Talk, active)),
                TabItem::Find => view.child(Icon::new(IconName::Find, active)),
                TabItem::Like => view.child(Icon::new(IconName::Like, active)),
                TabItem::Xiuren => view.child(Icon::new(IconName::Xiuren, active)),
            }
            .into_any()
        })
        .collect()
    }
}

impl Render for Tab {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let tabs = Self::tabs(self.selected.clone(), cx);
        let theme = cx.global::<Theme>();

        div()
            .h_full()
            .w(Self::WIDTH)
            .flex()
            .justify_center()
            .items_center()
            .border_r_1()
            .border_color(theme.border)
            .child(
                div()
                    .w_full()
                    .p(Self::PADDING)
                    .flex_col()
                    .justify_center()
                    .items_center()
                    .children(tabs),
            )
    }
}

#[derive(Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum TabItem {
    #[default]
    Home,
    Talk,
    Find,
    Like,
    Xiuren,
}
