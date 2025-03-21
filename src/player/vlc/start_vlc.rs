use std::process::Command;
use std::process::Output;

pub async fn start_vlc(
    current_time: &String, 
    port: &str, 
    address: String,
    content_url: &String, 
    token: Option<&String>, 
    title: String, 
    subtitle: String, 
    author: String, 
    server_address: String, 
    program: String, 
) -> Output {

    let output: Output = Command::new(format!("{}", program))
        .arg(format!("--start-time={}", current_time))
        .arg("--extraintf")
        .arg("rc")
        .arg("--rc-host")
        .arg(format!("{}:{}",address, port))
        .arg(format!("{}{}?token={}", server_address, content_url, token.unwrap()))
        .arg("--meta-description")
        .arg(author)
        .arg("--meta-title")
        .arg(subtitle)
        .arg("--meta-artist")
        .arg(title)
        .output()
        .expect("Failed to execute program");

    output
}

