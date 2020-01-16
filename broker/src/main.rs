/*!main.rs
 * Broker which reads and prints the current state of the GPIO registers,
 * as well as takes user input.
 * 
 * Author: Patrick Goldinger
 * License: GPL 3.0 (see LICENSE file for details)
 */

#![allow(dead_code)]
//#![allow(unused_imports)]

extern crate iced;

use utils::{
    gpioregs::RegMemory,
    shared_memory::*
};
use iced::{
    button, Align, Application, Background, Button, Color, Column, Command,
    Container, Element, HorizontalAlignment, Length, Row, Settings, Text,
};

const PROJECT_NAME: &str = "SimPi";
const APP_NAME: &str = "SimPi Broker";
const VERSION: &str = "0.1.0";

static GLOBAL_LOCK_ID: usize = 0;

pub fn main() {
    Broker::run(Settings::default());
}

struct Broker {
    //theme: style::Theme,
    reg_memory: Result<SharedMem, SharedMemError>,
    state_info: button::State,
    state_settings: button::State,
    state_pause: button::State,
    state_reset: button::State,
    state_terminate: button::State,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    //ThemeChanged(style::Theme),
    Info,
    Settings,
    Pause,
    Reset,
    Terminate,
}



impl Application for Broker {
    type Message = Message;

    fn new() -> (Broker, Command<Message>) {
        (
            Broker {
                //theme: style::Theme::Light,
                reg_memory: utils::init_shared_memory(),
                state_info: button::State::default(),
                state_settings: button::State::default(),
                state_pause: button::State::default(),
                state_reset: button::State::default(),
                state_terminate: button::State::default(),
            },
            Command::none()
        )
    }

    fn title(&self) -> String {
        String::from(APP_NAME)
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            //Message::ThemeChanged(theme) => self.theme = theme,
            Message::Terminate => {  },
            _ => {}
        }

        Command::none()
    }

    fn view(&mut self) -> Element<Message> {
        enum ButtonColor {
            Red,
            Grey,
        }
        let button = |state, label, button_color| {
            Button::new(state, Text::new(label))
                .background(Background::Color(match button_color {
                    ButtonColor::Red => Color {r:1.0,g:0.0,b:0.0,a:1.0},
                    ButtonColor::Grey => Color {r:0.7,g:0.7,b:0.7,a:1.0}
                }))
                .border_radius(4)
                .padding(8)
        };
        let row_header_quickpanel = Row::new()
            .spacing(8)
            .push(
                button(&mut self.state_info, "i", ButtonColor::Grey)
                    .on_press(Message::Info)
            )
            .push(
                button(&mut self.state_settings, "s", ButtonColor::Grey)
                    .on_press(Message::Settings)
            )
            .push(
                button(&mut self.state_pause, "p", ButtonColor::Grey)
                    .on_press(Message::Pause)
            )
            .push(
                button(&mut self.state_reset, "r", ButtonColor::Red)
                    .on_press(Message::Reset)
            )
            .push(
                button(&mut self.state_terminate, "t", ButtonColor::Red)
                    .on_press(Message::Terminate)
            );
        let row_header = Row::new()
            .padding(8)
            .spacing(400)
            .push(
                Text::new(PROJECT_NAME)
                    .size(32)
            )
            .push(row_header_quickpanel);
        
        let row_gpioregs = match &self.reg_memory {
            Ok(v) => {
                let gpioregs = v.rlock::<RegMemory>(GLOBAL_LOCK_ID).unwrap();
                Row::new()
                    .push(
                        Text::new(gpioregs.output.read_to_str())
                            .size(16)
                    )
            },
            Err(err) => {
                Row::new()
                    .push(
                        Text::new(format!("{}", err))
                            .size(16)
                    )
            }
        };

        let row_footer = Row::new()
            .push(
                Text::new(VERSION)
                    .size(16)
            );
        
        Container::new(
            Column::new()
                .width(Length::Fill)
                .height(Length::Fill)
                .push(row_header)
                .push(row_gpioregs)
                .push(row_footer)
        )
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}
