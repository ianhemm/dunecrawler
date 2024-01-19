use std::collections::{
    HashMap,
    VecDeque
};
use super::LinkResult;

pub struct Crawler {
    stop: bool,
    queue: VecDeque<LinkResult>,
    results: HashMap<String, LinkResult>,
    _database: Option<sqlite3::Connection>,
}

impl Crawler {
    pub fn new(seed: &[&str]) -> Crawler {
         let mut crawler = Crawler {
             stop: false,
             queue: VecDeque::new(),
             results: HashMap::new(),
             _database: None,
         };
         seed.iter().for_each(|x| {
            crawler.queue.push_back(LinkResult::new(x, 100.0));
         });
        
         crawler
    }

    pub fn queue_len(self: &Self) -> usize {
        self.queue.len()
    }

    pub fn submit(self: &mut Self, link: &str, weight: f64){
        if let Some(result) = self.results.get_mut(link){
            result.add_weight(weight);
        } else {
            if !self.stop(){
                self.queue.push_back(
                    LinkResult {
                        name: String::from(link), 
                        weight});
            }
            self.results.insert(String::from(link),
                LinkResult {
                    name: String::from(link), 
                    weight});
        }
    }

    pub fn pop_link(self: &mut Self) -> Option<LinkResult>{
        self.queue.pop_front()
    }

    pub fn stop(self: &Self) -> bool {
        self.stop
    }

    pub fn set_stop(self: &mut Self,stop: bool){
        self.stop = stop; 
    }

    pub fn results(self: &mut Self) -> Vec<&LinkResult> {
        let result: Vec<&LinkResult> = self.results.iter().map(|x| {
            x.1
        }).collect();

        result
    }
}
