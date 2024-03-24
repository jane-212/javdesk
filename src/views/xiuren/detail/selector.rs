use std::sync::OnceLock;

use scraper::Selector;

pub struct Selectors {
    pub image: Selector,
    pub title: Selector,
}

macro_rules! selector {
    ($s:expr) => {
        scraper::Selector::parse($s).expect("parse selector failed")
    };
}

impl Selectors {
    fn new() -> Self {
        Self {
            title: selector!("body>div.jeg_viewport>div.post-wrapper>div.post-wrap>div.jeg_main>div.jeg_container>div.jeg_content>div.container>div.entry-header>h1.jeg_post_title"),
            image: selector!("body>div.jeg_viewport>div.post-wrapper>div.post-wrap>div.jeg_main>div.jeg_container>div.jeg_content>div.container>div.row>div.jeg_main_content>div.jeg_inner_content>div.entry-content>div.content-inner>p>a>img"),
        }
    }
}

pub fn selectors() -> &'static Selectors {
    static SELECTORS: OnceLock<Selectors> = OnceLock::new();
    SELECTORS.get_or_init(Selectors::new)
}
