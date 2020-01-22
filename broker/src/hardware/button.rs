/*!button.rs
 * Hardware | Button definition.
 * 
 * Author: Patrick Goldinger
 * License: GPL 3.0 (see LICENSE file for details)
 */

use super::board::Board;
use serde_json::{Value as SerdeValue};
use std::io::{Error, ErrorKind};
use tui::backend::CrosstermBackend;
use tui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Borders, Paragraph, Text, Widget};
use tui::Frame;
use utils::gpioregs::RegMemory;

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
    pub fn from_json(json: SerdeValue) -> Result<Self, Error> {
        match json {
            SerdeValue::Object(map) => {
                let mut button = Self::default();
                let mut is_valid = false;
                for (k, v) in map.iter() {
                    match k.as_ref() {
                        "type" => {
                            if v.is_string() {
                                if v.as_str().unwrap() == "simpi/button" {
                                    is_valid = true;
                                }
                            }
                        },
                        "name" => {
                            if v.is_string() {
                                button.name = v.as_str().unwrap().to_owned();
                            }
                        },
                        "pin" => {
                            if v.is_u64() {
                                button.pin = v.as_u64().unwrap() as u8;
                            }
                        },
                        "hotkey" => {
                            if v.is_string() {
                                button.hotkey = v.as_str().unwrap().to_owned();
                            }
                        },
                        "colorOff" => {
                            if v.is_string() {
                                let c = super::helper_str_to_color(v.as_str().unwrap().to_owned());
                                if c.is_ok() {
                                    button.color_off = c.unwrap();
                                }
                            }
                        },
                        "colorOn" => {
                            if v.is_string() {
                                let c = super::helper_str_to_color(v.as_str().unwrap().to_owned());
                                if c.is_ok() {
                                    button.color_on = c.unwrap();
                                }
                            }
                        },
                        "position" => {
                            if v.is_object() {
                                let s = v.as_object().unwrap();
                                for (k, v) in s.iter() {
                                    match k.as_ref() {
                                        "x" => {
                                            if v.is_u64() {
                                                button.pos_x = v.as_u64().unwrap() as u16;
                                            }
                                        },
                                        "y" => {
                                            if v.is_u64() {
                                                button.pos_y = v.as_u64().unwrap() as u16;
                                            }
                                        },
                                        _ => {}
                                    }
                                }
                            }
                        },
                        _ => {}
                    }
                }
                if is_valid {
                    Ok(button)
                } else {
                    Err(Error::new(ErrorKind::InvalidData, "Input data is invalid!"))
                }
            },
            _ => Err(Error::new(ErrorKind::InvalidInput, "Input must be map!"))
        }
    }
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
