use std::fs;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Feeds{
    categories:Vec<Categories>,
}

impl Feeds{
    pub fn get_categories(&self) -> Vec<String>{
        let mut categories_string: Vec<String> = Vec::new();
        for categorie in &self.categories{
            categories_string.push(categorie.get_name());
        }
        categories_string
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Categories{
    name:String,
    sources:Vec<Feed>,
}

impl Categories {
    fn get_name(&self) -> String{
        let name = &self.name;
        name.to_string()
    }

}


#[derive(Serialize, Deserialize, Debug, Clone)]
struct Feed{
    title:String,
    url:String,
}

impl Feed{

}

pub fn get_feeds() -> Feeds{
    let file_path = "src/assets/feeds.yaml";
    let file_data = fs::read_to_string(file_path).unwrap();
    let feeds:Feeds = serde_yaml::from_str(&file_data).unwrap();

    feeds
}