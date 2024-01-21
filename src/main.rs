use std::collections::VecDeque;
use core::cmp::Ordering;

use reqwest::blocking::Client;
use scraper::{Html, Selector};
use std::{thread, time::Duration};

use dunecrawler::{crawler::Crawler, link_normalize, link_traverse};


const QUEUE_RESULTS_MAX:usize = 100000; // number of results in the queue before the stop flag is raised
const _QUEUE_THREADS:u8 = 4; // number of threads to spawn in the thread pool

fn main() {
    let seed: &[&str]; // List of URLs to start searching
    let _database = ""; // database to cache results in

    let client = Client::new();
    let mut crawler = Crawler::new(&[
        "https://wikipedia.org",
        "https://neocities.org/browse",
        "https://youtube.com",
    ]);

    // while we have links in the queue
    while let Some(link) = crawler.pop_link() {
        let mut links: Vec<String> = Vec::new();
        // take the next link in the queue
        println!("Crawling link: {}", &link.name());
        let response = match client
            .get(link.name().clone())
            .send() {
                Ok(result) => result,
                Err(..) => continue,
            };

        let html: Html = Html::parse_document(&response.text().unwrap());
        let selector = Selector::parse("a").unwrap();

        let href = html.select(&selector);
        for link in href {
            if let Some(cur) = link.attr("href") {
                links.push(String::from(cur));
            }
        }

        // normalize the link
        let mut full_links = Vec::new();
        links.into_iter().map(|x| {
            link_normalize(&link, &x)
        }).for_each(|x|{
            let mut links = link_traverse(&x);
            full_links.append(&mut links)
        });
        let links = full_links;


        // calculate the weight of a link
        // the goal is to have a weight which wont explode too quickly
        // (ideally slower than +1 weight per link)
        // but at the same time accurately put links that appear more higher on the list
        let linkweight = ((link.weight() * 10.0)
                          / links.len() as f64)
            .log2();

        links.into_iter().for_each(|x| {
            crawler.submit(&x, linkweight);
        });
        // Stop adding links to the queue after the queue gets larger than a certan point
        if crawler.queue_len() > QUEUE_RESULTS_MAX {
            crawler.set_stop(true);
        }

        thread::sleep(Duration::from_millis(250));
    }

    let mut results = crawler.results();
    results.sort_by(|a,b|{
        a.weight()
            .partial_cmp(&b.weight())
            .unwrap_or_else(||{Ordering::Less})});

    results.into_iter().for_each(|x|{
            println!("{}:{}",x.name(), x.weight());
        });
}
