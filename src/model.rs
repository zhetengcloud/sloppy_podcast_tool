use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
pub struct Item {
    pub title: String,
    pub subtitle: String,
    pub description: String,
    #[serde(rename = "pubDate")]
    pub pub_date: String,
    pub enclosure: Enclosure,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Enclosure {
    pub url: String,
}
