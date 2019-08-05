use std::collections::HashMap;

const HEADER_PARSING_ERROR: &'static str = "Header must be specified as key: value";

pub fn body<T: Into<String>>(body: Option<T>) -> hyper::Body {
    match body {
        None => hyper::Body::empty(),
        Some(body) => {
            let body = body.into();
            if body.len() > 0 {
                hyper::Body::from(body)
            } else {
                hyper::Body::empty()
            }
        }
    }
}

pub fn headers(headers: Option<clap::Values>) -> HashMap<String, String> {
    match headers {
        None => HashMap::new(),
        Some(h) => {
            let mut ret: HashMap<String, String> = HashMap::new();
            for header in h {
                let mut split = header.split(':');
                ret.insert(
                    split
                        .clone()
                        .nth(0)
                        .expect(HEADER_PARSING_ERROR)
                        .trim()
                        .to_string(),
                    split.nth(1).expect(HEADER_PARSING_ERROR).trim().to_string(),
                );
            }
            ret
        },
    }
}
