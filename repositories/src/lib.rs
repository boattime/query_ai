use dotenvy::dotenv;
use git2::{Error, Repository};
use serde::Deserialize;
use std::path::Path;
use std::{env, fs};
use toml;

#[derive(Deserialize, Debug, PartialEq)]
pub struct RepoConfig {
    pub repo_url: String,
    pub local_path: String,
}

pub trait GitRepo {
    fn clone_repo(&self, repo_url: &str, local_path: &str) -> Result<(), Error>;
}

pub struct GitClient;

impl GitRepo for GitClient {
    fn clone_repo(&self, repo_url: &str, local_path: &str) -> Result<(), Error> {
        Repository::clone(repo_url, Path::new(local_path))?;
        Ok(())
    }
}

pub fn clone_repository<T: GitRepo>(
    git_client: &T,
    repo_url: &str,
    local_path: &str,
) -> Result<(), Error> {
    git_client.clone_repo(repo_url, local_path)
}

pub fn load_config(file_path: Option<&str>) -> Result<RepoConfig, Box<dyn std::error::Error>> {
    dotenv().ok();

    if let Some(path) = file_path {
        if let Ok(contents) = fs::read_to_string(path) {
            let parsed: toml::Value = toml::from_str(&contents)?;
            let repo_url = parsed["repository"]["repo_url"]
                .as_str()
                .unwrap()
                .to_string();
            let local_path = parsed["repository"]["local_path"]
                .as_str()
                .unwrap()
                .to_string();
            return Ok(RepoConfig {
                repo_url,
                local_path,
            });
        }
    }

    let repo_url = env::var("REPO_URL").map_err(|_| "Missing REPO_URL")?;
    let local_path = env::var("LOCAL_PATH").map_err(|_| "Missing LOCAL_PATH")?;

    Ok(RepoConfig {
        repo_url,
        local_path,
    })
}

pub fn clone_repo() -> Result<(), git2::Error> {
    let config = load_config(Some("config.toml")).expect("Failed to load configuration");

    println!("Cloning {} to {}", config.repo_url, config.local_path);

    match Repository::clone(&config.repo_url, &config.local_path) {
        Ok(_) => {
            println!("Repository cloned successfully!");
            Ok(())
        }
        Err(e) => {
            eprintln!("Failed to clone repository: {}", e);
            Err(e)
        }
    }
}
