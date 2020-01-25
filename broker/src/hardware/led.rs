/*!led.rs
 * Hardware | Led definition.
 * 
 * Author: Patrick Goldinger
 * License: GPL 3.0 (see LICENSE file for details)
 */

use super::board::Board;
use serde_json::{Value as SerdeValue};
use std::io::{Error, ErrorKind};
use tui::backend::CrosstermBackend;
use tui::layout::{Rect};
use tui::style::{Color, Style};
use tui::widgets::{Block, Borders, Paragraph, Text, Widget};
use tui::Frame;
use utils::gpioregs::RegMemory;

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
            color_on: Color::LightRed,
            pos_x: 0,
            pos_y: 0,
            state: false,
        }
    }
}

impl Led {
    pub fn from_json(json: SerdeValue) -> Result<Self, Error> {
        match json {
            SerdeValue::Object(map) => {
                let mut led = Self::default();
                let mut is_valid = false;
                for (k, v) in map.iter() {
                    match k.as_ref() {
                        "type" => {
                            if v.is_string() {
                                if v.as_str().unwrap() == "simpi/led" {
                                    is_valid = true;
                                }
                            }
                        },
                        "name" => {
                            if v.is_string() {
                                led.name = v.as_str().unwrap().to_owned();
                            }
                        },
                        "pin" => {
                            if v.is_u64() {
                                led.pin = v.as_u64().unwrap() as u8;
                            }
                        },
                        "colorOff" => {
                            if v.is_string() {
                                let c = super::helper_str_to_color(v.as_str().unwrap().to_owned());
                                if c.is_ok() {
                                    led.color_off = c.unwrap();
                                }
                            }
                        },
                        "colorOn" => {
                            if v.is_string() {
                                let c = super::helper_str_to_color(v.as_str().unwrap().to_owned());
                                if c.is_ok() {
                                    led.color_on = c.unwrap();
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
                                                led.pos_x = v.as_u64().unwrap() as u16;
                                            }
                                        },
                                        "y" => {
                                            if v.is_u64() {
                                                led.pos_y = v.as_u64().unwrap() as u16;
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
                    Ok(led)
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
    pub fn sync(&mut self, reg_memory: &RegMemory) -> &mut Self {
        self.state = reg_memory.output.read_pin(self.pin) != 0;
        self
    }
    pub fn render(
        &self, f: &mut Frame<'_, CrosstermBackend<std::io::Stdout>>,
        area: Rect, board: &Board
    ) {
        let led_area = Rect {
            x: area.x + self.pos_x + 1,
            y: area.y + self.pos_y + 1,
            width: 9,
            height: 2,
        };
        if !super::helper_is_rect_in_range(area, led_area) {
            return;
        }
        let led_content = [
            Text::styled("  ", Style::default().bg(
                if self.state { self.color_on } else { self.color_off }
            )),
            Text::raw(" "),
            Text::styled(self.name.clone(), Style::default()
                .fg(board.foreground_color)
                .bg(board.background_color)
            ),
        ];
        Paragraph::new(led_content.iter())
            .block(Block::default()
                .borders(Borders::NONE)
            )
            .style(Style::default().bg(board.background_color))
            .render(f, led_area);
    }
}
