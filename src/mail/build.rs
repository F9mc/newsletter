use build_html::*;




pub fn build() -> String {
    let html: String = HtmlPage::new();
        .with_header(1, "Header")



        
    html.to_html_string();
}