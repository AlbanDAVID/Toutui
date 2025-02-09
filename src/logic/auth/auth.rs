use crate::login_app::AppLogin;
use crate::login_app::AppViewLogin;
use crate::app::AppView;
use ratatui::backend::CrosstermBackend;
use ratatui::widgets::{Block, Borders};
use ratatui::Terminal;
use std::io;
use tui_textarea::{Input, Key, TextArea};
use ratatui::{
    layout::Rect,
    style::{Color, Style},
};
use crate::api::server::auth::*;
use crossterm::event::{self, KeyEvent, KeyCode};  


impl AppLogin {
    pub async fn auth(&mut self) -> io::Result<()> {
        /// init input area
        let stdout = io::stdout();
        let stdout = stdout.lock();

        let backend = CrosstermBackend::new(stdout);
        let mut term = Terminal::new(backend)?;

        let mut textarea1 = TextArea::default();
        textarea1.set_block(
            Block::default()
            .borders(Borders::ALL)
            .title("Server address")
            .border_style(Style::default().fg(Color::LightBlue)),
        );

        let mut textarea2 = TextArea::default();
        textarea2.set_block(
            Block::default()
            .borders(Borders::ALL)
            .title("Username")
            .border_style(Style::default().fg(Color::LightBlue)),
        );

        let mut textarea3 = TextArea::default();
        textarea3.set_block(
            Block::default()
            .borders(Borders::ALL)
            .title("Password")
            .border_style(Style::default().fg(Color::LightBlue)),
        );
        textarea3.set_mask_char('\u{2022}');

        // display 
        let size = term.size()?;
        let search_area = Rect {
            x: (size.width - size.width / 2) / 2,
            y: (size.height - 3) / 2,
            width: size.width / 2,
            height: 3,
        };

        /// init variables
        let mut textareas = vec![textarea1, textarea2, textarea3];
        let mut current_index = 0;

        let mut collected_data : Vec<String> = Vec::new();




        loop {
            term.draw(|f| {
                f.render_widget(&textareas[current_index], search_area);
            })?;

            match crossterm::event::read()? {
                event::Event::Key(KeyEvent { code: KeyCode::Enter, .. }) => {
                    if current_index < textareas.len() - 1 {
                        // will just take textarea 1 and 2, 3 will take after break loop
                        collected_data.push(textareas[current_index].lines().join("\n"));
                        current_index += 1;
                    } else {
                        break; 
                    }
                }
                
                event::Event::Key(KeyEvent { code: KeyCode::Esc, .. }) => {
                    break; 
                }
                
                event::Event::Key(input) => {
                    if let Some(active_textarea) = textareas.get_mut(current_index) {
                        active_textarea.input(input); 
                    }
                }
                _ => {}
            }
        }

        // save the last input (from textearea3)
        collected_data.push(textareas[current_index].lines().join("\n"));

        // make disappear search_area (the input bar) after the break loop
        term.draw(|f| {
            let empty_block = Block::default();
            f.render_widget(empty_block, search_area); 
        })?;

        /// Fetch data from api and insert them in database


        // send result
        if let Some(active_textarea) = textareas.get(current_index) {
            let collected_data_clone = collected_data.clone();
            tokio::spawn(async move {
                println!("Wait...");
                match login(
                    collected_data_clone[1].as_str(),
                    collected_data_clone[2].as_str(),
                    collected_data_clone[0].as_str(),
                ).await {
                    Ok(response) => {
                        println!("Login successful");
                    }
                    Err(e) => {
                        eprintln!("Login failed: {}", e);
                    }
                }});


            Ok(())
        } else {
            Err(io::Error::new(io::ErrorKind::Other, "Invalid textarea"))
        }
    }
}
