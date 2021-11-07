use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Item {
    pub title: String,
    pub subtitle: String,
    pub description: String,
    #[serde(rename = "pubDate")]
    pub pub_date: String,
    pub enclosure: Enclosure,
    pub image: Image,
    pub link: String,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Enclosure {
    pub url: String,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Image {
    pub href: String,
}
