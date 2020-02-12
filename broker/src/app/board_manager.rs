/*!board_manager.rs
 * Manages all loaded boards and renders the manager UI.
 * 
 * Author: Patrick Goldinger
 * License: GPL 3.0 (see LICENSE file for details)
 */

use crate::hardware;
use crossterm::event::KeyCode;
use std::io::{Error};
use tui::backend::CrosstermBackend;
use tui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Borders, Paragraph, Row, Table, Text, Widget};
use tui::Frame;

enum FocusArea {
    Tab,
    Content,
    Field,
}

pub struct BoardManager {
    active_tab: usize,
    active_content: usize,
    active_field: usize,
    pub boards: Vec<hardware::Board>,
    current_focus_area: FocusArea,
    items_tab: usize,
    items_content: usize,
    items_field: usize,
    tmp_open_board: Result<hardware::Board, Error>,
    tmp_open_board_str: String,
}

impl Default for BoardManager {
    fn default() -> Self {
        Self {
            active_tab: 1, // Overview
            active_content: 0,
            active_field: 0,
            boards: vec![],
            current_focus_area: FocusArea::Tab,
            items_tab: 6,
            items_content: 0,
            items_field: 0,
            tmp_open_board: Ok(hardware::Board::default()),
            tmp_open_board_str: String::new(),
        }
    }
}

