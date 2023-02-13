use std::fs;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Feeds{
    categories:Vec<Categorie>,
}

impl Feeds{
    pub fn get_categories_names(&self) -> Vec<String>{
        let mut categories_string: Vec<String> = Vec::new();
        for categorie in &self.categories{
            categories_string.push(categorie.get_name());
        }
        categories_string
    }

    pub fn get_categories(&self) -> Vec<Categorie>{
        let mut categories:Vec<Categorie> = Vec::new();
        for c in &self.categories{
            categories.push(c.to_categorie())
        }
        categories
    }

}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Categorie{
    name:String,
    sources:Vec<Source>,
}

impl Categorie {
    fn new(input_name:String) -> Categorie{
        Categorie { 
            name: input_name, 
            sources: Vec::new() 
        }
    }

    pub fn get_name(&self) -> String{
        self.name.clone()
    }

    fn add_source(&mut self, source:Source){
        self.sources.push(source);
    }

    fn to_categorie(&self) -> Categorie{
        let mut categorie: Categorie = Categorie::new(self.name.clone());
        for source in &self.sources {
            categorie.add_source(source.to_source())
        }

        categorie
    }

    pub fn get_sources(&self) -> Vec<Source> {
        let mut sources: Vec<Source> = Vec::new();
        for source in &self.sources{
            sources.push(source.to_source());
        }
        sources
    }

}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Source{
    title:String,
    url:String,
}

impl Source{
    fn to_source(&self) -> Source {
        Source{
            title: self.title.clone(),
            url: self.url.clone()
        } 
    }

    pub fn get_name(&self) -> String {
        self.title.clone()
    }

    pub fn get_url(&self) -> String {
        self.url.clone()
    }
}

pub fn get_feeds() -> Feeds{
    let file_path = "src/assets/feeds.yaml";
    let file_data = fs::read_to_string(file_path).unwrap();
    let feeds:Feeds = serde_yaml::from_str(&file_data).unwrap();
    feeds
}