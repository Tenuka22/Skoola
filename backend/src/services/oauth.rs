use crate::errors::iam::IAMError;
use reqwest::Client;
use serde::Deserialize;
use tracing::info;
use crate::config::Config;
use crate::utils::logging::log_iam_error;

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

pub async fn get_google_user_info(code: &str, config: &Config) -> Result<GoogleUserInfo, IAMError> {
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
        .map_err(|e| {
            let err_msg = format!("Failed to exchange Google code for token: {}", e);
            log_iam_error("google_oauth_token_exchange", &e);
            IAMError::GoogleOAuthError(err_msg)
        })?
        .json::<GoogleTokenResponse>()
        .await
        .map_err(|e| {
            let err_msg = format!("Failed to parse Google token response: {}", e);
            log_iam_error("google_oauth_token_parse", &e);
            IAMError::GoogleOAuthError(err_msg)
        })?;

    let user_info_response = client
        .get("https://www.googleapis.com/oauth2/v2/userinfo")
        .bearer_auth(token_response.access_token)
        .send()
        .await
        .map_err(|e| {
            let err_msg = format!("Failed to fetch Google user info: {}", e);
            log_iam_error("google_oauth_user_info_fetch", &e);
            IAMError::GoogleOAuthError(err_msg)
        })?
        .json::<GoogleUserInfo>()
        .await
        .map_err(|e| {
            let err_msg = format!("Failed to parse Google user info: {}", e);
            log_iam_error("google_oauth_user_info_parse", &e);
            IAMError::GoogleOAuthError(err_msg)
        })?;

    info!(event = "google_oauth_success", email = %user_info_response.email, "Google OAuth successful");
    Ok(user_info_response)
}

pub async fn get_github_user_info(code: &str, config: &Config) -> Result<GithubUserInfo, IAMError> {
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
        .map_err(|e| {
            let err_msg = format!("Failed to exchange GitHub code for token: {}", e);
            log_iam_error("github_oauth_token_exchange", &e);
            IAMError::GithubOAuthError(err_msg)
        })?
        .json::<GithubTokenResponse>()
        .await
        .map_err(|e| {
            let err_msg = format!("Failed to parse GitHub token response: {}", e);
            log_iam_error("github_oauth_token_parse", &e);
            IAMError::GithubOAuthError(err_msg)
        })?;

    let user_info_response = client
        .get("https://api.github.com/user")
        .bearer_auth(token_response.access_token)
        .header("User-Agent", "skoola-backend")
        .send()
        .await
        .map_err(|e| {
            let err_msg = format!("Failed to fetch GitHub user info: {}", e);
            log_iam_error("github_oauth_user_info_fetch", &e);
            IAMError::GithubOAuthError(err_msg)
        })?
        .json::<GithubUserInfo>()
        .await
        .map_err(|e| {
            let err_msg = format!("Failed to parse GitHub user info: {}", e);
            log_iam_error("github_oauth_user_info_parse", &e);
            IAMError::GithubOAuthError(err_msg)
        })?;

    info!(event = "github_oauth_success", user_id = %user_info_response.id, "GitHub OAuth successful");
    Ok(user_info_response)
}