impl BoardManager {
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
    pub fn event_keypress(&mut self, c: char) -> bool {
        match self.active_tab {
            3 => {
                if let FocusArea::Content = self.current_focus_area {
                    self.tmp_open_board_str.push(c);
                } else if let FocusArea::Field = self.current_focus_area {
                    if c == 'y' {
                        self.event_keypress_special(KeyCode::Enter);
                    } else if c == 'n' {
                        self.event_keypress_special(KeyCode::Esc);
                    }
                }
            },
            6 => {
                if let FocusArea::Field = self.current_focus_area {
                    if c == 'y' {
                        self.boards.remove(self.active_content-1);
                        self.current_focus_area = FocusArea::Content;
                        self.active_content = 1;
                    } else if c == 'n' {
                        self.event_keypress_special(KeyCode::Esc);
                    }
                }
            },
            _ => {}
        }
        return false;
    }
    pub fn event_keypress_special(&mut self, c: KeyCode) -> bool {
        match c {
            KeyCode::Up => {
                if let FocusArea::Tab = self.current_focus_area {
                    if self.active_tab > 1 {
                        self.active_tab -= 1;
                    }
                } else if let FocusArea::Content = self.current_focus_area {
                    if self.active_content > 1 {
                        self.active_content -= 1;
                    }
                } else if let FocusArea::Field = self.current_focus_area {
                    if self.active_field > 1 {
                        self.active_field -= 1;
                    }
                }
                return true;
            },
            KeyCode::Down => {
                if let FocusArea::Tab = self.current_focus_area {
                    if self.active_tab < self.items_tab {
                        self.active_tab += 1;
                    }
                } else if let FocusArea::Content = self.current_focus_area {
                    if self.active_content < self.items_content {
                        self.active_content += 1;
                    }
                } else if let FocusArea::Field = self.current_focus_area {
                    if self.active_field < self.items_field {
                        self.active_field += 1;
                    }
                }
                return true;
            },
            KeyCode::Enter => {
                if let FocusArea::Tab = self.current_focus_area {
                    self.current_focus_area = FocusArea::Content;
                    self.active_content = 1;
                } else if let FocusArea::Content = self.current_focus_area {
                    if self.active_tab == 3 && self.tmp_open_board_str.len() > 0 {
                        self.tmp_open_board = hardware::Board::from_file(
                            self.tmp_open_board_str.as_str()
                        );
                        self.current_focus_area = FocusArea::Field;
                        self.active_field = 1;
                    }
                    if self.active_tab != 3 && self.items_content > 0 {
                        self.current_focus_area = FocusArea::Field;
                        self.active_field = 1;
                    }
                } else if let FocusArea::Field = self.current_focus_area {
                    if self.active_tab == 3 {
                        if self.tmp_open_board.as_ref().is_ok() {
                            self.boards.push(self.tmp_open_board.as_ref().unwrap().clone());
                        }
                        self.current_focus_area = FocusArea::Tab;
                        self.active_content = 0;
                        self.items_content = 0;
                        self.active_field = 0;
                        self.items_field = 0;
                        self.tmp_open_board_str = String::new();
                    }
                }
                return true;
            },
            KeyCode::Esc => {
                if let FocusArea::Content = self.current_focus_area {
                    self.current_focus_area = FocusArea::Tab;
                    self.active_content = 0;
                    self.items_content = 0;
                } else if let FocusArea::Field = self.current_focus_area {
                    self.current_focus_area = FocusArea::Content;
                    self.active_field = 0;
                    self.items_field = 0;
                }
                return true;
            },
            KeyCode::Backspace => {
                if let FocusArea::Content = self.current_focus_area {
                    if self.active_tab == 3 && self.tmp_open_board_str.len() > 0 {
                        self.tmp_open_board_str.pop();
                    }
                }
                return true;
            },
            _ => {
                return false;
            }
        }
    }
    fn render_list(
        &mut self, f: &mut Frame<'_, CrosstermBackend<std::io::Stdout>>, area: Rect,
        show_selection: bool
    ) {
        let table_header = [
            "#", "Name", "Size", "Hardware"
        ];
        self.items_content = self.boards.len();
        let table_rows = self.boards.iter().enumerate().map(|(i, board)| {
            let style = if self.active_content > 0 && show_selection {
                if i == self.active_content - 1 {
                    Self::style_hover()
                } else {
                    Self::style_normal()
                }
            } else {
                Self::style_normal()
            };
            Row::StyledData(vec![
                (i+1).to_string(),
                board.name.clone(),
                board.width.to_string() + "x" + &board.height.to_string(),
                board.get_hardware_summary(),
            ].into_iter(), style)
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
            .render(f, area);
    }
    pub fn render(
        &mut self, f: &mut Frame<'_, CrosstermBackend<std::io::Stdout>>, area: Rect
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
        let mut menu_styles = vec![];
        for _ in 0..6 {
            menu_styles.push(Self::style_normal());
        }
        if let FocusArea::Tab = self.current_focus_area {
            menu_styles[self.active_tab-1] = Self::style_hover();
        } else {
            menu_styles[self.active_tab-1] = Self::style_selected();
        }
        Paragraph::new([
            Text::raw("\n"),
            Text::styled("Overview          \n", menu_styles[0]),
            Text::styled("New Board         \n", menu_styles[1]),
            Text::styled("Open Board        \n", menu_styles[2]),
            Text::styled("Modify Board      \n", menu_styles[3]),
            Text::styled("Save Board        \n", menu_styles[4]),
            Text::styled("Remove Board      \n", menu_styles[5]),
        ].iter())
            .block(Block::default().borders(Borders::RIGHT))
            .alignment(Alignment::Left)
            .render(f, bm_layout[1]);
        // Draw tab content
        let tc_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1),     // Tab Content Padding Top
                Constraint::Min(1),        // Tab Content
                Constraint::Length(2),     // Bottom tooltip
            ].as_ref())
            .split(bm_layout[3]);
        // Draw content
        match self.active_tab {
            1 => {
                if let FocusArea::Field = self.current_focus_area {
                    let preview_layout = Layout::default()
                        .direction(Direction::Vertical)
                        .constraints([
                            Constraint::Length(2),     // Heading
                            Constraint::Min(1),        // Content
                        ].as_ref())
                        .split(tc_layout[1]);
                    let b = self.boards.get(self.active_content-1).unwrap();
                    Paragraph::new([
                        Text::styled("Preview of '",
                            Style::default().modifier(Modifier::BOLD)
                        ),
                        Text::styled(b.name.as_str(),
                            Style::default().modifier(Modifier::BOLD)
                        ),
                        Text::styled("'",
                            Style::default().modifier(Modifier::BOLD)
                        ),
                    ].iter())
                        .wrap(true)
                        .render(f, preview_layout[0]);
                    b.render(f, preview_layout[1]);
                } else {
                    self.render_list(f, tc_layout[1], true);
                }
            },
            3 => {
                if let FocusArea::Field = self.current_focus_area {
                    let preview_layout = Layout::default()
                        .direction(Direction::Vertical)
                        .constraints([
                            Constraint::Length(2),     // Heading
                            Constraint::Min(1),        // Content
                            Constraint::Length(2),     // Bottom Question
                        ].as_ref())
                        .split(tc_layout[1]);
                    if self.tmp_open_board.is_ok() {
                        let b = self.tmp_open_board.as_ref().unwrap();
                        Paragraph::new([
                            Text::styled("Preview of '",
                                Style::default().modifier(Modifier::BOLD)
                            ),
                            Text::styled(b.name.as_str(),
                                Style::default().modifier(Modifier::BOLD)
                            ),
                            Text::styled("'",
                                Style::default().modifier(Modifier::BOLD)
                            ),
                        ].iter())
                            .wrap(true)
                            .render(f, preview_layout[0]);
                        b.render(f, preview_layout[1]);
                        Paragraph::new([
                            Text::raw("Do you want to add this board?\n"),
                            Text::raw("<y> Yes        <n> No\n"),
                        ].iter())
                            .wrap(true)
                            .render(f, preview_layout[2]);
                    } else {
                        let err = self.tmp_open_board.as_ref().err().unwrap();
                        Paragraph::new([
                            Text::styled("An Error occured while loading '",
                                Style::default().modifier(Modifier::BOLD)
                            ),
                            Text::styled(self.tmp_open_board_str.as_str(),
                                Style::default().modifier(Modifier::BOLD)
                            ),
                            Text::styled("'",
                                Style::default().modifier(Modifier::BOLD)
                            ),
                        ].iter())
                            .wrap(true)
                            .render(f, preview_layout[0]);
                        Paragraph::new([
                            Text::raw(format!("{}", err)),
                        ].iter())
                            .wrap(true)
                            .render(f, preview_layout[1]);
                        Paragraph::new([
                            Text::raw("<Enter> Exit        <Esc> Modify input file path\n"),
                        ].iter())
                            .wrap(true)
                            .render(f, preview_layout[2]);
                    }
                } else {
                    Paragraph::new([
                        Text::styled("Open and load a board from a given JSON file\n\n",
                            Style::default().modifier(Modifier::BOLD)
                        ),
                        Text::raw("File: "),
                        Text::styled(self.tmp_open_board_str.clone(),
                            Style::default().modifier(Modifier::UNDERLINED)
                        ),
                        if let FocusArea::Content = self.current_focus_area {
                            Text::styled(" ", 
                                Self::style_hover()
                            )
                        } else {
                            Text::raw("")
                        }
                    ].iter())
                        .wrap(true)
                        .render(f, tc_layout[1]);
                }
            },
            6 => {
                if let FocusArea::Field = self.current_focus_area {
                    Paragraph::new([
                        Text::raw("Do you want to remove board #"),
                        Text::raw(self.active_content.to_string()),
                        Text::raw(" with the name '"),
                        Text::raw(self.boards[self.active_content-1].name.as_str()),
                        Text::raw("'?\n\n"),
                        Text::raw("Note, that this action will NOT delete the JSON board file. If the selected board was a temporary board however, it will be deleted permanently!\n\n"),
                        Text::raw("<y> Yes       <n> No"),
                    ].iter())
                        .wrap(true)
                        .render(f, tc_layout[1]);
                } else {
                    self.render_list(f, tc_layout[1], true);
                }
            },
            _ => {
                self.items_content = 0;
                self.items_field = 0;
                Paragraph::new([
                    Text::raw("This section is [NYI]!"),
                ].iter())
                    .render(f, tc_layout[1]);
            }
        }
        // Draw footer
        Paragraph::new([
            Text::raw("<Up/Down Arrow> to move cursor, <Enter> to confirm, <Esc> to cancel"),
        ].iter())
            .block(Block::default().borders(Borders::TOP))
            .render(f, tc_layout[2]);
    }
}
