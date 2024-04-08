use std::fs;
use std::io::prelude::*;
use serde::Deserialize;

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct JokeResponse {
    error: bool,
    category: String,
    #[serde(rename = "type")]
    joke_type: String,
    setup: String,
    delivery: String,
    flags: Flags,
    id: u32,
    safe: bool,
    lang: String,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Flags {
    nsfw: bool,
    religious: bool,
    political: bool,
    racist: bool,
    sexist: bool,
    explicit: bool,
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let joke = get_joke().await.unwrap();
    let text = gen_readme_text(joke).await.unwrap();
    let text_bytes = text.as_bytes();
    write_readme(text_bytes).await.unwrap();
    Ok(())
}

async fn write_readme(bytes: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .open("README.md")
        .unwrap();
    file.write_all(bytes).expect("Could not write!");
    Ok(())
}

async fn gen_readme_text(
    joke: JokeResponse,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut file = fs::File::open("src/sample_readme.md")?;
    let mut contents = String::new();
    let joke_str = format!(
        "<b>Joke of the day!</b> (Come again tommorrow for a new one 😎)<br>{}<br>{}",
        "<b>".to_owned() + &joke.setup + &"</b>".to_owned(),
        "<i>".to_owned() + &joke.delivery + &"</i>".to_owned()
    );
    file.read_to_string(&mut contents)?;
    contents = contents.replace("{question_boilerplate}", &joke_str);
    Ok(contents)
}

async fn get_joke() -> Result<JokeResponse, Box<dyn std::error::Error>> {
    let url = "https://v2.jokeapi.dev/joke/Programming?blacklistFlags=nsfw,religious,political,racist,sexist,explicit&type=twopart";
    let resp = reqwest::get(url)
        .await?
        .json::<JokeResponse>()
        .await?;

    Ok(resp)
}
