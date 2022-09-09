use std::collections::HashMap;
use std::fs;
use std::io::prelude::*;

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
        .open("README.md")
        .unwrap();
    file.write_all(bytes).expect("Could not write!");
    Ok(())
}

async fn gen_readme_text(
    joke: Vec<HashMap<String, String>>,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut file = fs::File::open("src/sample_readme.md")?;
    let mut contents = String::new();
    let joke_str = format!(
        "<b>Joke of the day!</b> (Come again tommorrow for a new one 😎)\n{}\n{}",
        "<b>".to_owned() + &joke[0]["question"] + &"</b>".to_owned(),
        "<i>".to_owned() + &joke[0]["punchline"] + &"</i>".to_owned()
    );
    file.read_to_string(&mut contents)?;
    contents = contents.replace("{question_boilerplate}", &joke_str);
    Ok(contents)
}

async fn get_joke() -> Result<Vec<HashMap<String, String>>, Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://backend-omega-seven.vercel.app/api/getjoke")
        .await?
        .json::<Vec<HashMap<String, String>>>()
        .await?;
    Ok(resp)
}
