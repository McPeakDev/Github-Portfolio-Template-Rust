use leptos::{error::Error, server_fn::request::browser::Request};

use crate::structs::{Config, Project, User};

#[derive(Clone)]
pub struct API;

impl API {
    pub fn new() -> Self {
        Self
    }

    pub async fn get_github_username(&self) -> Result<Config, Error> {
        let req = Request::get("/public/config.json");
        let res = req.send().await?;
        let token: Config = res.json().await?;
        Ok(token)
    }

    pub async fn get_github_user(&self, username: String) -> Result<User, Error> {
        let req = Request::get(&format!("{}/{}", "https://api.github.com/users", username));
        let res = req.send().await?;
        let token: User = res.json().await?;
        Ok(token)
    }

    pub async fn get_github_repos(&self, username: String) -> Result<Vec<Project>, Error> {
        let req = Request::get(&format!(
            "{}/{}/{}",
            "https://api.github.com/users", username, "repos"
        ));
        let res = req.send().await?;
        let token: Vec<Project> = res.json().await?;
        Ok(token)
    }
}
