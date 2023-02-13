
pub mod email_function{
    #[allow(unused_imports)]
    use log::{debug, warn, error, info};
    use std::{fs, env};
    use chrono::Utc;
    use serde::{Serialize, Deserialize};
    use lettre::transport::smtp::authentication::Credentials;
    use lettre::{SmtpTransport, Transport};
    use lettre::message::{Message, SinglePart};
    use crate::custom_rss::{Section};
    use crate::mail::email_body::{build_mail_msg};
    
    #[derive(Serialize, Deserialize, Debug, Clone)]
    struct Receivers{
        receivers:Vec<String>,
    }

    impl Receivers{
        fn emails(&self) -> Vec<String>{
            let mut emails:Vec<String> = Vec::new();
            for e in &self.receivers{
                emails.push(e.clone());
            }
            emails
        }
    }
    
    fn get_receivers() -> Receivers{
        let file_path = "receiver.yaml";
        let file_data = fs::read_to_string(file_path).unwrap();
        let receivers:Receivers = serde_yaml::from_str(&file_data).unwrap();
        debug!("Receivers : {:?}", receivers);
        receivers
    }

    pub fn send_mail(posts: Vec<Section>){
        let receivers:Receivers = get_receivers();
        let body:String = build_mail_msg(posts);
        debug!("Body : {:#?}", body);

        let username = match env::var_os("USERNAME") {
            Some(v) => v.into_string().unwrap(),
            None => panic!("$USERNAME is not set")
        };    
        let password = match env::var_os("PASSWORD") {
            Some(v) => v.into_string().unwrap(),
            None => panic!("$PASSWORD is not set")
        };
        
        for receiver in receivers.emails(){
            debug!("Sending to {:#?}", receiver);
            let email = Message::builder()
            .from(username.parse().unwrap())
            .to("newsletter@pcabon.com".parse().unwrap())
            .subject(format!("Newsletter {:}", Utc::now().format("%d/%m/%Y")))
            .singlepart(SinglePart::html(body.clone())).unwrap();
            
            let creds = Credentials::new(username.clone(), password.clone());
    
            // Open a remote connection to gmail
            let mailer = SmtpTransport::relay("smtp.gmail.com")
                .unwrap()
                .credentials(creds)
                .build();
    
            // Send the email
            match mailer.send(&email) {
                Ok(_) => info!("Email sent successfully!"),
                Err(e) => panic!("Could not send email: {:?}", e),
            }
        }
    }
}

mod email_body{
    use build_html::*;
    use crate::custom_rss::{Section};



    pub fn build_mail_msg(posts:Vec<Section>) -> String {
        let mut html = HtmlPage::new();

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