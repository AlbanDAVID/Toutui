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

pub enum AppView {
    Home,
    Library,
    SearchBook,
    PodcastEpisode,
    Libraries,
    Auth,
}

pub struct App {
   pub view_state: AppView,
   pub database: Database,
   pub id_selected_lib: String,
   pub token: Option<String>,
   pub should_exit: bool,
   pub list_state_cnt_list: ListState,
   pub list_state_library: ListState,
   pub list_state_search_results: ListState,
   pub list_state_pod_ep: ListState,
   pub list_state_libraries: ListState,
   pub titles_cnt_list: Vec<String>,
   pub auth_names_cnt_list: Vec<String>,
   pub ids_cnt_list: Vec<String>,
   pub titles_library: Vec<String>,
   pub ids_library: Vec<String>,
   pub auth_names_library: Vec<String>,
   pub ids_search_book: Vec<String>,
   pub search_query: String,
   pub search_mode: bool,
   pub is_podcast: bool,
   pub all_titles_pod_ep: Vec<Vec<String>>,
   pub all_ids_pod_ep: Vec<Vec<String>>,
   pub titles_pod_ep: Vec<String>,
   pub ids_pod_ep: Vec<String>,
   pub ids_ep_cnt_list: Vec<String>,
   pub all_titles_pod_ep_search: Vec<Vec<String>>,
   pub titles_pod_ep_search: Vec<String>,
   pub is_from_search_pod: bool,
   pub ids_library_pod_search: Vec<String>,
   pub all_ids_pod_ep_search: Vec<Vec<String>>,
   pub library_names: Vec<String>,
   pub media_types: Vec<String>,
   pub library_ids: Vec<String>,

}

/// Init app
impl App {
    pub async fn new() -> Result<Self> {

        // init config
        let config = load_config()?;

        let mut view_state = AppView::Home; // By default, Home will be the first AppView launched when the app start
        // Default view_state
        // init database from Database struct
        let mut database = Database::new().await?;
                                        
        // init token 
        let mut token: String = String::new();
        if let Some(var_token) = database.default_usr.get(3) {
            token = var_token.clone();

        }

        // init id_selected_lib
        let mut id_selected_lib: String = String::new();
        if let Some(var_id_selected_lib) = database.default_usr.get(6) {
            id_selected_lib = var_id_selected_lib.clone();

        }

    
        // init for `Shelf`
        let is_podcast = false;


        // init for `Home` (continue listening)
        let mut titles_cnt_list: Vec<String> = Vec::new();
        let mut auth_names_cnt_list: Vec<String> = Vec::new();
        let mut ids_cnt_list: Vec<String> = Vec::new();
        let mut ids_ep_cnt_list: Vec<String> = Vec::new();


        if is_podcast {
         // init for  `Home` (continue listening) for podcasts
         let continue_listening_pod = get_continue_listening_pod(&token).await?;
         ids_cnt_list = collect_ids_pod_cnt_list(&continue_listening_pod).await; // id of a podcast
         titles_cnt_list = collect_titles_cnt_list_pod(&continue_listening_pod).await;
         ids_ep_cnt_list = collect_ids_ep_pod_cnt_list(&continue_listening_pod).await; // id of a podcast episode
         }
         else {
         // init for  `Home` (continue listening) for books
         let continue_listening = get_continue_listening(&token).await?;
         titles_cnt_list = collect_titles_cnt_list(&continue_listening).await;
         auth_names_cnt_list = collect_auth_names_cnt_list(&continue_listening).await;
         ids_cnt_list = collect_ids_cnt_list(&continue_listening).await;
         }

         //init for `Library ` (all books  or podcasts of a Library (shelf))
         let all_books = get_all_books(&token, &id_selected_lib).await?;
         let titles_library = collect_titles_library(&all_books).await;
         let ids_library = collect_ids_library(&all_books).await;
         let auth_names_library = collect_auth_names_library(&all_books).await;


         // init for `SearchBook`
         let ids_search_book: Vec<String> = Vec::new();
         let search_mode = false;
         let search_query = "  ".to_string();
         let all_titles_pod_ep_search: Vec<Vec<String>> = Vec::new();
         let titles_pod_ep_search: Vec<String> = Vec::new();
         let is_from_search_pod = false;
         let ids_library_pod_search: Vec<String> = Vec::new();
         let mut all_ids_pod_ep_search: Vec<Vec<String>> = Vec::new();


         //init for `PodcastEpisode`
         let mut all_titles_pod_ep: Vec<Vec<String>> = Vec::new(); // fetch titles for all podcast episodes. Ex: {titles_pod1_ep1, title_pod1_ep2}, {titles_pod2_ep1, title_pod2_ep2} 
         let mut all_ids_pod_ep: Vec<Vec<String>> = Vec::new();
         let titles_pod_ep: Vec<String> = Vec::new(); // fetch episode titles for a podcast. {titles_pod1_ep1}, {title_pod1_ep2} 
         let ids_pod_ep: Vec<String> = Vec::new();

         for i in 0..ids_library.len() 
         {let podcast_episode = get_pod_ep(&token, ids_library[i].as_str()).await?;
         let title = collect_titles_pod_ep(&podcast_episode).await;
         all_titles_pod_ep.push(title);
         let id = collect_ids_pod_ep(&podcast_episode).await;
         all_ids_pod_ep.push(id);
         }

         // init for `Libraries` (get all Libraries (shelf), can be a podcast or book type)
         let all_libraries = get_all_libraries(&token).await?;
         let library_names = collect_library_names(&all_libraries).await;
         let media_types = collect_media_types(&all_libraries).await;
         let library_ids = collect_library_ids(&all_libraries).await;
 




         // Init ListeState for `Home` list (continue listening)
         let mut list_state_cnt_list = ListState::default(); // init the ListState ratatui's widget
         list_state_cnt_list.select(Some(0)); // select the first item of the list when app is launch

         // Init ListeState for `Library` list
         let mut list_state_library = ListState::default(); 
         list_state_library.select(Some(0)); 
                                             
         // Init ListeState for `SearchBook` list
         let mut list_state_search_results = ListState::default(); 
         list_state_search_results.select(Some(0)); 

         // Init ListState for `PodacastEpisode` list
         let mut list_state_pod_ep = ListState::default();
         list_state_pod_ep.select(Some(0));

         // Init ListState for `Libraries` list
         let mut list_state_libraries = ListState::default();
         list_state_libraries.select(Some(0));

        Ok(Self {
            database,
            id_selected_lib,
            token: Some(token),
            should_exit: false,
            list_state_cnt_list,
            list_state_library,
            list_state_search_results,
            list_state_pod_ep,
            list_state_libraries,
            titles_cnt_list,
            auth_names_cnt_list,
            ids_cnt_list,
            view_state,
            titles_library,
            ids_library,
            auth_names_library,
            ids_search_book,
            search_mode,
            search_query,
            is_podcast,
            all_titles_pod_ep,
            all_ids_pod_ep,
            titles_pod_ep,
            ids_pod_ep,
            ids_ep_cnt_list, 
            all_titles_pod_ep_search,
            titles_pod_ep_search,
            is_from_search_pod,
            ids_library_pod_search,
            all_ids_pod_ep_search,
            library_names,
            library_ids,
            media_types,
        })
    }


