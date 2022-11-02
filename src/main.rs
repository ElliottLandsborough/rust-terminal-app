use rustyline::error::ReadlineError;
use rustyline::{Editor};
use std::collections::HashMap;
use std::iter::FromIterator;
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

    loop {
        println!("Please input a github username below:");
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                let user_name = line.as_str().trim();

                let repositories = match get_user_repositories_by_date(user_name) {
                    Ok(resp200) => resp200,
                    Err(error) => panic!("Problem with response: {:?}", error),
                };

                // Did we get any repositories back?
                if repositories.len() > 0 {
                    output_favourite_languages(repositories);
                    break
                }
                
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
async fn get_user_repositories_by_date(user_name: &str) -> Result<Vec<Repository>, Box<dyn std::error::Error>> {
    // https://api.github.com/users/{username}/repos?sort=updated
    let prefix: &str = "https://api.github.com/users/";
    let suffix: &str = "/repos?sort=updated";
    let url = format!("{}{}{}", prefix, user_name, suffix);

    let client = reqwest::Client::new();

    let resp200 = client.get(url)
        .header(CONTENT_TYPE, "application/json")
        .header(USER_AGENT, "rust-terminal-app/0.1.0")
        .send()
        .await?;

    if resp200.status() != 200 {
        println!("{}", resp200.status());

        return Ok(Vec::new());
    }

    let json = resp200.text().await?;

    let datas: Vec<Repository> = serde_json::from_str(&json)?;

    Ok(datas)
}

// Format the repositories Vec into something readable by a human
fn output_favourite_languages(repositories: Vec<Repository>) {
    // New hashmap, string is language name, int is number of repos
    let mut map: HashMap<String, i64> = HashMap::new();

    // Iterate through all the repos
    for repository in repositories.iter() {
        // We either had a language or null, set null to 'Unknown'
        let key = repository.language.as_deref().unwrap_or("Unknown");

        // Add key if doesn't exist otherwise +1 to the count
        map
            .entry(String::from(key))
            .and_modify(|count| *count += 1)
            .or_insert(0);
    }

    // Remove repositories of unknown language
    map.remove("Unknown");

    // No hits?
    if map.len() == 0 {
        println!("Github was not able to determine the language of this user's repositories.");

        return;
    }

    // Sort map by counts
    let mut sorted_map = Vec::from_iter(map);
    sorted_map.sort_by(|&(_, a), &(_, b)| b.cmp(&a));

    let mut position = 1;

    // Output something readable
    for (key, _) in sorted_map.iter() {
        println!("{}. {}", position, key);
        position = position + 1;
    }
}
