mod feeds;
use feeds::get_feeds;
use crate::feeds::Feeds;
use crate::feeds::Source;

use log::{debug, warn, error, info, Level};

use dotenv::dotenv;


fn main() {
    dotenv().ok();
    env_logger::init();
    info!("Starting");
    let feeds:Feeds = get_feeds();
    for categorie in feeds.get_categories(){
        let categorie_name: String = categorie.get_name();
        for source in categorie.get_sources(){
            debug!("{categorie_name} - {:}", source.get_name());
        }
    }
}
