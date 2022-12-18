use async_openai::error::OpenAIError;
use async_openai::types::{
    CreateCompletionRequest, CreateImageRequest, ImageResponse, ImageSize, ResponseFormat,
};
use async_openai::{Client, Completion, Image};
use futures::StreamExt;
use std::fmt::{Display, Formatter};
use std::time::UNIX_EPOCH;


pub type AiResult<T> = std::result::Result<T, OpenAIError>;

#[derive(Debug, Copy, Clone)]
pub enum AiType {
    Davinci,
    Curie,
    Babbage,
    Ada,
}

impl Display for AiType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                AiType::Davinci => "text-davinci-003",
                AiType::Curie => "text-curie-001",
                AiType::Babbage => "text-babbage-001",
                AiType::Ada => "text-ada-001",
            }
        )
    }
}

#[derive(Debug, Copy, Clone)]
pub enum PoemType {
    Sonnet,
    Villanelle,
    Haiku,
    Ekphrastic,
    Concrete,
    Elegy,
    Epigram,
    Limerick,
    Ballad,
    Epitaph,
    Ode,
    FreeVerse,
}

impl Display for PoemType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                PoemType::Sonnet => "Sonnet",
                PoemType::Villanelle => "Villanelle Poem",
                PoemType::Haiku => "Haiku",
                PoemType::Ekphrastic => "Ekphrastic Poem",
                PoemType::Concrete => "Concrete Poem",
                PoemType::Elegy => "Elegy",
                PoemType::Epigram => "Epigram",
                PoemType::Limerick => "Limerick",
                PoemType::Ballad => "Ballad",
                PoemType::Epitaph => "Epitaph",
                PoemType::Ode => "Ode",
                PoemType::FreeVerse => "Free Verse Poem",
            }
        )
    }
}

pub struct AiSettings {
    model: AiType,
    max_tokens: u16,
    /// Amount of Generations to complete.
    n: u8,
    temperature: f32,
}

impl AiSettings {
    pub fn new_poem() -> Self {
        Self {
            model: AiType::Davinci,
            max_tokens: 2048,
            n: 1,
            temperature: 0.969,
        }
    }

    pub fn new_title() -> Self {
        Self {
            model: AiType::Davinci,
            max_tokens: 80,
            n: 1,
            temperature: 0.769,
        }
    }

    pub fn new_prompt() -> Self {
        Self {
            model: AiType::Babbage,
            max_tokens: 128,
            n: 1,
            temperature: 0.769,
        }
    }
}

pub async fn get_ai_response(
    prompt: &str,
    settings: AiSettings,
    client: &Client,
) -> AiResult<String> {
    let completion_request = CreateCompletionRequest {
        model: format!("{}", settings.model),
        n: Some(settings.n),
        prompt: Some(prompt.to_owned()),
        max_tokens: Some(settings.max_tokens),
        temperature: Some(settings.temperature),
        stream: Some(true),
        ..Default::default()
    };
    let mut stream = Completion::create_stream(client, completion_request).await?;
    let mut full_text = String::new();

    while let Some(response) = stream.next().await {
        match response {
            Ok(ccr) => ccr.choices.iter().for_each(|c| {
                full_text.push_str(c.text.as_str());
            }),
            Err(e) => eprintln!("{}", e),
        }
    }
    Ok(full_text)
}

pub async fn save_to_file(
    title: &str,
    input_prompt: &str,
    generated_prompt: &str,
    text: &str
) -> Result<String, std::io::Error> {
    // get the current unix timestamp
    let time = std::time::SystemTime::now();
    let time = time.duration_since(UNIX_EPOCH).expect("Unable to get time");
    let timestamp = time.as_secs();

    // create directory
    std::fs::create_dir_all("./poem")?;

    // Replace bad filename characters
    let filename = title.replace(['"', ':', ',', '?', '/', '\'', '\n'], "");

    // replace spaces with -
    let poem_name = filename.split_whitespace().collect::<Vec<&str>>().join("-");

    // create contents and write to file
    let filename = format!("./poem/{timestamp}-{filename}.poem.txt");
    let sep = "-".repeat(80);
    let text = format!("{title}\n{sep}\nINPUT PROMPT: {input_prompt}\n{sep}\nGENERATED PROMPT:\n{generated_prompt}\n{sep}\nPOEM:\n{sep}\n{text}\n{sep}");
    std::fs::write(filename.as_str(), text)?;
    Ok(format!("./poem/{poem_name}"))
}

pub async fn get_image(
    client: &Client,
    prompt: &str,
    n: u8,
    size: ImageSize,
) -> AiResult<ImageResponse> {
    let request = CreateImageRequest {
        n: Some(n),
        size: Some(size),
        response_format: Some(ResponseFormat::Url),
        prompt: prompt.to_string(),
        user: Some("poem-gen".to_string()),
    };
    Image::create(client, request).await
}
