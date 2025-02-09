//use crate::api::get_test::get_test;
use crate::api::utils::collect_personalized_view::*;
use crate::api::utils::collect_personalized_view_pod::*;
use crate::api::utils::collect_get_all_books::*;
use crate::api::utils::collect_get_pod_ep::*;
use crate::api::utils::collect_get_all_libraries::*;
use crate::api::libraries::get_library_perso_view::*;
use crate::api::libraries::get_library_perso_view_pod::*;
use crate::api::libraries::get_all_books::*;
use crate::api::libraries::get_all_libraries::*;
use crate::api::library_items::get_pod_ep::*;
use crate::api::server::auth::*;
use crate::logic::handle_input::handle_l_book::*;
use crate::logic::handle_input::handle_l_pod::*;
use crate::logic::handle_input::handle_l_pod_home::*;
use crate::main;
use crate::config::load_config;
use crate::db::crud::*;
use crate::db::database_struct::Database;
use color_eyre::Result;
use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    widgets::{ListState},
    DefaultTerminal,
};
use serde::{Serialize, Deserialize};
use rusqlite::Connection;
use std::thread;
use std::time::Duration;
use crate::app::AppView;

pub enum AppViewLogin {
    Auth,
}

pub struct AppLogin {
   pub view_state: AppViewLogin,
   pub should_exit: bool,

}

/// Init app
impl AppLogin {
    pub async fn new() -> Result<Self> {

        let view_state = AppViewLogin::Auth;

        Ok(Self {
            view_state,
            should_exit: false,
        })
    }


   /// handle events
 pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        while !self.should_exit {
            terminal.draw(|frame| frame.render_widget(&mut self, frame.area()))?;
            if let Event::Key(key) = event::read()? {
                self.handle_key(key);
            }
        }
        Ok(())
    }
   /// handle key
   pub fn handle_key(&mut self, key: KeyEvent) {
       if key.kind != KeyEventKind::Press {
           return;
       }

       match key.code {
           KeyCode::Char('Q') => {
               self.should_exit = true;
           }
           _ => {}
       }
   }

    /// Toggle between Home and Library views
    fn toggle_view(&mut self) {
        self.view_state = match self.view_state {
            AppViewLogin::Auth => AppViewLogin::Auth

        };
    }

 }
