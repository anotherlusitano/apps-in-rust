use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Joke {
    id: String,
    joke: String,
    status: i32,
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let joke_data: Joke = reqwest::Client::new()
        .get("https://icanhazdadjoke.com/")
        .header("Accept", "application/json")
        .send()
        .await?
        .json()
        .await?;

    println!("{:#?}", joke_data.joke);

    Ok(())
}
