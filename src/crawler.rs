use std::collections::{
    HashMap,
    VecDeque
};
use crate::link::*;

pub struct Crawler {
    stop: bool,
    queue: VecDeque<Link<ResultLink>>,
    results: HashMap<String, Link<ResultLink>>,
}

impl Crawler {
    pub fn new(seed: &[&str]) -> Crawler {
         let mut crawler = Crawler {
             stop: false,
             queue: VecDeque::new(),
             results: HashMap::new(),
         };
         seed.iter().for_each(|x| {
            // the seed links need to be valid for the code to work
            // the normalize function should catch these as the method
            // requires a valid URI
            // FIXME: make Link::normalize() return a Result to handle this error more gracefully,
            // as the link seed will be a user input in the future,
            // or have a validate() method return a bool
            let link: Link<RawLink> = Link::<RawLink>::new(x);
            let link = link.normalize(&Link::<ResultLink>::default());
            crawler.queue.push_back(link.submit(ResultLink {weight: 100.0}));
         });
        
         crawler
    }

    pub fn queue_len(&self) -> usize {
        self.queue.len()
    }

    pub fn submit(&mut self, link: Link<NormalizedLink>, weight: f64){
        if let Some(result) = self.results.get_mut(&link.name()){
            result.add_weight(weight);
        } else {
            let link = link.submit(ResultLink { weight });
            if !self.stop(){
                self.queue.push_back(link.clone());
            }
            self.results.insert(String::from(&link.name()), 
                                link);
        }
    }

    pub fn pop_link(&mut self) -> Option<Link<ResultLink>>{
        self.queue.pop_front()
    }

    pub fn stop(&self) -> bool{
        self.stop
    }

    pub fn set_stop(&mut self,stop: bool){
        self.stop = stop; 
    } 

    pub fn results(&mut self) -> Vec<&Link<ResultLink>> {
        let result: Vec<&Link<ResultLink>> = self.results.iter().map(|x| {
            x.1
        }).collect();

        result
    }
}

#[cfg(test)]
mod test {
    use crate::link::{Link,NormalizedLink};

    use super::Crawler;

    #[test]
    fn crawler_init(){
        let mut crawler = Crawler::new(&["https://example.org/"]);

        assert_eq!("https://example.org/", crawler.pop_link().expect("No link found in queue!").name());
        assert!(crawler.pop_link().is_none())
    }

    #[test]
    fn crawler_queue(){
        let mut crawler = Crawler::new(&["https://example.org/"]);

        assert_eq!(1, crawler.queue_len());
        let _ = crawler.pop_link();
        assert_eq!(0, crawler.queue_len());
        crawler.submit(Link { 
                name: "https://example.org/test/".to_string(), 
                state: NormalizedLink}, 
            100.0);
        assert_eq!(1, crawler.queue_len());
        crawler.submit(Link { 
                name: "https://example.org/test/".to_string(), 
                state: NormalizedLink}, 
            100.0);
        assert_eq!(1, crawler.queue_len());
    }

    #[test]
    fn crawler_results() {
        let mut crawler = Crawler::new(&["https://example.org/"]);

        crawler.submit(Link { 
                name: "https://example.org/test/".to_string(), 
                state: NormalizedLink}, 
            100.0);
        crawler.submit(Link { 
                name: "https://example.org/test/".to_string(), 
                state: NormalizedLink}, 
            100.0);
        let results = crawler.results();
        let first_result = results.first().expect("No link was found in results!");
        assert_eq!("https://example.org/test/", first_result.name());
        assert_eq!(200.0, first_result.weight());
    }
}
