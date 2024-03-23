use async_std::task;
use gpui::*;

mod detail;
mod idle;
mod page;
mod state;

use crate::views::loading::Loading;
use crate::{db::DB, views::error::Error};
use detail::Detail;
pub use state::State;
use state::StateMachine;

pub struct Like;

impl Like {
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
        let (total, items) = cx.global::<DB>().likes(page);
        cx.spawn(|_, mut cx| async move {
            cx.update_global::<State, ()>(|state, cx| {
                state.page_mut().to(page);
                state.page_mut().set_high(total);
                state
                    .idle_mut()
                    .change_to(items.into_iter().map(|item| item.into()).collect(), cx);
                state.machine_mut().idle();
                cx.refresh();
            })
            .ok();
        })
        .detach();
    }

    fn load_detail(cx: &mut ViewContext<Self>, href: String) {
        cx.spawn(|_, mut cx| async move {
            cx.update_global::<State, ()>(|state, cx| {
                state.machine_mut().loading();
                cx.refresh();
            })
            .ok();
        })
        .detach();

        let client = cx.global::<State>().client().clone();
        let task_handle = cx.background_executor().spawn({
            let href = href.clone();
            async move {
                task::spawn(async move {
                    let Ok(res) = client.get(href).send().await else {
                        return None;
                    };

                    res.text().await.ok()
                })
                .await
            }
        });
        cx.spawn(|_view, mut cx| async move {
            let Some(doc) = task_handle.await else {
                cx.update_global::<State, ()>(|state, cx| {
                    state.machine_mut().detail_error(href);
                    cx.refresh();
                })
                .ok();

                return;
            };

            let Some(info) = Detail::parse(doc) else {
                cx.update_global::<State, ()>(|state, cx| {
                    state.machine_mut().detail_error(href);
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

impl Render for Like {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let title = cx.global::<State>().title();
        cx.set_window_title(&title);

        let state = cx.global::<State>();

        let view = div().size_full().flex();
        match state.machine() {
            StateMachine::Idle => view.child(state.idle().clone()).child(state.page().clone()),
            StateMachine::LoadPage(page) => {
                Self::load_page(cx, *page);

                view
            }
            StateMachine::Loading => view.child(Loading),
            StateMachine::LoadDetail(href) => {
                Self::load_detail(cx, href.clone());

                view.child(Loading)
            }
            StateMachine::DetailError(href) => view
                .child(Error)
                .on_mouse_down(MouseButton::Left, {
                    let href = href.clone();
                    move |_event, cx| {
                        cx.update_global::<State, ()>(|state, cx| {
                            state.machine_mut().load_detail(href.clone());
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
