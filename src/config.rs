use crate::request::Request;
use std::fs::File;
use std::io::prelude::*;
use toml;

#[derive(Debug, Deserialize, Serialize)]
pub struct Project {
    pub requests: Vec<Request>,
}

impl Project {
    pub fn create(&self) -> std::io::Result<()> {
        let mut file = File::create(".hat.toml")?;
        file.write_all(toml::to_string(self).unwrap().as_bytes())?;
        Ok(())
    }
}
