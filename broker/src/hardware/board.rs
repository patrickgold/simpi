/*!board.rs
 * Board lib. Provides support for unpacking simpi json files
 * 
 * Author: Patrick Goldinger
 * License: GPL 3.0 (see LICENSE file for details)
 */

use super::{button::Button, led::Led, part::Part};
use serde_json::{Value as SerdeValue};
use std::fs::File;
use std::io::{Error, ErrorKind, Read};
use tui::backend::CrosstermBackend;
use tui::layout::{Rect};
use tui::style::{Color, Style};
use tui::widgets::{Block, Borders, Widget};
use tui::Frame;
use utils::gpioregs::RegMemory;

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
    pub fn from_file(file_name: &str) -> Result<Self, Error> {
        let file = File::open(file_name);
        if file.is_ok() {
            let mut file = file.unwrap();
            let mut data = String::new();
            file.read_to_string(&mut data).unwrap();
            let data = serde_json::from_str(data.as_ref()).unwrap();
            let board = Board::from_json(data);
            if board.is_ok() {
                Ok(board.unwrap())
            } else {
                Err(board.err().unwrap())
            }
        } else {
            Err(file.err().unwrap())
        }
    }
    pub fn from_json(json: SerdeValue) -> Result<Self, Error> {
        match json {
            SerdeValue::Object(map) => {
                let mut board = Self::default();
                let mut is_valid = false;
                for (k, v) in map.iter() {
                    match k.as_ref() {
                        "type" => {
                            if v.is_string() {
                                if v.as_str().unwrap() == "simpi/board" {
                                    is_valid = true;
                                }
                            }
                        },
                        "name" => {
                            if v.is_string() {
                                board.name = v.as_str().unwrap().to_owned();
                            }
                        },
                        "backgroundColor" => {
                            if v.is_string() {
                                let c = super::helper_str_to_color(v.as_str().unwrap().to_owned());
                                if c.is_ok() {
                                    board.background_color = c.unwrap();
                                }
                            }
                        },
                        "foregroundColor" => {
                            if v.is_string() {
                                let c = super::helper_str_to_color(v.as_str().unwrap().to_owned());
                                if c.is_ok() {
                                    board.foreground_color = c.unwrap();
                                }
                            }
                        },
                        "size" => {
                            if v.is_object() {
                                let s = v.as_object().unwrap();
                                for (k, v) in s.iter() {
                                    match k.as_ref() {
                                        "width" => {
                                            if v.is_u64() {
                                                board.width = v.as_u64().unwrap() as u16;
                                            }
                                        },
                                        "height" => {
                                            if v.is_u64() {
                                                board.height = v.as_u64().unwrap() as u16;
                                            }
                                        },
                                        _ => {}
                                    }
                                }
                            }
                        },
                        "hardware" => {
                            if v.is_array() {
                                let h = v.as_array().unwrap();
                                for part in h.iter() {
                                    if part.is_object() {
                                        let part_e = part.as_object().unwrap();
                                        let ptype = part_e.get("type");
                                        if ptype.is_some() {
                                            let ptype = ptype.unwrap();
                                            if ptype.is_string() {
                                                match ptype.as_str().unwrap() {
                                                    "simpi/led" => {
                                                        let p = Led::from_json(part.clone());
                                                        if p.is_ok() {
                                                            board.hardware.push(Part::Led(p.unwrap()));
                                                        }
                                                    },
                                                    "simpi/button" => {
                                                        let p = Button::from_json(part.clone());
                                                        if p.is_ok() {
                                                            board.hardware.push(Part::Button(p.unwrap()));
                                                        }
                                                    },
                                                    _ => {}
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        },
                        _ => {}
                    }
                }
                if is_valid {
                    Ok(board)
                } else {
                    Err(Error::new(ErrorKind::InvalidData, "Input data is invalid!"))
                }
            },
            _ => Err(Error::new(ErrorKind::InvalidInput, "Input must be map!"))
        }
    }
    pub fn event_keypress(&mut self, c: char) -> &mut Self {
        for part in self.hardware.iter_mut() {
            match part {
                Part::Button(button) => {
                    if button.hotkey == c.to_string() {
                        button.set(!button.get());
                    }
                },
                _ => {},
            }
        }
        self
    }
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
        let w = if area.width >= self.width { self.width } else { area.width };
        let h = if area.height >= self.height { self.height } else { area.height };
        let board_area = Rect {
            width: w,
            height: h,
            ..area
        };
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
