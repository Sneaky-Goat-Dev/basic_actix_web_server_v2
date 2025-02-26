use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct PostItem {
    name: String,
    description: Option<String>,
}

impl PostItem {
    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_description(&self) -> &Option<String> {
        &self.description
    }
}

#[derive(Serialize)]
pub struct InsertedItem {
    pub id: i32,
}
