use std::sync::OnceLock;

use scraper::Selector;

pub struct Selectors {
    pub post_avatar: Selector,
    pub post_name: Selector,
    pub page: Selector,
    pub title: Selector,
    pub items: Selector,
    pub tr: Selector,
    pub avatar: Selector,
    pub name: Selector,
    pub content: Selector,
    pub replys: Selector,
    pub reply_avatar: Selector,
    pub reply_name: Selector,
    pub reply_content: Selector,
    pub quote: Selector,
}

macro_rules! selector {
    ($s:expr) => {
        scraper::Selector::parse($s).expect("parse selector failed")
    };
}

impl Selectors {
    fn new() -> Self {
        Self {
            post_avatar: selector!(
                "div#ct>div.wp>div.sd>div.viewthread_authorinfo>div.avatar>a.avtm>img"
            ),
            post_name: selector!("div#ct>div.wp>div.sd>div.viewthread_authorinfo>div.authi>a.xw1"),
            page: selector!("div#ct>div.wp>div.mn>div.pgs>div.pg>label>span"),
            title: selector!("span#thread_subject"),
            items: selector!("div#postlist>div.nthread_postbox>table>tbody"),
            tr: selector!("tr"),
            avatar: selector!("td.pls>div.pls>div>div.avatar>a.avtm>img"),
            name: selector!("td.plc>div.pi>div.pti>div.authi>a.xw1"),
            content: selector!("td.plc>div.pct>div.pcb>div.t_fsz>table>tbody>tr>td"),
            replys: selector!("td.plc>div.pct>div.pcb>div.cm>div.pstl"),
            reply_avatar: selector!("div.psta>a>img"),
            reply_name: selector!("div.psta>a.xw1"),
            reply_content: selector!("div.psti"),
            quote: selector!(
                "td.plc>div.pct>div.pcb>div.t_fsz>table>tbody>tr>td>div.quote>blockquote"
            ),
        }
    }
}

pub fn selectors() -> &'static Selectors {
    static SELECTORS: OnceLock<Selectors> = OnceLock::new();
    SELECTORS.get_or_init(Selectors::new)
}
