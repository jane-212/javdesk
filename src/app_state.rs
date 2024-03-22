use gpui::*;

pub struct AppState {
    view_image: Option<String>,
}

impl AppState {
    pub fn init(cx: &mut WindowContext) {
        cx.set_global(Self::new())
    }

    pub fn reset(cx: &mut WindowContext) {
        cx.update_global::<Self, _>(|app_state, _| {
            app_state.clear();
        });
    }

    fn clear(&mut self) {
        self.view_image = None;
    }

    fn new() -> Self {
        Self { view_image: None }
    }

    pub fn should_view_image(&self) -> bool {
        self.view_image.is_some()
    }

    pub fn view_image(&self) -> &Option<String> {
        &self.view_image
    }

    pub fn open(&mut self, src: String) {
        self.view_image = Some(src);
    }

    pub fn close(&mut self) {
        self.view_image = None;
    }
}

impl Global for AppState {}
