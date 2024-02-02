use core::cmp::Ordering;

use reqwest::blocking::Client;
use std::{thread, time::Duration};

use dunecrawler::crawler::Crawler;
use dunecrawler::{Link,Page};

const INITIAL_SEED: &[&str] = &[
        "https://wikipedia.org",
        "https://neocities.org/browse",
        "https://youtube.com",
];

const QUEUE_RESULTS_MAX:usize = 100000; // number of results in the queue before the stop flag is raised
const _QUEUE_THREADS:u8 = 4; // number of threads to spawn in the thread pool

fn main() {
    let _database = ""; // database to cache results in

    let client = Client::new();
    // let mut crawler = Crawler::new(&[
    //     "https://wikipedia.org",
    //     "https://neocities.org/browse",
    //     "https://youtube.com",
    // ]);
    let mut crawler = Crawler::new(INITIAL_SEED);

    // while we have links in the queue
    while let Some(link) = crawler.pop_link() {
        // take the next link in the queue
        println!("Crawling link: {}", &link.name());
        let response = match client
            .get(link.name().clone())
            .send() {
                Ok(result) => result,
                Err(..) => continue,
            };

        let page = Page::parse(&response.text().unwrap());
        let links = page.links();

        let mut temp = Vec::new();
        links.into_iter()
            .map(|x| Link::new(&x).normalize(&link))
            .for_each(|x| temp.append(&mut x.traverse()));
        let links = temp;


        // calculate the weight of a link
        // the goal is to have a weight which wont explode too quickly
        // (ideally slower than +1 weight per link)
        // but at the same time accurately put links that appear more higher on the list
        let linkweight = ((link.weight() * 10.0)
                          / links.len() as f64)
                        .log2();

        links.into_iter().for_each(|x| crawler.submit(x, linkweight));

        if crawler.queue_len() > QUEUE_RESULTS_MAX {
            crawler.set_stop(true);
        }

        thread::sleep(Duration::from_millis(15));
    }

    let mut results = crawler.results();
    results.sort_by(|a,b|{
        a.weight()
            .partial_cmp(&b.weight())
            .unwrap_or_else(||{Ordering::Less})});

    results.into_iter().for_each(|x| println!("{}:{}",x.name(), x.weight()));
}
