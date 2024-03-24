use gpui::*;

pub struct Theme {
    pub background: Hsla,
    pub main: Hsla,
    pub text: Hsla,
    pub icon: Hsla,
    pub hover_background: Hsla,
    pub border: Hsla,
    pub name: Hsla,
    pub overlay: Hsla,
}

impl Theme {
    fn new() -> Self {
        Self {
            background: rgb(0x363433).into(),
            main: rgb(0xffc90c).into(),
            text: rgb(0xe3bd8d).into(),
            icon: rgb(0xe3bd8d).into(),
            hover_background: rgb(0x80766e).into(),
            border: rgb(0x80766e).into(),
            name: rgb(0x51c4d3).into(),
            overlay: rgba(0x00000055).into(),
        }
    }

    pub fn init(cx: &mut AppContext) {
        cx.set_global(Self::new())
    }
}

impl Global for Theme {}
