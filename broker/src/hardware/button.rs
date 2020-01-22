/*!button.rs
 * Hardware | Button definition.
 * 
 * Author: Patrick Goldinger
 * License: GPL 3.0 (see LICENSE file for details)
 */

use super::board::Board;
use utils::gpioregs::RegMemory;
use tui::backend::CrosstermBackend;
use tui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Borders, Paragraph, Text, Widget};
use tui::Frame;

#[derive(Clone)]
pub struct Button {
    pub pin: u8,
    pub name: String,
    pub hotkey: String,
    pub color_off: Color,
    pub color_on: Color,
    pub pos_x: u16,
    pub pos_y: u16,
    state: bool,
}

impl Default for Button {
    fn default() -> Self {
        Self {
            pin: 0,
            name: String::from("BTN $n"),
            hotkey: String::from(""),
            color_off: Color::Black,
            color_on: Color::Yellow,
            pos_x: 0,
            pos_y: 0,
            state: false,
        }
    }
}

impl Button {
    pub fn get(&self) -> bool {
        self.state
    }
    pub fn set(&mut self, state: bool) {
        self.state = state
    }
    pub fn sync(&mut self, reg_memory: &mut RegMemory) -> &mut Self {
        reg_memory.input.write_pin(self.pin, self.state as u8);
        self
    }
    pub fn render(
        &self, f: &mut Frame<'_, CrosstermBackend<std::io::Stdout>>,
        area: Rect, board: &Board
    ) {
        let button_area = Rect {
            x: area.x + self.pos_x + 1,
            y: area.y + self.pos_y + 1,
            width: 12,
            height: 3,
        };
        let button_content = [
            Text::styled("  ", Style::default().bg(
                if self.state { self.color_on } else { self.color_off }
            )),
            Text::raw(" "),
            Text::styled(
                ((self.name.clone() + "\n   [") + 
                    self.hotkey.clone().as_ref()) + "]", 
            Style::default()
                .fg(board.foreground_color)
                .bg(board.background_color)
            ),
        ];
        Paragraph::new(button_content.iter())
            .block(Block::default()
                .borders(Borders::NONE)
            )
            .style(Style::default().bg(board.background_color))
            .render(f, button_area);
    }
}
