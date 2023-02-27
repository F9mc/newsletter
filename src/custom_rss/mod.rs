#[allow(unused_imports)]
use log::{debug, warn, error, info};
use rss::{Channel};
use chrono::{DateTime, Duration, Utc};
use std::env;

pub struct Section{
    name:String,
    sources:Vec<Source>,
}

impl Section{
    pub fn new(name:String) -> Section {
        Section{
            name:name,
            sources:Vec::new()
        }
    }

    pub fn is_empty(&self) -> bool{
        let mut result: bool = true;
        for s in self.get_sources(){
            result = result & s.is_empty();
        }
        result
    }

    pub fn add_source(&mut self, source:Source) {
        self.sources.push(source.to_source());
    }

    pub fn get_sources(&self) -> Vec<Source> {
        let mut sources:Vec<Source> = Vec::new();
        for s in &self.sources{
            sources.push(s.to_source());
        }
        sources
    }

    pub fn get_name(&self) -> String{
        self.name.clone()
    }
}

pub struct Source{
    name:String,
    posts:Vec<Post>,
}

impl Source{
    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn new(name:String) -> Source {
        Source{
            name:name,
            posts: Vec::new()
        }
    }

    pub fn is_empty(&self) -> bool{
        self.get_posts().len() == 0
    }

    pub fn build(name:String, posts:Vec<Post>) -> Source {
        Source{
            name:name,
            posts:posts
        }
    }

    pub fn to_source(&self) -> Source{
        let mut source = Source::new(self.name.clone());
        for p in &self.posts{
            source.add_post(p.to_post());
        }
        source
    }

    pub fn get_posts(&self) -> Vec<Post> {
        let mut posts:Vec<Post> = Vec::new();
        for p in &self.posts{
            posts.push(p.to_post());
        }
        posts
    }

    pub fn add_post(&mut self, post:Post){
        self.posts.push(post)
    }

    pub async fn build_from_url(url:String, name:String) -> Source{
        let content = reqwest::get(url)
        .await.unwrap()
        .bytes()
        .await.unwrap();
        let channel = Channel::read_from(&content[..]).unwrap();
        let last_posts:Vec<Post> = get_last_post_by_channel(&channel);

        let source: Source = Source::build(name, last_posts);
    
        source
    }
}

pub struct Post{
    title: String,
    link: String
}

impl Post{
    pub fn new(title:String, link:String) -> Post {
        Post{
            title:title,
            link:link
        }
    }

    pub fn get_title(&self) -> String {
        self.title.clone()
    }

    pub fn get_link(&self) -> String {
        self.link.clone()
    }

    pub fn to_post(&self) -> Post{
        Post::new(self.title.clone(), self.link.clone())
    }
}


fn get_last_post_by_channel(channel: &Channel) -> Vec<Post> {
    let mut results:Vec<Post> = Vec::new();
    let env_time_ago = match env::var_os("PUB_TIME") {
        Some(v) => v.into_string().unwrap(),
        None => panic!("$PUB_TIME is not set")
    };      

    for post in &channel.items{

        match post.pub_date() {
            Some(parsed_date) => {
                match DateTime::parse_from_rfc2822(parsed_date){
                    Ok(date) => {
                        if Utc::now() + Duration::days(env_time_ago.parse().unwrap()) < date{
                            debug!("Find {} from {}", &post.title().unwrap(), &post.pub_date().unwrap());
                
                            results.push(Post{
                                title:post.title.as_ref().unwrap().to_string(),
                                link:post.link.as_ref().unwrap().to_string()
                            });
                        }
                    },
                    Err(_) => error!("Date could not be parsed")
                }
                
            },
            None => {
                error!("Date not found")
            }

        }


    }
    results
}


