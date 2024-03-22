use gpui::prelude::FluentBuilder;
use gpui::*;

use super::find::Find;
use super::home::Home;
use super::tab::{Tab, TabItem};
use super::talk::Talk;
use crate::app_state::AppState;
use crate::theme::Theme;

pub struct Javdesk {
    selected: Model<TabItem>,
    tab: View<Tab>,
    home: View<Home>,
    talk: View<Talk>,
    find: View<Find>,
}

impl Javdesk {
    const IMAGE_PADDING: Pixels = Pixels(40.0);

    pub fn new(cx: &mut WindowContext) -> View<Self> {
        let selected = cx.new_model(|_| TabItem::default());
        let tab = cx.new_view(|_| Tab::new(selected.clone()));
        let home = cx.new_view(|_| Home);
        let talk = cx.new_view(|_| Talk);
        let find = cx.new_view(|_| Find);

        cx.new_view(|_| Self {
            selected,
            tab,
            home,
            talk,
            find,
        })
    }

    pub fn reset(&mut self, cx: &mut WindowContext) {
        cx.update_model(&self.selected, |selected, _| {
            *selected = TabItem::Home;
        });
    }
}

impl Render for Javdesk {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();
        let app_state = cx.global::<AppState>();

        let view = div().size_full().flex().justify_center().items_center();
        let view = match self.selected.read(cx) {
            TabItem::Home => view.child(self.home.clone()),
            TabItem::Talk => view.child(self.talk.clone()),
            TabItem::Find => view.child(self.find.clone()),
        };

        div()
            .size_full()
            .flex()
            .justify_center()
            .items_center()
            .bg(theme.background)
            .text_color(theme.text)
            .when(!app_state.should_view_image(), |this| {
                this.child(self.tab.clone()).child(view)
            })
            .when_some(app_state.view_image().clone(), |this, src| {
                let size = cx.viewport_size();
                this.child(
                    img(src)
                        .w(size.width - Self::IMAGE_PADDING * 2)
                        .h(size.height - Self::IMAGE_PADDING * 2)
                        .rounded_md()
                        .overflow_hidden(),
                )
                .on_mouse_down(MouseButton::Left, |_event, cx| {
                    cx.update_global::<AppState, ()>(|app_state, cx| {
                        app_state.close();
                        cx.refresh();
                    });
                })
            })
    }
}
