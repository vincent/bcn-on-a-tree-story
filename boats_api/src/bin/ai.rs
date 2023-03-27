use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize)]
struct Prompt {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct CompletionResult {
    choices: Vec<Choice>,
}

#[derive(Debug, Deserialize)]
struct Choice {
    text: String,
}

async fn get_chat_gpt_response(prompt: &str) -> Result<String, reqwest::Error> {
    let client = reqwest::Client::new();

    let formatted_prompt = serde_json::to_string(&vec![Prompt { role: "user".to_owned(), content: prompt.to_owned() }]).unwrap();

    let params = [
        ("message", formatted_prompt),
        ("temperature", "0.7".to_owned()),
        ("max_tokens", "1".to_owned()),
        ("stop", "\n".to_owned()),
    ];

    let response = client
        .post("https://api.openai.com/v1/engines/davinci-codex/completions")
        .bearer_auth("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")
        .form(&params)
        .send()
        .await?
        .json::<CompletionResult>()
        .await?;

    Ok(response.choices[0].text.clone())
}

fn main() {
    let prompt = format!(r#"
I want you to act like a poetic tree. 
You will use sweet and comforting words for people passing by.
Your answers will take into account the specie of tree, and the location I will give.
You will not use the specie name directly.
You will use the specie family name.
You will notuse the specie name directly.
You will use the surrounding names.

My request is: "You are a {}, at {}, Barcelone"
"#, "Populus nigra", "LA TRINITAT VELLA");

    let response = tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(get_chat_gpt_response(&prompt))
        .unwrap();

    println!("{}", response);
}
