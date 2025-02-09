use crate::login_app::AppViewLogin;
use crate::login_app::AppLogin;
use crate::logic::auth;
use crate::api::server::auth::*;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{
        palette::tailwind::{BLUE, SLATE},
        Color, Modifier, Style, Stylize,
    },
    text::Line,
    widgets::{
        Block, Borders, HighlightSpacing, List, ListItem , ListState,  Paragraph, StatefulWidget,
        Widget,
    },
};

// Auth
use crate::app::AppView;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::io;
use tui_textarea::{Input, Key, TextArea};
use crate::api::server::auth::*;
use crossterm::event::{self, KeyEvent, KeyCode};






// const for color theme
const TODO_HEADER_STYLE: Style = Style::new().fg(SLATE.c100).bg(BLUE.c800);
const NORMAL_ROW_BG: Color = SLATE.c950;
const ALT_ROW_BG_COLOR: Color = SLATE.c900;
const SELECTED_STYLE: Style = Style::new().bg(SLATE.c800).add_modifier(Modifier::BOLD);

/// init widget for selected AppView 
impl Widget for &mut AppLogin {
  fn render(self, area: Rect, buf: &mut Buffer) {
        match self.view_state {
            AppViewLogin::Auth => self.render_auth(area, buf),
        }
    }
}


/// Rendering logic
    pub async fn auth() -> io::Result<()> {
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

impl AppLogin {
    /// AppView::Library rendering
    fn render_auth(&mut self, area: Rect, buf: &mut Buffer) {
let [header_area, main_area, footer_area] = Layout::vertical([
            Constraint::Length(2),
            Constraint::Fill(1),
            Constraint::Length(1),
        ]).areas(area);

        let [list_area, item_area] = Layout::vertical([Constraint::Fill(1), Constraint::Fill(1)]).areas(main_area);

        let render_list_title = "Continue Listening";
        let text_render_footer = "Use ↓↑ to move, → to play, s to search, q to quit.";

        AppLogin::render_header(header_area, buf);
        AppLogin::render_footer(footer_area, buf, text_render_footer);
       // self.render_list(list_area, buf, render_list_title, &self.titles_cnt_list.clone(), &mut self.list_state_cnt_list.clone());

    }

        //self.render_selected_item(item_area, buf, &self.titles_library.clone(), self.auth_names_library.clone());
    


    /// General functions for rendering 

    fn render_header(area: Rect, buf: &mut Buffer) {
        Paragraph::new("< Home >")
            .bold()
            .centered()
            .render(area, buf);
    }

    fn render_footer(area: Rect, buf: &mut Buffer, text_render_footer: &str) {
        Paragraph::new(text_render_footer)
            .centered()
            .render(area, buf);

        Paragraph::new("toutui v0.1.0")
            .right_aligned()
            .render(area, buf);
    }

    fn render_list(&mut self, area: Rect, buf: &mut Buffer, render_list_title: &str, render_list_items: &[String], list_state: &mut ListState) {
        let block = Block::new()
            .title(Line::raw(format!("{}", render_list_title)).centered())
            .borders(Borders::TOP)
            .border_style(TODO_HEADER_STYLE)
            .bg(NORMAL_ROW_BG);

        let items: Vec<ListItem> = render_list_items
            .iter()
            .enumerate()
            .map(|(i, title)| {
                let color = Self::alternate_colors(i);
                ListItem::new(title.clone()).bg(color)
            })
        .collect();


        let list = List::new(items)
            .block(block)
            .highlight_style(SELECTED_STYLE)
            .highlight_symbol(">")
            .highlight_spacing(HighlightSpacing::Always);

        StatefulWidget::render(list, area, buf, list_state);
    }

//    fn render_selected_item(&self, area: Rect, buf: &mut Buffer, list_state: &ListState, author_name: Vec<&String>) {
//        if let Some(selected) = list_state.selected() {
//            let content = author_name[selected];
//            Paragraph::new(content.clone())
//                .wrap(Wrap { trim: true })
//                .render(area, buf);
//        }
//    }

    const fn alternate_colors(i: usize) -> Color {
        if i % 2 == 0 {
            NORMAL_ROW_BG
        } else {
            ALT_ROW_BG_COLOR
        }
    }
}

