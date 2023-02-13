mod feeds;
mod custom_rss;
mod mail;

#[allow(unused_imports)]
use log::{debug, warn, error, info};
use dotenv::dotenv;
use crate::feeds::{Feeds, get_feeds};
use crate::custom_rss::{Source, Section};
use crate::mail::email_function;


#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();

    let mut posts:Vec<Section> = Vec::new();

    let feeds:Feeds = get_feeds();
    for categorie in feeds.get_categories(){
        let mut sections:Section = Section::new(categorie.get_name());
        
        for source in categorie.get_sources(){           
            debug!("Source {:}", source.get_name());

            let source:Source = Source::build_from_url(source.get_url(), source.get_name()).await;
            sections.add_source(source);

        }

        posts.push(sections);
    }

    email_function::send_mail(posts);
}
