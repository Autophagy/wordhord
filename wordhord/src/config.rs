use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::path::Path;
use std::slice::Iter;

#[derive(Clone, Deserialize, Serialize, PartialEq)]
pub enum Tag {
    Nix,
}

impl Tag {
    pub fn iterator() -> Iter<'static, Tag> {
        static TAGS: [Tag; 1] = [Tag::Nix];
        TAGS.iter()
    }
}

impl fmt::Display for Tag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let tag = match self {
            Tag::Nix => "Nix",
        };
        write!(f, "{}", tag)
    }
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Post {
    pub title: String,
    pub published: NaiveDate,
    pub slug: String,
    pub tags: Vec<Tag>,
    pub content: String,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Config {
    pub hord: Vec<Post>,
    pub drv: String,
    pub build_dir: String,
    pub repo: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("Not enough arguments");
        }
        let config_path = args[1].clone();

        if !Path::new(&config_path).exists() {
            return Err("Configfile does not exist");
        }

        let parsed_config: Result<Config, _> = serde_dhall::from_file(&args[1]).parse();
        match parsed_config {
            Ok(config) => Ok(config),
            Err(_) => Err("Error parsing config"),
        }
    }
}
