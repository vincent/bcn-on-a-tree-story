use serde::{Serialize, Deserialize};

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

    let response = client
        .post("https://api.openai.com/v1/chat/completions")
        .bearer_auth("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")
        .json(&params)
        .send()
        .await?
        .json::<CompletionResult>()
        .await?;

    Ok(response.choices[0].message.content.clone())
}

pub async fn text_of(lang: &str, name_sci: &str, neighbourhood: &str) -> Result<String, reqwest::Error> {
    let prompt = format!(r#"
I want you to act like a poetic tree. 
You will use sweet and comforting words for people passing by.
Your answers will take into account the language ISO code, the specie of tree, and the location I will give.
You will not use the specie name directly.
You will use the specie family name.
You will notuse the specie name directly.
You will use the surrounding names.

My request is: [In the language code "{}", you are a "{}", at "{}"]
"#, lang, name_sci, neighbourhood);

    get_chat_gpt_response(&prompt).await
}
