use std::sync::OnceLock;

use scraper::Selector;

pub struct Selectors {
    pub items: Selector,
    pub href: Selector,
    pub cover: Selector,
    pub title: Selector,
    pub date: Selector,
    pub view: Selector,
}

macro_rules! selector {
    ($s:expr) => {
        scraper::Selector::parse($s).expect("parse selector failed")
    };
}

impl Selectors {
    fn new() -> Self {
        Self {
            items: selector!("article.jeg_post>div.box_wrap"),
            href: selector!("div.jeg_thumb>a"),
            cover: selector!("div.jeg_thumb>a>div.thumbnail-container>img"),
            title: selector!("div.jeg_thumb>a>div.thumbnail-container>img"),
            date: selector!("div.jeg_postblock_content>div.jeg_post_meta>div.jeg_meta_date>a"),
            view: selector!("div.jeg_postblock_content>div.jeg_post_meta>div.jeg_meta_views>a"),
        }
    }
}

pub fn selectors() -> &'static Selectors {
    static SELECTORS: OnceLock<Selectors> = OnceLock::new();
    SELECTORS.get_or_init(Selectors::new)
}
