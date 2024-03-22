use std::time::Duration;

use gpui::*;
use reqwest::{
    header::{self, HeaderMap, HeaderValue},
    Client, ClientBuilder,
};

use super::detail::Detail;
use super::idle::Idle;
use super::page::Page;
use crate::config::Config;

pub struct State {
    client: Client,
    state_machine: StateMachine,
    page: Page,
    idle: Idle,
    detail: Detail,
}

impl State {
    pub fn new(cx: &mut WindowContext) -> Self {
        let mut headers = HeaderMap::new();
        headers.insert(
            header::ACCEPT_ENCODING,
            HeaderValue::from_static("gzip, deflate, br"),
        );
        headers.insert(header::USER_AGENT, HeaderValue::from_static("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.2.1 Safari/605.1.15"));
        headers.insert(header::HOST, HeaderValue::from_static("www.javbus.com"));
        headers.insert(
            header::ACCEPT,
            HeaderValue::from_static(
                "text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8",
            ),
        );
        headers.insert(
            header::ACCEPT_LANGUAGE,
            HeaderValue::from_static("zh-CN,zh-Hans;q=0.9"),
        );
        headers.insert(
            header::REFERER,
            HeaderValue::from_static("https://www.javbus.com/forum/"),
        );
        headers.insert(header::COOKIE, HeaderValue::from_static("4fJN_2132_forum_lastvisit=D_2_1711099677; 4fJN_2132_lastact=1711099677%09forum.php%09forumdisplay; 4fJN_2132_sid=A8XWwx; 4fJN_2132_st_t=0%7C1711099677%7Ca2d31057b3591c29c72f5ff0017a559b; 4fJN_2132_visitedfid=2; 4fJN_2132_sendmail=1; 4fJN_2132_onlineusernum=6752; 4fJN_2132_lastvisit=1711096068; 4fJN_2132_saltkey=NfI5nEeN; PHPSESSID=jsvjspm82pob3d0c4tag91faq0; existmag=mag"));
        let proxy = cx.global::<Config>().proxy.clone();
        let mut client_builder = ClientBuilder::new()
            .timeout(Duration::from_secs(5))
            .default_headers(headers);
        if let Some(proxy) = proxy.and_then(|proxy| reqwest::Proxy::https(proxy).ok()) {
            client_builder = client_builder.proxy(proxy);
        }
        let client = client_builder.build().expect("build http client failed");

        let current_page = 1;
        let mut page = Page::new();
        page.to(current_page);

        let mut state_machine = StateMachine::default();
        state_machine.load_page(current_page);

        let idle = Idle::new(cx);
        let detail = Detail::new();

        Self {
            client,
            state_machine,
            page,
            idle,
            detail,
        }
    }

    pub fn machine(&self) -> &StateMachine {
        &self.state_machine
    }

    pub fn machine_mut(&mut self) -> &mut StateMachine {
        &mut self.state_machine
    }

    pub fn idle(&self) -> &Idle {
        &self.idle
    }

    pub fn idle_mut(&mut self) -> &mut Idle {
        &mut self.idle
    }

    pub fn page(&self) -> &Page {
        &self.page
    }

    pub fn page_mut(&mut self) -> &mut Page {
        &mut self.page
    }

    pub fn client(&self) -> &Client {
        &self.client
    }

    pub fn detail(&self) -> &Detail {
        &self.detail
    }

    pub fn detail_mut(&mut self) -> &mut Detail {
        &mut self.detail
    }

    pub fn title(&self) -> String {
        match self.state_machine {
            StateMachine::Idle => "Talk".to_string(),
            StateMachine::Detail => self.detail.title(),
            StateMachine::LoadPage(_) => "Loading".to_string(),
            StateMachine::LoadDetail(_, _) => "Loading".to_string(),
            StateMachine::Loading => "Loading".to_string(),
            StateMachine::PageError(_) => "Error".to_string(),
            StateMachine::DetailError(_, _) => "Error".to_string(),
        }
    }
}

impl Global for State {}

#[derive(Default)]
pub enum StateMachine {
    #[default]
    Idle,
    Detail,
    LoadPage(i32),
    LoadDetail(i32, i32),
    Loading,
    PageError(i32),
    DetailError(i32, i32),
}

impl StateMachine {
    pub fn reset(&mut self) {
        *self = Self::LoadPage(1);
    }

    pub fn idle(&mut self) {
        match self {
            Self::Idle | Self::LoadPage(_) | Self::PageError(_) | Self::LoadDetail(_, _) => {}
            Self::Loading | Self::Detail | Self::DetailError(_, _) => *self = Self::Idle,
        }
    }

    pub fn detail(&mut self) {
        match self {
            Self::Detail
            | Self::Idle
            | Self::LoadPage(_)
            | Self::PageError(_)
            | Self::DetailError(_, _)
            | Self::LoadDetail(_, _) => {}
            Self::Loading => *self = Self::Detail,
        }
    }

    pub fn load_page(&mut self, page: i32) {
        match self {
            Self::LoadPage(_)
            | Self::Loading
            | Self::LoadDetail(_, _)
            | Self::Detail
            | Self::DetailError(_, _) => {}
            Self::Idle | Self::PageError(_) => *self = Self::LoadPage(page),
        }
    }

    pub fn load_detail(&mut self, id: i32, page: i32) {
        match self {
            Self::LoadPage(_) | Self::Loading | Self::LoadDetail(_, _) | Self::PageError(_) => {}
            Self::Idle | Self::DetailError(_, _) | Self::Detail => {
                *self = Self::LoadDetail(id, page)
            }
        }
    }

    pub fn loading(&mut self) {
        match self {
            Self::Idle
            | Self::Loading
            | Self::PageError(_)
            | Self::DetailError(_, _)
            | Self::Detail => {}
            Self::LoadPage(_) | Self::LoadDetail(_, _) => *self = Self::Loading,
        }
    }

    pub fn page_error(&mut self, page: i32) {
        match self {
            Self::Idle
            | Self::Detail
            | Self::LoadPage(_)
            | Self::LoadDetail(_, _)
            | Self::PageError(_)
            | Self::DetailError(_, _) => {}
            Self::Loading => *self = Self::PageError(page),
        }
    }

    pub fn detail_error(&mut self, id: i32, page: i32) {
        match self {
            Self::Idle
            | Self::Detail
            | Self::LoadPage(_)
            | Self::LoadDetail(_, _)
            | Self::PageError(_)
            | Self::DetailError(_, _) => {}
            Self::Loading => *self = Self::DetailError(id, page),
        }
    }
}
