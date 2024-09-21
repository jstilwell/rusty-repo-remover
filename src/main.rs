use clap::{Parser, Subcommand};
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, USER_AGENT};
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    List,
    Delete,
}

#[derive(Deserialize)]
struct Config {
    token: String,
    owner: String,
    repos: Vec<String>,
}

#[derive(Deserialize, Serialize)]
struct Repository {
    name: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config: Config = toml::from_str(&fs::read_to_string("config/config.toml")?)?;
    let cli = Cli::parse();

    let client = reqwest::Client::new();
    let mut headers = HeaderMap::new();
    headers.insert(AUTHORIZATION, HeaderValue::from_str(&format!("token {}", config.token))?);
    headers.insert(USER_AGENT, HeaderValue::from_static("rusty-repo-remover"));

    match cli.command {
        Some(Commands::List) => {
            let mut page = 1;
            let mut all_repos = Vec::new();

            loop {
                let repos: Vec<Repository> = client
                    .get(&format!("https://api.github.com/user/repos?page={}&per_page=100", page))
                    .headers(headers.clone())
                    .send()
                    .await?
                    .json()
                    .await?;

                if repos.is_empty() {
                    break;
                }

                all_repos.extend(repos);
                page += 1;
            }

            println!("All repositories:");
            for repo in all_repos {
                println!("- {}", repo.name);
            }
        }
        Some(Commands::Delete) => {
            for repo in &config.repos {
                println!("Deleting repository: {}", repo);
                let response = client
                    .delete(&format!("https://api.github.com/repos/{}/{}", config.owner, repo))
                    .headers(headers.clone())
                    .send()
                    .await?;

                if response.status().is_success() {
                    println!("Successfully deleted {}", repo);
                } else {
                    println!("Failed to delete {}. Status: {}", repo, response.status());
                }
            }
        }
        None => {
            println!("Dry run (simulation):");
            for repo in &config.repos {
                println!("Would delete repository: {}", repo);
            }
        }
    }

    Ok(())
}