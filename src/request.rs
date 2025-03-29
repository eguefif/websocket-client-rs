#[allow(dead_code)]
#[derive(Debug)]
pub struct Request {
    status: String,
    version: String,
    verbose_status: String,
    headers: Vec<(String, String)>,
}

impl Request {
    pub fn new(response: &str) -> Self {
        println!("Response: {:?}", response);
        Self {
            status: get_status(&response),
            version: get_version(&response),
            verbose_status: get_verbose_status(&response),
            headers: get_headers(&response),
        }
    }

    pub fn get_value(&self, key: &str) -> Option<&str> {
        for (header_key, value) in self.headers.iter() {
            if header_key == key {
                return Some(value);
            }
        }
        None
    }
}

fn get_status(response: &str) -> String {
    let mut splits = response.split_ascii_whitespace();
    splits.next().unwrap();
    splits.next().unwrap().to_string()
}

fn get_version(response: &str) -> String {
    let mut splits = response.split_ascii_whitespace();
    splits.next().unwrap().to_string()
}

fn get_verbose_status(response: &str) -> String {
    let mut splits = response.split_ascii_whitespace();
    splits.next().unwrap();
    splits.next().unwrap();
    splits.next().unwrap().to_string()
}

fn get_headers(response: &str) -> Vec<(String, String)> {
    let mut headers: Vec<(String, String)> = Vec::new();
    let mut lines = response.lines();
    lines.next().unwrap();
    for line in lines {
        let mut splits = line.split(":");
        let key = splits.next().unwrap().to_string();
        let value = splits.next().unwrap().to_string();
        headers.push((key, value))
    }
    headers
}
