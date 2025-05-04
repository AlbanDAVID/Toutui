use serde_json::Value;
use reqwest::header::USER_AGENT;
use reqwest::Client;

const LOCAL_VERSION: &str = env!("CARGO_PKG_VERSION");

pub async fn check_update() -> Option<String> {
    match get_latest_release_gh().await {
        Ok(latest_version_gh) => {
            if latest_version_gh != LOCAL_VERSION {
                log::warn!(
                    "You are not up-to-date. Current: {} / Available: {}",
                    LOCAL_VERSION,
                    latest_version_gh
                );
                Some(format!(
                    "🔄 Update to {} available (go to settings > update)",
                    latest_version_gh
                ))
            } else {
                None
            }
        }
        Err(e) => {
            log::error!("{}", e);
            None
        }
    }
}

pub async fn get_latest_release_gh() -> Result<String, Box<dyn std::error::Error>> {
    let client = Client::new();
    let response = client
        .get("https://api.github.com/repos/AlbanDAVID/Toutui/releases/latest")
        .header(USER_AGENT, "Toutui-Updater")
        .send()
        .await?;
    let text = response.text().await?;

    let v: Value = serde_json::from_str(&text)?;

    if let Some(tag_name) = v["tag_name"].as_str() {
    Ok(tag_name.trim_start_matches('v').to_string())
    } else {
        Err("[get_latest_release_gh] couldn't find last release".into())
    }
}

