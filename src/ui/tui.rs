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
        Block, Borders, HighlightSpacing, List, ListItem , Paragraph, StatefulWidget,
        Widget,
    },
};
use ratatui::widgets::Wrap;

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

        App::render_header(header_area, buf);
        App::render_footer(footer_area, buf);
        self.render_list(list_area, buf, render_list_title, &self.titles.clone());
        self.render_selected_item(item_area, buf);
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

        App::render_header(header_area, buf);
        App::render_footer(footer_area, buf);
        self.render_list(list_area, buf, render_list_title, &self.titles_library.clone());
        self.render_selected_item(item_area, buf);
    }

    /// General functions for rendering 

    fn render_header(area: Rect, buf: &mut Buffer) {
        Paragraph::new("< Home >")
            .bold()
            .centered()
            .render(area, buf);
    }

    fn render_footer(area: Rect, buf: &mut Buffer) {
        Paragraph::new("Use ↓↑ to move, → to play, g/G to go top/bottom, q to quit.")
            .centered()
            .render(area, buf);

        Paragraph::new("toutui v0.1.0")
            .right_aligned()
            .render(area, buf);
    }

    fn render_list(&mut self, area: Rect, buf: &mut Buffer, render_list_title: &str, render_list_items: &[String]) {
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

        StatefulWidget::render(list, area, buf, &mut self.list_state);
    }

    fn render_selected_item(&self, area: Rect, buf: &mut Buffer) {
        if let Some(selected) = self.list_state.selected() {
            let content = &self.authors_names[selected];
            Paragraph::new(content.clone())
                .wrap(Wrap { trim: true })
                .render(area, buf);
        }
    }

    const fn alternate_colors(i: usize) -> Color {
        if i % 2 == 0 {
            NORMAL_ROW_BG
        } else {
            ALT_ROW_BG_COLOR
        }
    }
}

