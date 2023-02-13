
pub mod EmailFunction{
    #[allow(unused_imports)]
    use log::{debug, warn, error, info};
    use crate::custom_rss::{Section};
    use crate::mail::EmailBody::{build_mail_msg};

    pub fn send_mail(posts: Vec<Section>){
        let body:String = build_mail_msg(posts);
        debug!("Body : {:#?}", body);
    }
}

mod EmailBody{
    use build_html::*;
    use crate::custom_rss::{Section};



    pub fn build_mail_msg(posts:Vec<Section>) -> String {
        let mut html = HtmlPage::new();
            html.add_header(1, "Newsletter");

        for section in posts{
            let mut section_container = Container::new(ContainerType::Article);
            section_container.add_header_attr(2, section.get_name(), [("class", "section-header")]);

            for source in section.get_sources() {
                let mut source_container = Container::new(ContainerType::Article);
                source_container.add_header_attr(3, source.get_name(), [("class", "source-header")]);
                
                for post in source.get_posts() {
                    let mut post_container = Container::new(ContainerType::Article);
                    post_container.add_link_attr(post.get_link(),post.get_title(), [("class", "post-header")]);

                    source_container.add_container(post_container);
                }
                section_container.add_container(source_container);
            }
            html.add_container(section_container);
        }



        
    html.to_html_string()
    }
}