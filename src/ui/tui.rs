use crate::App;
use crate::app::AppView;
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
use tui_textarea::{Input, Key, TextArea};
use std::io;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;





// const for color theme
const TODO_HEADER_STYLE: Style = Style::new().fg(SLATE.c100).bg(BLUE.c800);
const NORMAL_ROW_BG: Color = SLATE.c950;
const ALT_ROW_BG_COLOR: Color = SLATE.c900;
const SELECTED_STYLE: Style = Style::new().bg(SLATE.c800).add_modifier(Modifier::BOLD);

/// init widget for selected AppView 
impl Widget for &mut App {
  fn render(self, area: Rect, buf: &mut Buffer) {
        match self.view_state {
            AppView::Home => self.render_home(area, buf),
            AppView::Library => self.render_library(area, buf),
            AppView::SearchBook => self.render_search_book(area, buf),
            AppView::PodcastEpisode => self.render_pod_ep(area, buf),
            AppView::Libraries => self.render_libraries(area, buf),
            AppView::Auth => self.render_auth(area, buf),
        }
    }
}


/// Rendering logic

impl App {
    /// AppView::Home rendering
    fn render_home(&mut self, area: Rect, buf: &mut Buffer) {
        let [header_area, main_area, footer_area] = Layout::vertical([
            Constraint::Length(2),
            Constraint::Fill(1),
            Constraint::Length(1),
        ]).areas(area);

        let [list_area, item_area] = Layout::vertical([Constraint::Fill(1), Constraint::Fill(1)]).areas(main_area);

        let render_list_title = "Continue Listening";
        let text_render_footer = "Use ↓↑ to move, → to play, s to search, q to quit.";

        App::render_header(header_area, buf);
        App::render_footer(footer_area, buf, text_render_footer);
        self.render_list(list_area, buf, render_list_title, &self.titles_cnt_list.clone(), &mut self.list_state_cnt_list.clone());
//        self.render_selected_item(item_area, buf, &mut self.list_state.clone());
    }

    /// AppView::Library rendering
    fn render_library(&mut self, area: Rect, buf: &mut Buffer) {
        let [header_area, main_area, footer_area] = Layout::vertical([
            Constraint::Length(2),
            Constraint::Fill(1),
            Constraint::Length(1),
        ]).areas(area);
        
        let [list_area, item_area] = Layout::vertical([Constraint::Fill(1), Constraint::Fill(1)]).areas(main_area);

        let render_list_title = "Library";
        let text_render_footer = "Use ↓↑ to move, → to play, s to search, q to quit.";

        App::render_header(header_area, buf);
        App::render_footer(footer_area, buf, text_render_footer);
        self.render_list(list_area, buf, render_list_title, &self.titles_library.clone(), &mut self.list_state_library.clone());
        //self.render_selected_item(item_area, buf, &self.titles_library.clone(), self.auth_names_library.clone());
    }

    /// AppView::Library rendering
    fn render_auth(&mut self, area: Rect, buf: &mut Buffer) {

        //self.auth();



    }

//        App::render_footer(footer_area, buf, text_render_footer);
        //self.render_selected_item(item_area, buf, &self.titles_library.clone(), self.auth_names_library.clone());
    

    /// AppView::Libraries rendering
    fn render_libraries(&mut self, area: Rect, buf: &mut Buffer) {
        let [header_area, main_area, footer_area] = Layout::vertical([
            Constraint::Length(2),
            Constraint::Fill(1),
            Constraint::Length(1),
        ]).areas(area);
        
        let [list_area, item_area] = Layout::vertical([Constraint::Fill(1), Constraint::Fill(1),]).areas(main_area);

        let render_list_title = "Libraries";
        let text_render_footer = "Use ↓↑ to move, → to play, s to search, q to quit.";

        App::render_header(header_area, buf);
        App::render_footer(footer_area, buf, text_render_footer);
        self.render_list(list_area, buf, render_list_title, &self.library_names.clone(), &mut self.list_state_libraries.clone());
        //self.render_selected_item(item_area, buf, &self.titles_library.clone(), self.auth_names_library.clone());
    }

