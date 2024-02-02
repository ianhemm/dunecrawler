use http::Uri;

pub trait LinkState{}

pub struct RawLink;
impl LinkState for RawLink{}
pub struct NormalizedLink;
impl LinkState for NormalizedLink{}

#[derive(Debug,Clone)]
pub struct ResultLink {
    pub weight:     f64,
    // TODO update: SystemTime,
    // TODO title:  Option<String>,
    // TODO desc:   Option<String>,
}
impl LinkState for ResultLink{}


#[derive(Debug,Clone)]
pub struct Link<State: LinkState>{
    name: String, 
    state: State
}

impl<State: LinkState> Link<State> {

    pub fn default() -> Link<ResultLink> {
        Link{name: "".to_string(), state: ResultLink {weight: 0.0}}
    }

    pub fn name(self: &Self) -> String {
        String::from(&self.name)
    }
}

impl Link<RawLink>{
    pub fn new(link:&str) -> Link<RawLink> {
        Link{name: String::from(link), state: RawLink}
    }

    pub fn normalize(self: &Self, parent: &Link<ResultLink>) -> Link<NormalizedLink> {
        let mut link = String::from(self.name());

        if link.starts_with('/') { // '/' is the root level, meaning we will have to use the top level
            let uri = parent.name().parse::<Uri>().unwrap();
            let host = uri.host().unwrap();

            link.remove(0);
            link = format!("https://{host}/{link}");
        }
        if link.starts_with('.'){
            link.remove(0);
            link.remove(0);
            link = format!("{}/{}", parent.name(), link);
        }

        Link{
            name: link, 
            state: NormalizedLink
        }
    }

}

impl Link<NormalizedLink> {
    /** 
     * API implementation of link_traverse(&String) requiring that the link is normalized
     *
     */
    pub fn traverse(self: &Self) -> Vec<Link<NormalizedLink>> {
        let mut result: Vec<Link<NormalizedLink>> = vec![];
        for link in link_traverse(&self.name()) {
            // If the parent link is already normalized, 
            // then the traversed links are also valid
            // to submit
            let link = Link{name: link, state: NormalizedLink};
            result.push(link);
        }

        result
    }

    pub fn submit(self: Self, data: ResultLink) -> Link<ResultLink> {
        Link {
            name: self.name, 
            state: data 
        }
    }
}

impl Link<ResultLink> {
    pub fn weight(self: &Self) -> f64 {
        self.state.weight
    }
    pub fn add_weight(self: &mut Self, weight: f64) {
        self.state.weight += weight
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
 * assert!(links.contains("https://example.org"))
 * assert!(links.contains("https://example.org/test/"))
 * assert!(links.contains("https://example.org/test/foo/"))
 * assert!(links.contains("https://example.org/test/foo/bar/"))
 * ```
 **/
fn link_traverse(link: &str) -> Vec<String>{
    let mut result: Vec<String> = Vec::new();
    let uri = match link.parse::<Uri>() {
        Ok(uri) => uri,
        Err(_) => return vec![String::from(link)], // just give back the link as is if theres an error
    };
    let host = uri.host().expect("Link not normalized! No host!");
    let path = uri.path();
    //println!("{}", path);

    let mut path_builder = format!("https://{}",host);
    for link in path.split("/"){
        if !link.is_empty() { // deals with split accidentally treating the whitespace before the 
                              // first '/' as its own empty string
            result.push(String::from(&path_builder));
            path_builder = format!("{}/{}",path_builder,link)
        }
    }
    result.push(path_builder);


    result
}
