use rustyline::error::ReadlineError;
use rustyline::{Editor};
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use reqwest::header::CONTENT_TYPE;
use reqwest::header::USER_AGENT;

// Deserialize the user's repositories, we only care about 'language'
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Repository {
    language: Option<String>, // Has to be 'Option' type to account for null
}

#[derive(Serialize, Deserialize, Debug)]
struct JSONResponse {
    json: HashMap<String, String>,
}

fn main() {
    // `()` can be used when no completer is required
    let mut rl = match Editor::<()>::new() {
        Ok(rl) => rl,
        Err(error) => panic!("Problem with readline: {:?}", error),
    };

    println!("Please input a github username below:");

    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                let user_name = line.as_str().trim();

                let _resp200 = match get_user_repositories_by_date(user_name) {
                    Ok(resp200) => resp200,
                    Err(error) => panic!("Problem with response: {:?}", error),
                };

                println!("Line: {}", user_name);
                break
            },
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break
            },
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break
            }
        }
    }
}

#[tokio::main]
async fn get_user_repositories_by_date(user_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    // https://api.github.com/users/ElliottLandsborough/repos?sort=updated
    let prefix: &str = "https://api.github.com/users/";
    let suffix: &str = "/repos?sort=updated";

    let url = format!("{}{}{}", prefix, user_name, suffix);

    let client = reqwest::Client::new();

    let resp200 = client.get(url)
        .header(CONTENT_TYPE, "application/json")
        .header(USER_AGENT, "rust-terminal-app/0.1.0")
        .send()
        .await?;

    let json = resp200.text().await?;

    let datas: Vec<Repository> = serde_json::from_str(&json)?;

    println!("{:#?}", datas);

    Ok(())
}