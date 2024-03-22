use gpui::*;

mod item;

use super::State;
use crate::components::Scroll;
use item::Item;

#[derive(IntoElement, Clone)]
pub struct Idle {
    items: Model<Vec<Item>>,
    scroll: Scroll,
}

impl Idle {
    pub fn new(cx: &mut WindowContext) -> Self {
        let items: Model<Vec<Item>> = cx.new_model(|_| Vec::new());

        let list_state = ListState::new(items.read(cx).len(), ListAlignment::Top, Pixels(0.0), {
            let items = items.clone();
            move |i, cx| items.read(cx)[i].clone().into_any_element()
        });

        Self {
            items,
            scroll: Scroll::new(list_state),
        }
    }

    fn scroll(&mut self, delta: Pixels) {
        self.scroll.scroll(delta);
    }

    pub fn change_to(&mut self, mut cur_items: Vec<Item>, cx: &mut WindowContext) {
        cx.update_model(&self.items, |items, _cx| {
            items.clear();
            items.append(&mut cur_items);
        });
        let items = self.items.clone();
        self.scroll = Scroll::new(ListState::new(
            items.read(cx).len(),
            ListAlignment::Top,
            Pixels(0.0),
            move |i, cx| items.read(cx)[i].clone().into_any_element(),
        ));
    }
}

impl RenderOnce for Idle {
    fn render(self, _cx: &mut WindowContext) -> impl IntoElement {
        div()
            .h_full()
            .w_full()
            .flex()
            .justify_center()
            .items_center()
            .child(self.scroll)
            .on_scroll_wheel(move |event, cx| {
                let delta = event.delta.pixel_delta(Pixels(1.0)).y;
                cx.update_global::<State, ()>(|state, cx| {
                    state.idle_mut().scroll(delta);
                    cx.refresh();
                });
            })
    }
}
