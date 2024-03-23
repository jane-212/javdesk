use gpui::*;

use crate::app_state::AppState;
use crate::assets::Assets;
use crate::config::Config;
use crate::db::DB;
use crate::proxy::Proxy;
use crate::theme::Theme;
use crate::views::{Find, Home, Javdesk, Like, Talk, Xiuren};

pub const NAME: &str = "javdesk";

pub fn run_app(app: App) {
    app.with_assets(Assets).run(|cx: &mut AppContext| {
        Config::init(cx);
        Theme::init(cx);
        let proxy = cx.global::<Config>().proxy.clone();
        Proxy::init(cx, proxy);
        DB::init(cx);

        let bounds = Bounds::maximized(cx);
        cx.open_window(
            WindowOptions {
                bounds: Some(bounds),
                ..Default::default()
            },
            move |cx| {
                cx.activate(true);

                AppState::init(cx);
                Home::init(cx);
                Talk::init(cx);
                Find::init(cx);
                Like::init(cx);
                Xiuren::init(cx);
                let javdesk = Javdesk::new(cx);

                cx.on_window_should_close({
                    let javdesk = javdesk.downgrade();
                    move |cx| {
                        cx.hide();
                        Config::reload(cx);
                        AppState::reset(cx);
                        Home::reset(cx);
                        Talk::reset(cx);
                        Find::reset(cx);
                        Like::reset(cx);
                        Xiuren::reset(cx);
                        if let Some(javdesk) = javdesk.upgrade() {
                            cx.update_view(&javdesk, |javdesk, cx| {
                                javdesk.reset(cx);
                            });
                        }
                        cx.refresh();

                        false
                    }
                });

                javdesk
            },
        );
    });
}
