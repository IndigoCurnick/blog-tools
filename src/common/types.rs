use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

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
}
