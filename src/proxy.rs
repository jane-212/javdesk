use std::sync::OnceLock;

use actix_web::{
    get, middleware,
    web::{self, Data},
    App, HttpResponse, HttpServer, Responder,
};
use async_std::task;
use gpui::*;
use reqwest::{
    header::{self, HeaderMap, HeaderValue},
    Client, ClientBuilder,
};
use serde::Deserialize;

pub struct Proxy;

impl Proxy {
    pub fn init(cx: &mut AppContext, proxy: Option<String>) {
        let mut client_builder = ClientBuilder::new();
        if let Some(proxy) = proxy.and_then(|proxy| reqwest::Proxy::https(proxy).ok()) {
            client_builder = client_builder.proxy(proxy);
        }
        let client = client_builder.build().expect("build proxy failed");

        cx.background_executor()
            .spawn(async {
                task::spawn(async { Self::run(client).await });
            })
            .detach();
    }

    async fn run(client: Client) {
        let state = Data::new(State::new(client));

        HttpServer::new(move || {
            App::new()
                .app_data(state.clone())
                .service(image)
                .wrap(middleware::Logger::default())
        })
        .disable_signals()
        .bind(("127.0.0.1", 8888))
        .expect("bind proxy failed")
        .run()
        .await
        .expect("run proxy failed");
    }
}

struct State {
    client: Client,
}

impl State {
    fn new(client: Client) -> Self {
        Self { client }
    }
}

#[derive(Deserialize)]
struct Param {
    t: String,
    src: String,
}

#[get("/image")]
async fn image(param: web::Query<Param>, state: web::Data<State>) -> impl Responder {
    let Ok(bytes) = state
        .client
        .get(&param.src)
        .headers(headers(&param.t).clone())
        .send()
        .await
        .map(|res| res.bytes())
    else {
        return HttpResponse::NotFound().body("error");
    };

    let Ok(bytes) = bytes.await else {
        return HttpResponse::NotFound().body("error");
    };

    HttpResponse::Ok().body(bytes)
}

pub enum ProxyUrl {
    Home(String),
    #[cfg(feature = "avatar")]
    Avatar(String),
    Talk(String),
}

impl ToString for ProxyUrl {
    fn to_string(&self) -> String {
        match self {
            ProxyUrl::Home(src) => format!("http://127.0.0.1:8888/image?t=home&src={}", src),
            #[cfg(feature = "avatar")]
            ProxyUrl::Avatar(src) => format!("http://127.0.0.1:8888/image?t=avatar&src={}", src),
            ProxyUrl::Talk(src) => format!("http://127.0.0.1:8888/image?t=talk&src={}", src),
        }
    }
}

fn headers(t: &str) -> &'static HeaderMap {
    match t {
        "home" => {
            static CLIENT: OnceLock<HeaderMap> = OnceLock::new();
            CLIENT.get_or_init(|| {
                let mut headers = HeaderMap::new();
                headers.insert(header::USER_AGENT, HeaderValue::from_static("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.2.1 Safari/605.1.15"));
                headers.insert(
                    header::REFERER,
                    HeaderValue::from_static("https://www.javbus.com/"),
                );
                headers.insert(header::HOST, HeaderValue::from_static("www.javbus.com"));
                headers
            })
        }
        #[cfg(feature = "avatar")]
        "avatar" => {
            static CLIENT: OnceLock<HeaderMap> = OnceLock::new();
            CLIENT.get_or_init(|| {
                let mut headers = HeaderMap::new();
                headers.insert(header::USER_AGENT, HeaderValue::from_static("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.2.1 Safari/605.1.15"));
                headers.insert(
                    header::REFERER,
                    HeaderValue::from_static("https://www.javbus.com/"),
                );
                headers.insert(header::HOST, HeaderValue::from_static("uc.javbus22.com"));
                headers
            })
        }
        "talk" => {
            static CLIENT: OnceLock<HeaderMap> = OnceLock::new();
            CLIENT.get_or_init(|| {
                let mut headers = HeaderMap::new();
                headers.insert(header::USER_AGENT, HeaderValue::from_static("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.2.1 Safari/605.1.15"));
                headers.insert(
                    header::REFERER,
                    HeaderValue::from_static("https://www.javbus.com/"),
                );
                headers.insert(header::HOST, HeaderValue::from_static("cloud.javcdn.cc"));
                headers
            })
        }
        _ => {
            static CLIENT: OnceLock<HeaderMap> = OnceLock::new();
            CLIENT.get_or_init(|| {
                let mut headers = HeaderMap::new();
                headers.insert(header::USER_AGENT, HeaderValue::from_static("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.2.1 Safari/605.1.15"));
                headers.insert(
                    header::REFERER,
                    HeaderValue::from_static("https://www.javbus.com/"),
                );
                headers.insert(header::HOST, HeaderValue::from_static("www.javbus.com"));
                headers
            })
        }
    }
}
