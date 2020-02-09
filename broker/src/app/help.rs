/*!help.rs
 * Manages help UI.
 * 
 * Author: Patrick Goldinger
 * License: GPL 3.0 (see LICENSE file for details)
 */

use crossterm::event::KeyCode;
use serde_json::{Value as SerdeValue};
use tui::backend::CrosstermBackend;
use tui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Borders, Paragraph, Text, Widget};
use tui::Frame;

const HELP_RAW: &'static str = include_str!("../../config/help.json");
const LICENSE: &'static str = include_str!("../../../LICENSE");

struct Article {
    pub title: String,
    pub menu_title: String,
    pub body: String,
}

impl Default for Article {
    fn default() -> Self {
        Self {
            title: String::new(),
            menu_title: String::new(),
            body: String::new(),
        }
    }
}

impl Article {
    pub fn create_vec_from_json_str(raw: &'static str) -> Vec<Article> {
        let json = serde_json::from_str(raw);
        if json.is_ok() {
            let mut ret = vec![];
            let json: SerdeValue = json.unwrap();
            if json.is_array() {
                let json = json.as_array().unwrap();
                for article in json.iter() {
                    if article.is_object() {
                        let mut tmp_article = Article::default();
                        let article = article.as_object().unwrap();
                        for (k, v) in article.iter() {
                            match k.as_str() {
                                "title" => {
                                    if v.is_string() {
                                        tmp_article.title = v.as_str().unwrap().to_owned();
                                    }
                                },
                                "menuTitle" => {
                                    if v.is_string() {
                                        tmp_article.menu_title = v.as_str().unwrap().to_owned();
                                    }
                                },
                                "body" => {
                                    if v.is_array() {
                                        for s in v.as_array().unwrap().iter() {
                                            if s.is_string() {
                                                tmp_article.body += s.as_str().unwrap();
                                                tmp_article.body += "\n\n";
                                            }
                                        }
                                    }
                                },
                                _ => {}
                            }
                        }
                        ret.push(tmp_article);
                    }
                }
            }
            return ret;
        } else {
            return vec![];
        }
    }
}

pub struct Help {
    active_article: usize,
    articles: Vec<Article>,
    overflow_lines: usize,
    scroll_offset: usize,
}

impl Default for Help {
    fn default() -> Self {
        let mut articles = Article::create_vec_from_json_str(HELP_RAW);
        articles.push(Article {
            title: "License Text for SimPi".to_owned(),
            menu_title: "License".to_owned(),
            body: LICENSE.to_owned(),
        });
        Self {
            active_article: 1, // Welcome
            articles,
            overflow_lines: 0,
            scroll_offset: 0,
        }
    }
}

