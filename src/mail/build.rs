use build_html::*;
use crate::custom_rss::{Section};



pub fn build_mail_msg(posts:Vec<Section>) -> String {
    let mut html: String = HtmlPage::new();
        .with_header(1, "Newsletter")

    for section in posts{
        html.with_container(
            Container::new(ContainerType::Article)
                .with_attributes([("id", "article1")])
                .with_header_attr(2, "Hello, World", [("id", "article-head"), ("class", "header")])
                .with_paragraph("This is a simple HTML demo")
        )
    }



        
    html.to_html_string();
}