    /// AppView::SearchBook rendering
    fn render_search_book(&mut self, area: Rect, buf: &mut Buffer) {
        let [header_area, main_area, footer_area] = Layout::vertical([
            Constraint::Length(2),
            Constraint::Fill(1),
            Constraint::Length(1),
        ]).areas(area);

        let [list_area, item_area] = Layout::vertical([Constraint::Fill(1), Constraint::Fill(1)]).areas(main_area);

        let render_list_title = "Search result";
        let text_render_footer = "Use Tab to back home, ↓↑ to move, → to play, s to search, q to quit.";

        if self.search_mode {
            if let Ok(query) = self.search_active() {
                self.search_query = query.to_string();
                self.search_mode = false; 
            }
        }

        // init variables for search result (search by a book by title)
        let idx_and_titles: Vec<(usize, String)> = self.titles_library
            .iter()
            .enumerate() 
            .filter(|(_, x)| x.to_lowercase().contains(&self.search_query.to_lowercase())) 
            .map(|(index, title)| (index, title.clone())) 
            .collect();

        let mut titles_search_book: Vec<String> = Vec::new();
        let mut index_to_keep: Vec<usize> = Vec::new();
        for (index, title) in idx_and_titles {
            titles_search_book.push(title.to_string());
            index_to_keep.push(index)
        }

        let titles_search_book: &[String] = &titles_search_book;

        // for book
        self.ids_search_book = self.ids_library
            .iter()
            .enumerate()
            .filter(|(index, _)| index_to_keep.contains(&index))
            .map(|(_, value)| value.clone())
            .collect();

        // for podacst
        self.all_titles_pod_ep_search = self.all_titles_pod_ep
            .iter()
            .enumerate()
            .filter(|(index, _)| index_to_keep.contains(&index))
            .map(|(_, value)| value.clone())
            .collect();
        self.all_ids_pod_ep_search = self.all_ids_pod_ep
            .iter()
            .enumerate()
            .filter(|(index, _)| index_to_keep.contains(&index))
            .map(|(_, value)| value.clone())
            .collect();
        self.ids_library_pod_search = self.ids_library
            .iter()
            .enumerate()
            .filter(|(index, _)| index_to_keep.contains(&index))
            .map(|(_, value)| value.clone())
            .collect();

        App::render_header(header_area, buf);
        App::render_footer(footer_area, buf, text_render_footer);
        self.render_list(list_area, buf, render_list_title, titles_search_book, &mut self.list_state_search_results.clone());
        //self.render_selected_item(item_area, buf, &self.titles_library.clone(), self.auth_names_library.clone());

    }

    /// AppView::PodcastEpisode
    fn render_pod_ep(&mut self, area: Rect, buf: &mut Buffer) {
        let [header_area, main_area, footer_area] = Layout::vertical([
            Constraint::Length(2),
            Constraint::Fill(1),
            Constraint::Length(1),
        ]).areas(area);
        
        let [list_area, item_area] = Layout::vertical([Constraint::Fill(1), Constraint::Fill(1)]).areas(main_area);

        let render_list_title = "Pod Ep";
        let text_render_footer = "Use ↓↑ to move, → to play, s to search, q to quit.";

        App::render_header(header_area, buf);
        App::render_footer(footer_area, buf, text_render_footer);
        if self.is_from_search_pod {
        self.render_list(list_area, buf, render_list_title, &self.titles_pod_ep_search.clone(), &mut self.list_state_pod_ep.clone());
        } else {
        self.render_list(list_area, buf, render_list_title, &self.titles_pod_ep.clone(), &mut self.list_state_pod_ep.clone());
        }
        //self.render_selected_item(item_area, buf, &self.titles_library.clone(), self.auth_names_library.clone());
    }

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

