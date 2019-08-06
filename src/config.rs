use crate::request::Request;
use std::fs::File;
use std::io::prelude::*;
use toml;

#[derive(Debug, Deserialize, Serialize)]
pub struct Project {
    pub requests: Vec<Request>,
}

impl Project {
    fn load() -> std::io::Result<String> {
        let mut file = File::open(".hat.toml")?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        Ok(content)
    }

    pub fn get() -> Self {
        toml::from_str(
            &Project::load()
                .expect("Could not load your .hat.toml file. Make sure it is in your current working directory."),
        ).expect("Your .hat.toml file have a syntax error.")
    }

    pub fn _create(&self) -> std::io::Result<()> {
        let mut file = File::create(".hat.toml")?;
        file.write_all(toml::to_string(self).unwrap().as_bytes())?;
        Ok(())
    }
}

impl std::fmt::Display for Project {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Requests: {:?}", self.requests)
    }
}
