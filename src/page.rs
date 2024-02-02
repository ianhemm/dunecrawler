use scraper::{Html, Selector};

pub struct Page {
    //body: String
    links: Vec<String>,
}

impl Page {
    pub fn parse(doc: &str) -> Page{
        let html: Html = Html::parse_document(doc);
        let selector = Selector::parse("a").unwrap();

        let mut page = Page {
            links: Vec::new(),
        };
        let href = html.select(&selector);
        for link in href {
            if let Some(cur) = link.attr("href") {
                page.links.push(String::from(cur));
            }
        }
        page
    }

    pub fn links(self) -> Vec<String> {
        self.links
    }
}


