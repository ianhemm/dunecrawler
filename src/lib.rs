pub mod crawler;

use http::Uri;

const _CREATE_TABLE: &str = "CREATE TABLE Page (
    id SERIAL PRIMARY KEY
    title VARCHAR
    update DATETIME
    description VARCHAR
    link VARCHAR
    weight REAL
    )";


#[derive(Debug,Clone)]
pub struct LinkResult {
    pub name:       String,
    pub weight:     f64,
    // TODO update: SystemTime,
    // TODO title:  Option<String>,
    // TODO desc:   Option<String>,
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

#[cfg(test)]
mod test {
    use crate::LinkResult;

    #[test]
    fn link_result(){
        let result = LinkResult::new("https://example.org/", 100.0);

        assert_eq!("https://example.org/", result.name());
        assert_eq!(100.0, result.weight());
    }

    #[test]
    fn link_result_add_weight(){
        let mut result = LinkResult::new("https://example.org/", 100.0);
        result.add_weight(100.0);

        assert_eq!(200.0, result.weight());

    }
}

/**
 * Takes a normalized link and
 * traverses up to the host,
 * returning every potential URI of a site
 * from this dir
 *
 * Example:
 * ```
 * let links = link_traverse("https://example.org/test/foo/bar/");
 *
 * assert_eq!(&[
 *  "https://example.org/test/",
 *  "https://example.org/test/foo"/,
 *  "https://example.org/test/foo/bar/",
 *  "https://example.org/",
 * ], links)
 * ```
 **/
pub fn link_traverse(link: &str) -> Vec<String>{
    let mut result: Vec<String> = Vec::new();
    let uri = match link.parse::<Uri>() {
        Ok(uri) => uri,
        Err(_) => return vec![String::from(link)], // just give back the link as is if theres an error
    };
    let host = uri.host().expect("Link not normalized! No host!");
    let path = uri.path();

    let mut path_builder = format!("https://{}",host);
    for link in path.split('/') {
        result.push(String::from(&path_builder));
        path_builder = format!("{path_builder}/{link}")
    }
    result.push(path_builder);



    result
}

pub fn link_normalize(parent: &LinkResult, link: &str) -> String {
    let mut link = String::from(link);

    if link.starts_with('/') { // '/' is the root level, meaning we will have to use the top level
        let uri = parent.name().parse::<Uri>().unwrap();
        let host = uri.host().unwrap();

        link.remove(0);
        link = format!("https://{host}{link}");
    }
    if link.starts_with('.'){
        link.remove(0);
        link.remove(0);
        link = format!("{}{}", parent.name(), link);
    }

    link
}
