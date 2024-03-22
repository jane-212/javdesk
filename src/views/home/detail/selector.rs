use std::sync::OnceLock;

use scraper::Selector;

pub struct Selectors {
    pub cover: Selector,
    pub title: Selector,
    pub date: Selector,
    pub id: Selector,
    pub cost: Selector,
    pub samples: Selector,
}

macro_rules! selector {
    ($s:expr) => {
        scraper::Selector::parse($s).expect("parse selector failed")
    };
}

impl Selectors {
    fn new() -> Self {
        Self {
            cover: selector!("body>div.container>div.movie>div.screencap>a.bigImage>img"),
            title: selector!("body>div.container>div.movie>div.screencap>a.bigImage>img"),
            id: selector!("body>div.container>div.movie>div.info>p>span"),
            date: selector!("body>div.container>div.movie>div.info>p"),
            cost: selector!("body>div.container>div.movie>div.info>p"),
            samples: selector!(
                "body>div.container>div#sample-waterfall>a.sample-box>div.photo-frame>img"
            ),
        }
    }
}

pub fn selectors() -> &'static Selectors {
    static SELECTORS: OnceLock<Selectors> = OnceLock::new();
    SELECTORS.get_or_init(Selectors::new)
}
