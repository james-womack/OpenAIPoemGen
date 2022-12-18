mod ai;
mod constants;

use async_openai as openai;
#[cfg(feature = "do_image_gen")]
use async_openai::types::ImageSize;

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
        prompt,
        AiSettings::new_prompt(),
        &client,
    )
    .await?.trim().to_string();

    let generated_poem_prompt = format!("Write a {rand_type} about:\n{full_text}");
    let title_prompt = format!("Write a title for a {rand_type} about:\n{full_text}");
    println!("{}", "-".repeat(80));
    println!("INPUT PROMPT:\n{prompt}");
    println!("{}", "-".repeat(80));
    println!("GENERATED PROMPT:\n{generated_poem_prompt}");
    println!("{}", "=".repeat(80));
    let title = ai::get_ai_response(&title_prompt, AiSettings::new_title(), &client).await?;
    let title_trimmed = title.trim();
    println!("{title}");
    println!("{}", "=".repeat(80));
    let full_text =
        ai::get_ai_response(&generated_poem_prompt, AiSettings::new_poem(), &client).await?.trim().to_string();
    println!("{full_text}");
    println!("{}", "-".repeat(80));

    // Do image generation
    #[cfg(feature = "do_image_gen")]
    {
        println!("Generating image using prompt:");
        let image_prompt = format!("Picture of the {rand_type} named {title_trimmed}\n\nCONTENT:\n\n{full_text}\n\n");
        let trimmed_prompt = image_prompt.chars().take(1000).collect::<String>();
        println!("{}", "+".repeat(80));
        println!("{trimmed_prompt}");
        println!("{}", "+".repeat(80));
        let image_data = ai::get_image(&client, &trimmed_prompt, 1, ImageSize::S512x512).await?;

        println!("Saving poem and representative image.");

        let filename = ai::save_to_file(
            title_trimmed,
            prompt,
            &generated_poem_prompt,
            &full_text,
        )
            .await
            .expect("Failed to save file");

        // Download/save image data
        match image_data.save(filename.as_str()).await {
            Ok(_) => println!("Saved image(s) to ./poem/{filename}"),
            Err(err) => println!("Failed to save images: {err:?}"),
        };
    }
    #[cfg(not(feature = "do_image_gen"))]
    {
        println!("Saving poem...");

        let filename = ai::save_to_file(
            title_trimmed,
            prompt,
            &generated_poem_prompt,
            &full_text,
        )
            .await
            .expect("Failed to save file");
    }

    Ok(())
}
