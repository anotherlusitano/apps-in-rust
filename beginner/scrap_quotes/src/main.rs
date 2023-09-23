use scraper::{Html, Selector};

fn main() {
    let url = "https://quotes.toscrape.com/";
    let response = reqwest::blocking::get(url).expect("Could not load url.");
    let html = response.text().unwrap();

    let document = Html::parse_document(html.as_str());

    let quotes = Selector::parse(r#"span[class="text"]"#).unwrap();
    let authors = Selector::parse(r#"small[class="author"]"#).unwrap();

    for (element_quotes, element_authors) in document.select(&quotes).zip(document.select(&authors))
    {
        let quotes_text = element_quotes.text().collect::<Vec<_>>();
        let authors_text = element_authors.text().collect::<Vec<_>>();
        println!("{} - {} \n", quotes_text[0], authors_text[0]);
    }
}
