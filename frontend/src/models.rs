use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Artifact {
    pub title: String,
    pub description: String,
    pub tags: Vec<String>,
    pub link: Option<String>,   // URL to the document/video
    pub link_text: String,      // "Read Report", "Watch Video"
    pub icon: String,           // SVG string
}