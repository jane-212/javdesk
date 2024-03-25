use gpui::*;

pub struct Theme {
    pub background: Hsla,
    pub tab_background: Hsla,
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
            background: rgb(0x282828).into(),
            tab_background: rgb(0x32302f).into(),
            main: rgb(0xffc90c).into(),
            text: rgb(0xeeeeee).into(),
            icon: rgb(0xeeeeee).into(),
            hover_background: rgb(0x1d2021).into(),
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
