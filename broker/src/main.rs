/*!main.rs
 * Broker which reads and prints the current state of the GPIO registers,
 * as well as takes user input.
 * 
 * Author: Patrick Goldinger
 * License: GPL 3.0 (see LICENSE file for details)
 */

#![allow(dead_code)]
#![allow(unused_imports)]

use std::{
    io::{stdout, Write},
    sync::mpsc,
    thread,
    time::Duration,
};
use utils::{
    gpioregs::RegMemory,
    shared_memory::*
};
use tui::Terminal;
use tui::backend::CrosstermBackend;
use tui::layout::{Alignment, Constraint, Direction, Layout};
use tui::style::{Color, Style};
use tui::widgets::{Block, Borders, Paragraph, Text, Widget};
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen},
};
use crossterm::terminal::LeaveAlternateScreen;

const PROJECT_NAME: &str = "SimPi";
const APP_NAME: &str = "SimPi Broker";
const VERSION: &str = "0.1.0";

static GLOBAL_LOCK_ID: usize = 0;

enum BrokerEvent<I> {
    Input(I),
    Tick,
}

struct Broker {
    tick_rate: u64,
}

fn reg_to_bin_str(reg: u32) -> String {
    let mut ret = String::new();
    for i in 0..32 {
        ret = (if (reg & (1 << i)) > 0 { " 1 " } else { " 0 " }).to_owned() + &ret;
    }
    return ret;
}

pub fn main() -> Result<(), failure::Error> {
    let broker = Broker {
        tick_rate: 50,
    };

    enable_raw_mode()?;

    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);

    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;

    // Setup input handling
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        loop {
            // poll for tick rate duration, if no events, sent tick event.
            if event::poll(Duration::from_millis(broker.tick_rate)).unwrap() {
                if let Event::Key(key) = event::read().unwrap() {
                    tx.send(BrokerEvent::Input(key)).unwrap();
                }
            }
            tx.send(BrokerEvent::Tick).unwrap();
        }
    });

    terminal.clear()?;

    let reg_memory = utils::init_shared_memory();

    loop {
        terminal.draw(|mut f| {
            let body_layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints(
                    [
                        Constraint::Length(3),
                        Constraint::Length(9),
                        Constraint::Length(3),
                    ].as_ref()
                )
                .split(f.size());
            Block::default()
                .title(" SimPi Broker ")
                .borders(Borders::ALL)
                .render(&mut f, body_layout[0]);
            
            let gpioregs_layout = Layout::default()
                .direction(Direction::Horizontal)
                .margin(1)
                .constraints(
                    [
                        Constraint::Length(10),
                        Constraint::Min(1),
                    ].as_ref()
                )
                .split(body_layout[1]);
            let gpioregs_names = [
                Text::raw("\n"),
                Text::raw("INPUT\n"),
                Text::raw("OUTPUT\n"),
                Text::raw("CONFIG\n"),
                Text::raw("INTEN\n"),
                Text::raw("INT0\n"),
                Text::raw("INT1\n"),
            ];
            let gpioregs_data = match reg_memory.as_ref() {
                Ok(v) => {
                    let reg_memory = v.rlock::<RegMemory>(GLOBAL_LOCK_ID).unwrap();
                    [
                        Text::styled(
                            "31 30 29 28 27 26 25 24 23 22 21 20 19 18 17 16 15 14 13 12 11 10 09 08 07 06 05 04 03 02 01 00 \n",
                            Style::default().fg(Color::DarkGray)
                        ),
                        Text::raw(reg_to_bin_str(reg_memory.input.read()) + "\n"),
                        Text::raw(reg_to_bin_str(reg_memory.output.read()) + "\n"),
                        Text::raw(reg_to_bin_str(reg_memory.config.read()) + "\n"),
                        Text::raw(reg_to_bin_str(reg_memory.inten.read()) + "\n"),
                        Text::raw(reg_to_bin_str(reg_memory.int0.read()) + "\n"),
                        Text::raw(reg_to_bin_str(reg_memory.int1.read()) + "\n"),
                    ]
                },
                Err(err) => {
                    [
                        Text::raw(format!("{}", err)),
                        Text::raw(""),
                        Text::raw(""),
                        Text::raw(""),
                        Text::raw(""),
                        Text::raw(""),
                        Text::raw(""),
                    ]
                }
            };
            let gpioregs_block = Block::default()
                .borders(Borders::NONE);
            Block::default()
                .title(" GPIO Registers ")
                .borders(Borders::ALL)
                .render(&mut f, body_layout[1]);
            Paragraph::new(gpioregs_names.iter())
                .block(gpioregs_block.clone())
                .alignment(Alignment::Left)
                .render(&mut f, gpioregs_layout[0]);
            Paragraph::new(gpioregs_data.iter())
                .block(gpioregs_block.clone())
                .alignment(Alignment::Right)
                .render(&mut f, gpioregs_layout[1]);
        }).unwrap();
        match rx.recv()? {
            BrokerEvent::Input(event) => {
                match event.code {
                    KeyCode::Char('q') => {
                        disable_raw_mode()?;
                        execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
                        terminal.show_cursor()?;
                        break;
                    },
                    _ => {}
                }
            },
            BrokerEvent::Tick => {},
            //_ => {}
        }
    }
    Ok(())
}
