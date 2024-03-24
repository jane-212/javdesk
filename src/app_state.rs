use gpui::*;

pub struct AppState {
    view_image: Option<String>,
    view_scale: Pixels,
    view_pos: Point<Pixels>,
}

impl AppState {
    pub fn init(cx: &mut WindowContext) {
        cx.set_global(Self::new())
    }

    #[cfg(feature = "hide")]
    pub fn reset(cx: &mut WindowContext) {
        cx.update_global::<Self, _>(|app_state, _| {
            app_state.clear();
        });
    }

    fn clear(&mut self) {
        self.view_image = None;
        self.view_scale = Pixels(1.0);
        self.view_pos.x = Pixels::ZERO;
        self.view_pos.y = Pixels::ZERO;
    }

    fn new() -> Self {
        Self {
            view_image: None,
            view_scale: Pixels(1.0),
            view_pos: Point::new(Pixels::ZERO, Pixels::ZERO),
        }
    }

    pub fn should_view_image(&self) -> bool {
        self.view_image.is_some()
    }

    pub fn view_image(&self) -> &Option<String> {
        &self.view_image
    }

    pub fn view_scale(&self) -> Pixels {
        self.view_scale
    }

    pub fn view_pos(&self) -> Point<Pixels> {
        self.view_pos
    }

    pub fn scale(&mut self, scale: Pixels) {
        self.view_scale = (self.view_scale - scale).clamp(Pixels(0.5), Pixels(2.0));
    }

    pub fn pos_move(&mut self, x: Pixels, y: Pixels, size: Size<Pixels>) {
        self.view_pos.x = (self.view_pos.x + x).clamp(-size.width, size.width);
        self.view_pos.y = (self.view_pos.y + y).clamp(-size.height, size.height);
    }

    pub fn open(&mut self, src: String) {
        self.view_image = Some(src);
    }

    pub fn close(&mut self) {
        self.clear();
    }
}

impl Global for AppState {}
