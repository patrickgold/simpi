/*!led.rs
 * Hardware | Led definition.
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
pub struct Led {
    pub pin: u8,
    pub name: String,
    pub color_off: Color,
    pub color_on: Color,
    pub pos_x: u16,
    pub pos_y: u16,
    state: bool,
}

impl Default for Led {
    fn default() -> Self {
        Self {
            pin: 0,
            name: String::from("LED $n"),
            color_off: Color::Black,
            color_on: Color::Red,
            pos_x: 0,
            pos_y: 0,
            state: false,
        }
    }
}

impl Led {
    fn get(&self) -> bool {
        self.state
    }
    fn set(&mut self, state: bool) {
        self.state = state
    }
    pub fn sync(&mut self, reg_memory: &RegMemory) -> &mut Self {
        self.state = reg_memory.output.read_pin(self.pin) != 0;
        self
    }
    pub fn render<F>(&self, f: F, area: Rect)
        where F: FnOnce(Frame<CrosstermBackend<dyn std::io::Write>>)
    {}
}
