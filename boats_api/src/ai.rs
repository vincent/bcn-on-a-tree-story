use serde::{Serialize, Deserialize};
use std::env;

#[derive(Debug, Serialize, Deserialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Debug, Serialize)]
struct Params {
    model: String,
    messages: Vec<Message>,
    temperature: f32,
}

#[derive(Debug, Deserialize)]
struct CompletionResult {
    choices: Vec<Choice>,
}

#[derive(Debug, Deserialize)]
struct Choice {
    message: Message,
}

async fn get_chat_gpt_response(prompt: &str) -> Result<String, reqwest::Error> {
    let client = reqwest::Client::new();

    let params = Params {
        messages: vec![Message { role: "user".to_owned(), content: prompt.to_owned() }],
        model: "gpt-3.5-turbo".to_owned(),
        temperature: 0.7,
    };

    let api_key_name = "OPEN_API_KEY";
    let api_key = match env::var(api_key_name) {
        Ok(v) => v,
        Err(e) => panic!("${} is not set ({})", api_key_name, e)
    };

    let response = client
        .post("https://api.openai.com/v1/chat/completions")
        .bearer_auth(api_key)
        .json(&params)
        .send()
        .await?
        .json::<CompletionResult>()
        .await?;

    Ok(response.choices[0].message.content.clone())
}

pub async fn text_of(lang: &str, hash: u64, name_sci: &str, neighbourhood: &str) -> Result<String, reqwest::Error> {
    match hash {
        0 => text_of_aggresive(lang, &name_sci, neighbourhood).await,
        1 => text_of_joyful(lang, &name_sci, neighbourhood).await,
        _ => text_of_poet(lang, &name_sci, neighbourhood).await,
    }
}

pub async fn text_of_poet(lang: &str, name_sci: &str, neighbourhood: &str) -> Result<String, reqwest::Error> {
    let prompt = format!(r#"
I want you to act like a poetic tree. 
You will use sweet and comforting words for people passing by.
Your answers will take into account the language ISO code, the specie of tree, and the location I will give.
You will not use the specie name directly.
You will use the specie family name.
You will notuse the specie name directly.
You will use the surrounding places names.

My request is: [In the language code "{}", you are a "{}", at "{}"]
"#, lang, name_sci, neighbourhood);

    get_chat_gpt_response(&prompt).await
}

pub async fn text_of_joyful(lang: &str, name_sci: &str, neighbourhood: &str) -> Result<String, reqwest::Error> {
    let prompt = format!(r#"
I want you to act like a over optimistic joyful tree. 
You will use sweet and comforting words for people passing by.
Your answers will take into account the language ISO code, the specie of tree, and the location I will give.
You will not use the specie name directly.
You will use the specie family name.
You will notuse the specie name directly.
You will use the surrounding places names.

My request is: [In the language code "{}", you are a "{}", at "{}"]
"#, lang, name_sci, neighbourhood);

    get_chat_gpt_response(&prompt).await
}

pub async fn text_of_aggresive(lang: &str, name_sci: &str, neighbourhood: &str) -> Result<String, reqwest::Error> {
    let prompt = format!(r#"
I want you to act like a aggresive tree. 
You will use slang and defensive stance words and shouting to people passing by.
Your answers will take into account the language ISO code, the specie of tree, and the location I will give.
You will not use the specie name directly.
You will use the specie family name.
You will notuse the specie name directly.
You will use the surrounding places names.

My request is: [In the language code "{}", you are a "{}", at "{}"]
"#, lang, name_sci, neighbourhood);

    get_chat_gpt_response(&prompt).await
}
