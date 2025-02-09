mod app;
mod login_app;
mod config;
mod api;
mod ui;
mod player;
mod logic;
mod db;

use login_app::AppLogin;
use app::App;
use crate::logic::auth::auth;
use crate::db::database_struct::Database;
use color_eyre::Result;
use std::time::Duration;
use crossterm::event::{self, Event, KeyCode};
#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    let app = AppLogin::new().await?;
    let terminal = ratatui::init();
    let app_result = app.run(terminal);
    ratatui::restore();
    app_result

}
