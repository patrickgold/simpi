/*!board.rs
 * Board lib. Provides support for unpacking simpi json files
 * 
 * Author: Patrick Goldinger
 * License: GPL 3.0 (see LICENSE file for details)
 */

use super::part::Part;
use utils::gpioregs::RegMemory;
use tui::backend::CrosstermBackend;
use tui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Borders, Paragraph, Text, Widget};
use tui::Frame;

#[derive(Clone)]
pub struct Board {
    pub name: String,
    pub background_color: Color,
    pub foreground_color: Color,
    pub width: u16,
    pub height: u16,
    pub hardware: Vec<Part>,
}

impl Default for Board {
    fn default() -> Self {
        Self {
            name: String::from("Board $n"),
            background_color: Color::Green,
            foreground_color: Color::White,
            width: 64,
            height: 24,
            hardware: vec![],
        }
    }
}

impl Board {
    pub fn sync(&mut self, reg_memory: &mut RegMemory) -> &mut Self {
        for part in self.hardware.iter_mut() {
            match part {
                Part::Led(led) => { led.sync(reg_memory); },
                Part::Button(button) => { button.sync(reg_memory); },
            }
        }
        self
    }
    pub fn render(
        &self, f: &mut Frame<'_, CrosstermBackend<std::io::Stdout>>, area: Rect
    ) {
        let board_area: Rect;
        if area.width >= self.width && area.height >= self.height {
            board_area = Rect {
                width: self.width,
                height: self.height,
                ..area
            }
        } else {
            board_area = area;
        }
        Block::default()
            .title(self.name.as_ref())
            .title_style(Style::default().fg(self.foreground_color))
            .borders(Borders::ALL)
            .border_style(Style::default().fg(self.foreground_color))
            .style(Style::default().bg(self.background_color))
            .render(f, board_area);
        for part in self.hardware.iter() {
            match part {
                Part::Led(led) => { led.render(f, board_area, &self); },
                Part::Button(button) => { button.render(f, board_area, &self); },
            }
        }
    }
}