   /// handle events
   pub fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        while !self.should_exit {
            terminal.draw(|frame| frame.render_widget(&mut *self, frame.area()))?;
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
        KeyCode::Char('s') => {
            let _ = self.search_active();
        }
        KeyCode::Char('c') => {
            self.view_state = AppView::Libraries;
        }
        KeyCode::Tab => {
            if self.is_from_search_pod {
                self.is_from_search_pod = false;
            };
            self.toggle_view()
        }
        KeyCode::Char('Q') => self.should_exit = true, // need to exit run function once, and after
                                                       // should quit once again the run from loop main function :
                                                       // (`let result = app.run(&mut terminal);`)
        KeyCode::Char('R') => self.should_exit = true, // same as above, need to quit once before
                                                       // be able to execute `R` from main function 
        KeyCode::Char('j') | KeyCode::Down => self.select_next(),
        KeyCode::Char('k') | KeyCode::Up => self.select_previous(),
        KeyCode::Char('g') | KeyCode::Home => self.select_first(),
        KeyCode::Char('G') | KeyCode::End => self.select_last(),
        KeyCode::Char('l') | KeyCode::Right | KeyCode::Enter => {
            // Clone needed because variables will be used in a spawn
            let token = self.token.clone();
            let port = "1234".to_string();

            // Init for `Continue Listening` (AppView::Home)
            let ids_cnt_list = self.ids_cnt_list.clone();
            let selected_cnt_list = self.list_state_cnt_list.selected();

            // Init for `Library`
            let ids_library = self.ids_library.clone();
            let selected_library = self.list_state_library.selected();

            // Init for `Search Book`
            let ids_search_book = self.ids_search_book.clone();
            let selected_search_book = self.list_state_search_results.selected();

            // Init for `PodcastEpisode`
            let ids_pod_ep = self.ids_pod_ep.clone();
            let selected_pod_ep = self.list_state_pod_ep.selected();
            let ids_ep_cnt_list = self.ids_ep_cnt_list.clone();


            // Now, spawn the async task based on the current view state
            match self.view_state {
                AppView::Auth => {

                }
                AppView::Home => {
                    if self.is_podcast {
                        tokio::spawn(async move {
                            handle_l_pod_home(token.as_ref(), &ids_cnt_list, selected_cnt_list, port, ids_ep_cnt_list).await;
                        });
                    } else {
                    tokio::spawn(async move {
                        handle_l_book(token.as_ref(), ids_cnt_list, selected_cnt_list, port).await;
                    });
                    }}
                AppView::Libraries => {
                    if let Ok(conn) = Connection::open("db/db.sqlite3") {
                        if let Err(e) = update_id_selected_lib(&conn, "64c39f84-9c58-4045-a89c-e17a6d99076", "luc") {
                            println!("Error updating selected library: {}", e);
                        } else {
                            println!("Selected library updated successfully!");
                        }
                    } else {
                        println!("Error connecting to the database.");
                    }
                }
                AppView::Library => {
                    if self.is_podcast {
                    if let Some(index) = selected_library {
                        self.titles_pod_ep = self.all_titles_pod_ep[index].clone();
                        self.list_state_pod_ep.select(Some(0));
                        self.view_state = AppView::PodcastEpisode;
                    }} else {
                        tokio::spawn(async move {
                            handle_l_book(token.as_ref(), ids_library, selected_library, port).await;
                        });
                    }
                }
                AppView::SearchBook => {
                    if self.is_podcast {
                        self.is_from_search_pod = true;
                        if let Some(index) = selected_search_book {
                            self.titles_pod_ep_search = self.all_titles_pod_ep_search[index].clone();
                            self.list_state_pod_ep.select(Some(0));
                            self.view_state = AppView::PodcastEpisode;
                        }} else {   
                            tokio::spawn(async move {
                                handle_l_book(token.as_ref(), ids_search_book, selected_search_book, port).await;
                            });

                        }
                }
                AppView::PodcastEpisode => {
                    if self.is_from_search_pod {
                    // we need the index of selected_search_book to feet after with
                    // ids_library_pod_search
                    if let Some(index) = selected_search_book {
                        // ids_library_pod_search because we need the pod id and he is given by
                        // this variable
                        if let Some(id_pod) = self.ids_library_pod_search.get(index) {
                            println!("{:?}", id_pod);
                            let all_ids_pod_ep_search_clone = self.all_ids_pod_ep_search.clone();
                            println!("{:?}", all_ids_pod_ep_search_clone[index]);
                            let id_pod_clone = id_pod.clone();
                            tokio::spawn(async move {
                                handle_l_pod(token.as_ref(), &all_ids_pod_ep_search_clone[index], selected_pod_ep, port, id_pod_clone.as_str()).await;
                            });
                        }
                    }
                    } else {
                        // selected_livrary ids_library because we need the pod id and he is given by
                        // these variables
                        // we also need the index of selected library to feet after with
                        // ids_library
                    if let Some(index) = selected_library {
                        if let Some(id_pod) = ids_library.get(index) {
                            let all_ids_pod_ep_clone = self.all_ids_pod_ep.clone();
                            let id_pod_clone = id_pod.clone();
                            tokio::spawn(async move {
                                handle_l_pod(token.as_ref(), &all_ids_pod_ep_clone[index], selected_pod_ep, port, id_pod_clone.as_str()).await;
                            });
                        }
                    }

                    }
                }
            }
        }
        _ => {}
    }
}

    /// Toggle between Home and Library views
    fn toggle_view(&mut self) {
        self.view_state = match self.view_state {
            AppView::Home => AppView::Library,
            AppView::Library => AppView::Home,
            AppView::SearchBook => AppView::Home,
            AppView::PodcastEpisode => AppView::Home,
            AppView::Libraries => AppView::Home,
            AppView::Auth => AppView::Auth

        };
    }

    /// Select functions that apply to both views
    /// all select functions are from ListState widget
    pub fn select_next(&mut self) {
        match self.view_state {
            AppView::Home => self.list_state_cnt_list.select_next(),
            AppView::Library => self.list_state_library.select_next(),
            AppView::SearchBook => self.list_state_search_results.select_next(),
            AppView::PodcastEpisode => self.list_state_pod_ep.select_next(),
            AppView::Libraries => self.list_state_libraries.select_next(),
            AppView::Auth => {},
        }
    }

    pub fn select_previous(&mut self) {
        match self.view_state {
            AppView::Home => self.list_state_cnt_list.select_previous(),
            AppView::Library => self.list_state_library.select_previous(),
            AppView::SearchBook => self.list_state_search_results.select_previous(),
            AppView::PodcastEpisode => self.list_state_pod_ep.select_previous(),
            AppView::Libraries => self.list_state_libraries.select_previous(),
            AppView::Auth => {},
        }
    }

    pub fn select_first(&mut self) {
        match self.view_state {
            AppView::Home => self.list_state_cnt_list.select_first(),
            AppView::Library => self.list_state_library.select_first(),
            AppView::SearchBook => self.list_state_search_results.select_first(),
            AppView::PodcastEpisode => self.list_state_pod_ep.select_first(),
            AppView::Libraries => self.list_state_libraries.select_first(),
            AppView::Auth => {},
        }
    }

    pub fn select_last(&mut self) {
        match self.view_state {
            AppView::Home => self.list_state_cnt_list.select_last(),
            AppView::Library => self.list_state_library.select_last(),
            AppView::SearchBook => self.list_state_search_results.select_last(),
            AppView::PodcastEpisode => self.list_state_pod_ep.select_last(),
            AppView::Libraries => self.list_state_libraries.select_last(),
            AppView::Auth => {},
        }
    }

 }
