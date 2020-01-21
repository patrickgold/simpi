/*!button.rs
 * Hardware | Button definition.
 * 
 * Author: Patrick Goldinger
 * License: GPL 3.0 (see LICENSE file for details)
 */

use utils::gpioregs::RegMemory;
use tui::backend::CrosstermBackend;
use tui::layout::{Rect};
use tui::style::{Color, Modifier, Style};
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
    fn get(&self) -> bool {
        self.state
    }
    fn set(&mut self, state: bool) {
        self.state = state
    }
    pub fn sync(&mut self, reg_memory: &mut RegMemory) -> &mut Self {
        reg_memory.input.write_pin(self.pin, self.state as u8);
        self
    }
    pub fn render<F>(&self, f: F, area: Rect)
        where F: FnOnce(Frame<CrosstermBackend<dyn std::io::Write>>)
    {}
}
