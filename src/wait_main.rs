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

    // Initial database creation
    let mut database = Database::new().await?;
    ratatui::restore();

    // Wait for the database to be ready, waiting for the user to enter their credentials
    while database.default_usr.is_empty() {
        
    let app_login = AppLogin::new().await?;
    let terminal = ratatui::init();
    let app_result = app_login.run(terminal);
    app_result;
        // Wait for 1 second before checking again
//        tokio::time::sleep(Duration::from_secs(1)).await;
        
        // Reload or update the database
        database = Database::new().await?;
    }

    // Once the database is ready, initialize the app
    color_eyre::install()?;
    let mut app = App::new().await?;
    let mut terminal = ratatui::init();

    // Running the app in a loop
    loop {
        // If `app` variable is reinitialized below (`app = App::new().await?`), it will be taken into account and data will be refreshed
        // Otherwise, the current `app` variable will still be used.
        let result = app.run(&mut terminal);

        if let Err(e) = result {
            eprintln!("Error running the app: {:?}", e);
        }

        // Checking if any key is pressed (waiting for events with a 200ms delay here)
        if crossterm::event::poll(Duration::from_millis(200))? {
            if let event::Event::Key(key) = crossterm::event::read()? {
                match key.code {
                    // If the 'R' key is pressed, refresh the app
                    KeyCode::Char('R') => {
                        println!("Refreshing app...");
                        app = App::new().await?; // Reinitialize to refresh
                    }
                    // If 'Q' or 'Esc' is pressed, exit the app
                    KeyCode::Char('Q') | KeyCode::Esc => {
                        println!("Exiting app...");
                        break;
                    }
                    _ => {}
                }
            }
        }

        // Short pause between event checks
        tokio::time::sleep(Duration::from_millis(50)).await;
    }

    // Restore the terminal state before exiting the application
    ratatui::restore();
    Ok(())
}

