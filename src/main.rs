use std::collections::VecDeque;
use core::cmp::Ordering;

use reqwest::blocking::Client;
use scraper::{Html, Selector};
use std::{thread, time::Duration};
use http::Uri;

use dunecrawler::crawler::Crawler;


const QUEUE_RESULTS_MAX:usize = 100000; // number of results in the queue before the stop flag is raised
const _QUEUE_THREADS:u8 = 4; // number of threads to spawn in the thread pool

fn main() {
    let seed: &[&str] = &[
        "https://wikipedia.org",
        "https://neocities.org/browse",
        "https://youtube.com",
    ]; // List of URLs to start searching
    let _database = ""; // database to cache results in

    // the download queue that all download threads will read from
    let mut queue: VecDeque<String> = VecDeque::new();
    seed.iter().for_each(|x| {
        queue.push_back(x.to_string());
    });

    // All the results currently cached.
    // This will contain all strong pointers to their results and return weak pointers
    // This will be used when looking for link duplicates

    let client = Client::new();
    let mut crawler = Crawler::new(seed);

    // while we have links in the queue
    while let Some(link) = crawler.pop_link() {
        let mut links: Vec<String> = Vec::new();
        // take the next link in the queue
        println!("Parsing link: {}", &link.name());
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
        links = links.into_iter().map(|mut x| {
            // get the current domain of the site
            let uri = link.name().parse::<Uri>().unwrap();
            let host = uri.host().unwrap();
            if x.starts_with('/') { // '/' is the root level, meaning we will have to use the top level
                x.insert_str(0, &String::from("https://".to_owned() + host));
            }
            if x.starts_with('.'){
                x.remove(0);
                x.remove(0);
                x.insert_str(0, &String::from(&link.name()));
            }

            x
        }).collect();


        // if the link is not currently in the results AND the database doesnt contain the link
        let linkweight = (link.weight() * 10.0) / links.len() as f64;
        links.into_iter().for_each(|x| {
            crawler.submit(&x, linkweight);
        });
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
