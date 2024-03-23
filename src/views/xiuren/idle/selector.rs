use std::sync::OnceLock;

use scraper::Selector;

pub struct Selectors {
    pub pages: Selector,
    pub items: Selector,
    pub cover: Selector,
    pub title: Selector,
    pub date: Selector,
    pub id: Selector,
}

macro_rules! selector {
    ($s:expr) => {
        scraper::Selector::parse($s).expect("parse selector failed")
    };
}

impl Selectors {
    fn new() -> Self {
        Self {
            pages: selector!("body>div.text-center>ul.pagination>li>a"),
            items: selector!(
                "body>div.container-fluid>div.row>div#waterfall>div#waterfall>div.item>a.movie-box"
            ),
            cover: selector!("div.photo-frame>img"),
            title: selector!("div.photo-frame>img"),
            date: selector!("div.photo-info>span>date"),
            id: selector!("div.photo-info>span>date"),
        }
    }
}

pub fn selectors() -> &'static Selectors {
    static SELECTORS: OnceLock<Selectors> = OnceLock::new();
    SELECTORS.get_or_init(Selectors::new)
}