impl Help {
    fn style_normal() -> Style {
        Style::default()
    }
    fn style_hover() -> Style {
        Style::default()
            .fg(Color::White).bg(Color::Red)
    }
    fn style_selected() -> Style {
        Style::default()
            .fg(Color::Black).bg(Color::White)
    }
    pub fn event_keypress(&mut self, _c: char) -> bool {
        return false;
    }
    pub fn event_keypress_special(&mut self, c: KeyCode) -> bool {
        match c {
            KeyCode::PageUp => {
                if self.active_article > 1 {
                    self.active_article -= 1;
                }
                self.overflow_lines = 0;
                self.scroll_offset = 0;
                return true;
            },
            KeyCode::PageDown => {
                if self.active_article < self.articles.len() {
                    self.active_article += 1;
                }
                self.overflow_lines = 0;
                self.scroll_offset = 0;
                return true;
            },
            KeyCode::Up => {
                if self.scroll_offset >= 4 {
                    self.scroll_offset -= 4;
                } else {
                    self.scroll_offset = 0;
                }
                return true;
            },
            KeyCode::Down => {
                if self.scroll_offset + 4 <= self.overflow_lines {
                    self.scroll_offset += 4;
                } else if self.scroll_offset <= self.overflow_lines {
                    self.scroll_offset = self.overflow_lines;
                }
                return true;
            },
            _ => {
                return false;
            }
        }
    }
    pub fn render(
        &mut self, f: &mut Frame<'_, CrosstermBackend<std::io::Stdout>>, area: Rect
    ) {
        let help_layout = Layout::default()
            .direction(Direction::Horizontal)
            .margin(1)
            .constraints([
                Constraint::Length(1),     // Menu Padding Left
                Constraint::Length(20),    // Menu
                Constraint::Length(1),     // Article Padding Left
                Constraint::Min(1),        // Article
                Constraint::Length(1),     // Article Padding Right
            ].as_ref())
            .split(area);
        Block::default()
            .title(" Help ")
            .borders(Borders::ALL)
            .render(f, area);
        let mut tab_items = vec![Text::raw("\n")];
        let mut n = 1;
        for article in self.articles.iter() {
            let text = (
                article.menu_title.clone() + " ".repeat(18).as_str()
            )[0..18].to_owned() + "\n";
            let style = if n == self.active_article {
                Self::style_hover()
            } else {
                Self::style_normal()
            }
;           tab_items.push(Text::styled(text, style));
            n += 1;
        }
        Paragraph::new(tab_items.iter())
            .block(Block::default().borders(Borders::RIGHT))
            .alignment(Alignment::Left)
            .render(f, help_layout[1]);
        // Draw tab content
        let content_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1),     // Article Padding Top
                Constraint::Min(1),        // Article
                Constraint::Length(2),     // Bottom toolbar
            ].as_ref())
            .split(help_layout[3]);
        // Draw content
        let article = self.articles.get(self.active_article-1).unwrap();
        let scroll_offset = {
            let lines = article.body.matches("\n").count() + 2;
            let height = content_layout[1].height;
            if lines >= height as usize {
                self.overflow_lines = lines - height as usize;
                self.scroll_offset
            } else {
                self.overflow_lines = 0;
                0
            }
        };
        Paragraph::new([
            Text::styled(article.title.clone(),
                Style::default().modifier(Modifier::BOLD)
            ),
            Text::raw("\n\n"),
            Text::raw(article.body.clone()),
        ].iter())
            .wrap(true)
            .scroll(scroll_offset as u16)
            .render(f, content_layout[1]);
        // Draw scrollbar if needed
        if self.overflow_lines > 0 {
            let lines = article.body.matches("\n").count() + 2;
            let scrollbar_layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(1),     // Padding Top
                    Constraint::Min(1),        // Scrollbar
                    Constraint::Length(2),     // Padding Bottom
                ].as_ref())
                .split(help_layout[4]);
            let scrollbar_track = scrollbar_layout[1];
            let mut track: Vec<Text> = vec![];
            for _ in 0..scrollbar_track.height {
                track.push(Text::styled(" \n",
                    Style::default().modifier(Modifier::REVERSED)
                ));
            }
            Paragraph::new(track.iter())
                .render(f, scrollbar_track);
            let scrollbar_thumb = Rect {
                y: scrollbar_track.y + (scrollbar_track.height as f32 * (
                    self.scroll_offset as f32 / lines as f32
                )).round() as u16,
                height: (scrollbar_track.height as f32 * (
                    scrollbar_track.height as f32 / lines as f32
                )).round() as u16,
                ..scrollbar_track
            };
            let mut thumb: Vec<Text> = vec![];
            for _ in 0..scrollbar_thumb.height {
                thumb.push(Text::styled(" \n",
                    Style::default().bg(Color::Red)
                ));
            }
            Paragraph::new(thumb.iter())
                .render(f, scrollbar_thumb);
        }
        // Draw footer
        Paragraph::new([
            Text::raw("<PageUp> Prev. Article    <PageDown> Next Article    <Up> Scroll up    <Down> Scroll down"),
        ].iter())
            .block(Block::default().borders(Borders::TOP))
            .render(f, content_layout[2]);
    }
}
