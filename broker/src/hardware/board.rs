/*!board.rs
 * Board lib. Provides support for unpacking simpi json files
 * 
 * Author: Patrick Goldinger
 * License: GPL 3.0 (see LICENSE file for details)
 */

use super::part::Part;
use utils::gpioregs::RegMemory;
use tui::backend::CrosstermBackend;
use tui::layout::{Rect};
use tui::style::{Color, Modifier, Style};
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
            height: 32,
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
    pub fn render<F>(&self, f: F, area: Rect)
        where F: FnOnce(Frame<CrosstermBackend<dyn std::io::Write>>)
    {}
}
