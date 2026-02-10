use crate::errors::APIError;
use reqwest::Client;
use serde::Deserialize;
use crate::config::Config;

#[derive(Deserialize)]
struct GoogleTokenResponse {
    access_token: String,
}

#[derive(Deserialize)]
pub struct GoogleUserInfo {
    pub id: String,
    pub email: String,
}

#[derive(Deserialize)]
struct GithubTokenResponse {
    access_token: String,
}

#[derive(Deserialize)]
pub struct GithubUserInfo {
    pub id: i64,
    pub email: Option<String>,
}

pub async fn get_google_user_info(code: &str, config: &Config) -> Result<GoogleUserInfo, APIError> {
    let client = Client::new();

    let token_response = client
        .post("https://oauth2.googleapis.com/token")
        .form(&[
            ("code", code),
            ("client_id", &config.google_client_id),
            ("client_secret", &config.google_client_secret),
            ("redirect_uri", &config.google_redirect_uri),
            ("grant_type", &"authorization_code".to_string()),
        ])
        .send()
        .await
        ?
        .json::<GoogleTokenResponse>()
        .await
        ?;

    let user_info_response = client
        .get("https://www.googleapis.com/oauth2/v2/userinfo")
        .bearer_auth(token_response.access_token)
        .send()
        .await
        ?
        .json::<GoogleUserInfo>()
        .await
        ?;

    Ok(user_info_response)
}

pub async fn get_github_user_info(code: &str, config: &Config) -> Result<GithubUserInfo, APIError> {
    let client = Client::new();

    let token_response = client
        .post("https://github.com/login/oauth/access_token")
        .header("Accept", "application/json")
        .form(&[
            ("code", code),
            ("client_id", &config.github_client_id),
            ("client_secret", &config.github_client_secret),
            ("redirect_uri", &config.github_redirect_uri),
        ])
        .send()
        .await
        ?
        .json::<GithubTokenResponse>()
        .await
        ?;

    let user_info_response = client
        .get("https://api.github.com/user")
        .bearer_auth(token_response.access_token)
        .header("User-Agent", "skoola-backend")
        .send()
        .await
        ?
        .json::<GithubUserInfo>()
        .await
        ?;

    Ok(user_info_response)
}
