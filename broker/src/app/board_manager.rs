/*!board_manager.rs
 * Manages all loaded boards and renders the manager UI.
 * 
 * Author: Patrick Goldinger
 * License: GPL 3.0 (see LICENSE file for details)
 */

use crate::hardware;
use std::io::{Error, ErrorKind, Read};
use tui::backend::CrosstermBackend;
use tui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Borders, Paragraph, Row, Table, Text, Widget};
use tui::Frame;

#[derive(Clone)]
pub struct BoardManager {
    active_tab: u32,
    pub boards: Vec<hardware::Board>,
    is_active: bool,
}

impl Default for BoardManager {
    fn default() -> Self {
        Self {
            active_tab: 1, // Overview
            boards: vec![],
            is_active: false,
        }
    }
}

impl BoardManager {
    pub fn set_active(&mut self, s: bool) {
        self.is_active = s;
    }
    pub fn event_keypress(&mut self, c: char) -> bool {
        if self.is_active && c.is_digit(10) {
            let inp = c.to_digit(10).unwrap();
            if inp >= 1 && inp <= 9 {
                self.active_tab = inp;
                return true;
            }
            return false;
        } else {
            return false;
        }
    }
    pub fn render(
        &self, f: &mut Frame<'_, CrosstermBackend<std::io::Stdout>>, area: Rect
    ) {
        let bm_layout = Layout::default()
            .direction(Direction::Horizontal)
            .margin(1)
            .constraints([
                Constraint::Length(1),     // Menu Padding Left
                Constraint::Length(20),    // Menu
                Constraint::Length(1),     // List Padding Left
                Constraint::Min(1),        // List or Action area
                Constraint::Length(1),     // List Padding Right
            ].as_ref())
            .split(area);
        Block::default()
            .title(" Board Manager ")
            .borders(Borders::ALL)
            .render(f, area);
        let selected_style = Style::default()
            .fg(Color::White).bg(Color::Red);
        let normal_style = Style::default();
        let _disabled_style = Style::default()
            .fg(Color::DarkGray);
        let mut menu_styles = vec![];
        for _ in 0..9 {
            menu_styles.push(normal_style);
        }
        menu_styles[(self.active_tab-1) as usize] = selected_style;
        Paragraph::new([
            Text::raw("\n"),
            Text::styled("(1) Overview      \n", menu_styles[0]),
            Text::styled("(2) Add Board     \n", menu_styles[1]),
            Text::styled("(3) Modify Board  \n", menu_styles[2]),
            Text::styled("(4) Remove Board  \n", menu_styles[3]),
            Text::styled("(5) ?             \n", menu_styles[4]),
            Text::styled("(6) ?             \n", menu_styles[5]),
            Text::styled("(7) ?             \n", menu_styles[6]),
            Text::styled("(8) ?             \n", menu_styles[7]),
            Text::styled("(9) ?             \n", menu_styles[8]),
        ].iter())
            .block(Block::default().borders(Borders::RIGHT))
            .alignment(Alignment::Left)
            .render(f, bm_layout[1]);
        // Draw Board List
        if self.active_tab == 1 {
            let table_layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(1),     // Table Padding Top
                    Constraint::Min(1),        // Table
                ].as_ref())
                .split(bm_layout[3]);
            let selected_style = Style::default()
                .fg(Color::Red);
            let normal_style = Style::default();
            let _disabled_style = Style::default()
                .fg(Color::DarkGray).bg(Color::Black);
            let table_header = [
                "#", "Name", "Size", "Hardware"
            ];
            let table_rows = self.boards.iter().enumerate().map(|(i, board)| {
                Row::StyledData(vec![
                    (i+1).to_string(),
                    board.name.clone(),
                    board.width.to_string() + "x" + &board.height.to_string(),
                    "[NYI]".to_owned(),
                ].into_iter(), normal_style)
            });
            Table::new(table_header.iter(), table_rows)
                .block(Block::default())
                .widths(&[
                    Constraint::Length(3),
                    Constraint::Min(20),
                    Constraint::Min(10),
                    Constraint::Min(20),
                ])
                .header_style(Style::default().modifier(Modifier::BOLD))
                .column_spacing(1)
                .render(f, table_layout[1]);
        } else {
            Paragraph::new([
                Text::raw("\nThis section is [NYI]!"),
            ].iter())
                .render(f, bm_layout[3]);
        }
    }
}
