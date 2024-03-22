use std::sync::OnceLock;

use scraper::Selector;

pub struct Selectors {
    pub page: Selector,
    pub items: Selector,
    pub avatar: Selector,
    pub title: Selector,
    pub date: Selector,
    pub name: Selector,
    pub view: Selector,
    pub reply: Selector,
    pub span: Selector,
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
            page: selector!("a#autopbn"),
            items: selector!("table#threadlisttableid>tbody"),
            avatar: selector!("tr>th>div.post_avatar>a>img"),
            title: selector!(
                "tr>th>div.post_inforight>div.post_infolist>div.post_infolist_tit>a.s"
            ),
            date: selector!("tr>th>div.post_inforight>div.post_infolist_other>div>span.dateline"),
            name: selector!("tr>th>div.post_inforight>div.post_infolist_other>div>span.author>a"),
            view: selector!("tr>th>div.post_inforight>div.post_infolist_other>div>span.views"),
            reply: selector!("tr>th>div.post_inforight>div.post_infolist_other>div>span.reply"),
            span: selector!("span"),
            samples: selector!(
                "tr>th>div.post_inforight>div.post_infolist>div.post_infolist_tit>a>img"
            ),
        }
    }
}

pub fn selectors() -> &'static Selectors {
    static SELECTORS: OnceLock<Selectors> = OnceLock::new();
    SELECTORS.get_or_init(Selectors::new)
}
