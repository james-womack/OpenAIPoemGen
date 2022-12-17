mod ai;
mod constants;

use async_openai as openai;

use openai::Client;
use rand::seq::SliceRandom;

use crate::constants::POEM_PROMPTS;
use ai::AiSettings;
use constants::POEM_TYPES;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new()
        .with_api_key(std::fs::read_to_string("./token").expect("Unable to load token"));
    let rand_type = POEM_TYPES
        .choose(&mut rand::thread_rng())
        .expect("No poem types");
    let prompt = POEM_PROMPTS
        .choose(&mut rand::thread_rng())
        .expect("No poem prompts");

    let full_text = ai::get_ai_response(
        "Write a prompt for a poem about robots that compete with other robots",
        AiSettings::new_prompt(),
        &client,
    )
    .await?;

    let generated_poem_prompt = format!("Write a {rand_type} about {full_text}");
    let title_prompt = format!("Write a title for a {rand_type} about {full_text}");
    println!("{}", "-".repeat(80));
    println!("INPUT PROMPT: {prompt}");
    println!("{}", "-".repeat(80));
    println!("GENERATED PROMPT: {generated_poem_prompt}");
    println!("{}", "=".repeat(80));
    let title = ai::get_ai_response(&title_prompt, AiSettings::new_title(), &client).await?;
    println!("{title}");
    println!("{}", "=".repeat(80));
    let full_text =
        ai::get_ai_response(&generated_poem_prompt, AiSettings::new_poem(), &client).await?;
    println!("{full_text}");
    println!("{}", "-".repeat(80));
    ai::save_to_file(&title, prompt, &generated_poem_prompt, &full_text)
        .expect("Failed to save file");
    Ok(())
}
