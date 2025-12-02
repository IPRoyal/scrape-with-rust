#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::builder().build()?;

    let response = client
        .get("https://news.ycombinator.com/")
        .send()
        .await?
        .text()
        .await?;

    let document = scraper::Html::parse_document(&response);

    let title_selector = scraper::Selector::parse("span.titleline>a").unwrap();
    let titles = document.select(&title_selector).map(|x| x.inner_html());

    let subtext_selector = scraper::Selector::parse("td.subtext").unwrap();
    let subtexts = document.select(&subtext_selector);

    let score_selector = scraper::Selector::parse("span.score").unwrap();

    let scores = subtexts.map(|subtext| {
        subtext
            .select(&score_selector)
            .next()
            .and_then(|score| score.text().nth(0))
            .unwrap_or("0 points")
    });

    titles.zip(scores).for_each(|pair| println! {"{:?}", pair});
    Ok(())
}
