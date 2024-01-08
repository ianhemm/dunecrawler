use std::collections::{VecDeque, HashMap};

use reqwest::blocking::Client;
use scraper::{Html, Selector};
use std::{thread, time::Duration};
use http::Uri;

const QUEUE_RESULTS_MAX:usize = 100000; // number of results in the queue before the stop flag is raised
const QUEUE_THREADS:u8 = 4; // number of threads to spawn in the thread pool

#[derive(Debug)]
struct Result {
    link: String, // the link to the website
    weight: f32, // the weight as a float that determines how important this site is
}

fn main() {
    let mut stop = false;

    let seed: &[&str] = &[
        "https://wikipedia.org",
        "https://neocities.org/browse",
        "https://youtube.com",
    ]; // List of URLs to start searching
    let _types_allowed: Vec<String> = Vec::new(); // List of MIMEtypes to allow
    let _database = ""; // database to cache results in

    // the download queue that all download threads will read from
    let mut queue: VecDeque<String> = VecDeque::new();
    seed.iter().for_each(|x| {
        queue.push_back(x.to_string());
    });

    // All the results currently cached.
    // This will contain all strong pointers to their results and return weak pointers
    // This will be used when looking for link duplicates
    let mut results: HashMap<String,Result> = HashMap::new();

    let client = Client::new();

    // while we have links in the queue
    while !queue.is_empty(){
        // take the next link in the queue
        let mut links: Vec<String> = Vec::new();
        if let Some(link) = queue.pop_front() {
            println!("Parsing link: {}", &link);
            let response = match client
                .get(link.clone())
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
                let uri = link.parse::<Uri>().unwrap();
                let host = uri.host().unwrap();
                if x.starts_with('/') { // '/' is the root level, meaning we will have to use the top level
                    x.insert_str(0, &String::from("https://".to_owned() + host));
                }
                if x.starts_with('.'){
                    x.remove(0);
                    x.remove(0);
                    x.insert_str(0, &String::from(&link));
                }

                x
            }).collect();


            // if the link is not currently in the results AND the database doesnt contain the link
            links.into_iter().for_each(|x| {
                if !results.contains_key(&x) {
                    if !stop {
                        queue.push_back(x.clone());
                        if queue.len() > QUEUE_RESULTS_MAX {
                            stop = true;
                        }
                    }
                    results.insert(x.clone(), Result {
                        link: x,
                        weight: 1.0,
                    });
                } else {
                    if let Some(result) = results.get_mut(&x){
                        result.weight += 1.0;
                    }
                }
            });
                    // if our stop flag hasnt been raised
                        // add it to the queue
                    // add the link to the results and set its weight to 1
                // otherwise just add a weight to the result
            // add the results to the database and update the weights in the database accordingly


            thread::sleep(Duration::from_millis(250));
        }
    }
    for result in results {
        println!("{}:{}",result.1.link, result.1.weight);
    }
}
