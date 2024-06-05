use std::{fmt::Display, io};

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use std::error::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct BlogJson {
    pub title: String,
    pub date: NaiveDate,
    pub desc: Option<String>,
    pub slug: String,
    pub tags: Vec<String>,
    pub keywords: Option<Vec<String>>,
    pub canonical_link: Option<String>,
    pub author_name: Option<String>,
    pub author_webpage: Option<String>,
    pub last_modified: Option<NaiveDate>, // For sitemap, if not present uses `date`
    pub priority: Option<f64>, // For sitemap, if not present uses the default 
}

#[derive(Debug)]
pub enum BlogError {
    File(io::Error),
    Markdown(String),
    FileNotFound, // TODO: It would be nice to include the file not found here but that would rely on converting a Path to a String - which involves TWO unwraps!
    /// Include the date as found as a string
    ImproperDate(String),
}

impl Error for BlogError {}

impl Display for BlogError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BlogError::File(a) => write!(f, "File read error caused by: {}", a),
            BlogError::Markdown(b) => write!(f, "Markdown rendering error caused by: {}", b),
            BlogError::FileNotFound => write!(f, "File not found"),
            BlogError::ImproperDate(c) => write!(f, "Found date `{}` which appears to be improper - dates should be in the yyyy-mm-dd format", c)
        }
    }
}
