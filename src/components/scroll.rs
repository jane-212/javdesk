use gpui::*;

#[derive(IntoElement, Clone)]
pub struct Scroll {
    list_state: ListState,
}

impl Scroll {
    pub fn new(list_state: ListState) -> Self {
        Self { list_state }
    }

    pub fn scroll(&self, delta: Pixels) {
        let state = &self.list_state;
        let mut top = state.logical_scroll_top();
        top.offset_in_item -= delta;
        top.offset_in_item = top.offset_in_item.max(Pixels::ZERO);
        state.scroll_to(top);
    }
}

impl RenderOnce for Scroll {
    fn render(self, _cx: &mut WindowContext) -> impl IntoElement {
        list(self.list_state).size_full()
    }
}

#[derive(IntoElement, Clone)]
pub struct ScrollOwned {
    list_state: Model<ListState>,
}

impl ScrollOwned {
    pub fn new(list_state: Model<ListState>) -> Self {
        Self { list_state }
    }

    pub fn scroll(&self, delta: Pixels, cx: &mut WindowContext) {
        let state = self.list_state.read(cx);
        let mut top = state.logical_scroll_top();
        top.offset_in_item -= delta;
        top.offset_in_item = top.offset_in_item.max(Pixels::ZERO);
        state.scroll_to(top);
    }

    pub fn state(&self) -> Model<ListState> {
        self.list_state.clone()
    }
}

impl RenderOnce for ScrollOwned {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        list(self.list_state.read(cx).clone()).size_full()
    }
}
