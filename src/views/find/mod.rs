use async_std::task;
use gpui::*;

mod detail;
mod idle;
mod page;
mod state;

use crate::views::error::Error;
use crate::views::loading::Loading;
use detail::Detail;
use idle::Idle;
use state::{State, StateMachine};

pub struct Find;

impl Find {
    const BASE_URL: &'static str = "https://www.javbus.com";

    pub fn init(cx: &mut WindowContext) {
        let state = State::new(cx);
        cx.set_global(state);
    }

    pub fn reset(cx: &mut WindowContext) {
        cx.update_global::<State, _>(|state, cx| {
            *state = State::new(cx);
        });
    }

    fn load_page(cx: &mut ViewContext<Self>, page: i32) {
        cx.spawn(|_view, mut cx| async move {
            cx.update_global::<State, ()>(|state, cx| {
                state.machine_mut().loading();
                cx.refresh();
            })
            .ok();
        })
        .detach();

        let client = cx.global::<State>().client().clone();
        let task_handle = cx.background_executor().spawn(async move {
            task::spawn(async move {
                let url = format!(
                    "{}/forum/forum.php?mod=forumdisplay&fid=36&page={}",
                    Self::BASE_URL,
                    page
                );
                let Ok(res) = client.get(url).send().await else {
                    return None;
                };

                res.text().await.ok().map(Idle::parse)
            })
            .await
        });
        cx.spawn(|_view, mut cx| async move {
            let Some((high, items)) = task_handle.await else {
                cx.update_global::<State, ()>(|state, cx| {
                    state.machine_mut().page_error(page);
                    cx.refresh();
                })
                .ok();

                return;
            };
            if items.is_empty() {
                cx.update_global::<State, ()>(|state, cx| {
                    state.machine_mut().page_error(page);
                    cx.refresh();
                })
                .ok();

                return;
            }

            cx.update_global::<State, ()>(|state, cx| {
                state.page_mut().to(page);
                state.page_mut().set_high(high);
                state.idle_mut().change_to(items, cx);
                state.machine_mut().idle();
                cx.refresh();
            })
            .ok();
        })
        .detach();
    }

    fn load_detail(cx: &mut ViewContext<Self>, id: i32, page: i32) {
        cx.spawn(|_view, mut cx| async move {
            cx.update_global::<State, ()>(|state, cx| {
                state.machine_mut().loading();
                cx.refresh();
            })
            .ok();
        })
        .detach();

        let client = cx.global::<State>().client().clone();
        let task = cx.background_executor().spawn({
            let url = format!(
                "{}/forum/forum.php?mod=viewthread&tid={}&extra=&page={}",
                Self::BASE_URL,
                id,
                page
            );
            async move {
                task::spawn(async move {
                    let Ok(res) = client.get(url).send().await else {
                        return None;
                    };

                    res.text().await.ok()
                })
                .await
            }
        });
        cx.spawn(|_view, mut cx| async move {
            let Some(doc) = task.await else {
                cx.update_global::<State, ()>(|state, cx| {
                    state.machine_mut().detail_error(id, page);
                    cx.refresh();
                })
                .ok();

                return;
            };

            let Some(info) = Detail::parse(doc, id, page) else {
                cx.update_global::<State, ()>(|state, cx| {
                    state.machine_mut().detail_error(id, page);
                    cx.refresh();
                })
                .ok();

                return;
            };

            cx.update_global::<State, ()>(|state, cx| {
                state.detail_mut().show(info);
                state.machine_mut().detail();
                cx.refresh();
            })
            .ok();
        })
        .detach();
    }
}

impl Render for Find {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let title = cx.global::<State>().title();
        cx.set_window_title(&title);

        let state = cx.global::<State>();

        let view = div().size_full().flex();
        match state.machine() {
            StateMachine::Idle => view.child(state.idle().clone()).child(state.page().clone()),
            StateMachine::LoadPage(page) => {
                Self::load_page(cx, *page);

                view.child(Loading)
            }
            StateMachine::Loading => view.child(Loading),
            StateMachine::PageError(page) => view.child(Error).on_mouse_down(MouseButton::Left, {
                let page = *page;
                move |_event, cx| {
                    cx.update_global::<State, ()>(|state, cx| {
                        state.machine_mut().load_page(page);
                        cx.refresh()
                    });
                }
            }),
            StateMachine::LoadDetail(id, page) => {
                Self::load_detail(cx, *id, *page);

                view.child(Loading)
            }
            StateMachine::DetailError(id, page) => view
                .child(Error)
                .on_mouse_down(MouseButton::Left, {
                    let id = *id;
                    let page = *page;
                    move |_event, cx| {
                        cx.update_global::<State, ()>(|state, cx| {
                            state.machine_mut().load_detail(id, page);
                            cx.refresh();
                        });
                    }
                })
                .on_mouse_down(MouseButton::Right, {
                    move |_event, cx| {
                        cx.update_global::<State, ()>(|state, cx| {
                            state.machine_mut().idle();
                            cx.refresh();
                        });
                    }
                }),
            StateMachine::Detail => view.child(state.detail().clone()),
        }
    }
}
