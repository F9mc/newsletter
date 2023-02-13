#[allow(unused_imports)]
use log::{debug, warn, error, info};
use rss::{Channel};
use chrono::{DateTime, Duration, Utc};

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

    pub fn add_source(&mut self, source:Source) {
        self.sources.push(source);
    }
}

pub struct Source{
    name:String,
    posts:Vec<Post>,
}

impl Source{
    pub fn new(name:String) -> Source{
        Source{
            name:name,
            posts: Vec::new()
        }
    }

    pub fn build(name:String, posts:Vec<Post>) -> Source {
        Source{
            name:name,
            posts:posts
        }
    }

    pub async fn build_from_url(url:String) -> Source{
        let content = reqwest::get(url)
        .await.unwrap()
        .bytes()
        .await.unwrap();
        let channel = Channel::read_from(&content[..]).unwrap();
        let last_posts:Vec<Post> = get_last_post_by_channel(&channel);

        let source: Source = Source::build(channel.title, last_posts);
    
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
}


fn get_last_post_by_channel(channel: &Channel) -> Vec<Post> {
    let mut results:Vec<Post> = Vec::new();

    for post in &channel.items{
        let date = DateTime::parse_from_rfc2822(post.pub_date().unwrap()).unwrap();
        if Utc::now() + Duration::days(-1) < date{
            debug!("Find {} from {}", &post.title().unwrap(), &post.pub_date().unwrap());

            results.push(Post{
                title:post.title.as_ref().unwrap().to_string(),
                link:post.link.as_ref().unwrap().to_string()
            });
        }
    }
    results
}


