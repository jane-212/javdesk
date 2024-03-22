use gpui::*;

use crate::theme::Theme;

#[derive(IntoElement)]
pub struct Icon {
    path: &'static str,
    active: bool,
}

impl Icon {
    pub fn new(name: IconName, active: bool) -> Self {
        Self {
            path: name.path(),
            active,
        }
    }
}

impl RenderOnce for Icon {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        let color = if self.active { theme.main } else { theme.icon };
        svg().size_full().path(self.path).text_color(color)
    }
}

pub enum IconName {
    Home,
    Talk,
    Find,
    Loading,
    Error,
    Date,
    ID,
    Cost,
    View,
    Reply,
}

impl IconName {
    fn path(self) -> &'static str {
        match self {
            Self::Home => "icons/home.svg",
            Self::Talk => "icons/talk.svg",
            Self::Find => "icons/find.svg",
            Self::Loading => "icons/loading.svg",
            Self::Error => "icons/error.svg",
            Self::Date => "icons/date.svg",
            Self::ID => "icons/id.svg",
            Self::Cost => "icons/cost.svg",
            Self::View => "icons/view.svg",
            Self::Reply => "icons/reply.svg",
        }
    }
}
