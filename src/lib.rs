pub mod crawler;


#[derive(Debug,Clone)]
pub struct LinkResult {
    pub name: String,
    pub weight: f64,
}

impl LinkResult {
    pub fn new(link:&str, weight: f64) -> LinkResult {
        LinkResult { name: link.to_string(), weight }
    }

    pub fn name(self: &Self) -> String {
        return String::from(self.name.clone());
    }

    pub fn weight(self: &Self) -> f64 {
        return self.weight;
    }

    pub fn add_weight(self:&mut Self,weight: f64) {
        self.weight += weight;
    }
}





