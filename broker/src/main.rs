/*!main.rs
 * Broker which reads and prints the current state of the GPIO registers,
 * as well as takes user input.
 * 
 * Author: Patrick Goldinger
 * License: GPL 3.0 (see LICENSE file for details)
 */

#![allow(dead_code)]

use std::{
    io::{stdout, Write},
    sync::mpsc,
    thread,
    time::Duration,
};
use utils::{
    gpioregs::{Reg, RegMemory},
    shared_memory::*
};
use tui::Terminal;
use tui::backend::CrosstermBackend;
use tui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Borders, Paragraph, Text, Widget};
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen},
};
use crossterm::terminal::LeaveAlternateScreen;

mod hardware;
use crate::hardware::*;

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

fn reg_to_styled(reg: &Reg, data: &mut Vec<Text>) {
    for i in (0..32).rev() {
        if reg.read_pin(i) > 0 {
            data.push(Text::raw(" "));
            data.push(Text::styled("1", Style::default().fg(Color::White).bg(Color::LightRed).modifier(Modifier::BOLD)));
            data.push(Text::raw(" "));
        } else {
            data.push(Text::raw(" "));
            data.push(Text::styled("0", Style::default().fg(Color::Gray)));
            data.push(Text::raw(" "));
        }
    }
    data.push(Text::raw("\n"));
}

fn get_body_margin(rect: Rect, size: u16) -> u16 {
    if rect.width < size {
        0
    } else {
        100 * ((rect.width - size) / 2) / rect.width
    }
}

pub fn main() -> Result<(), failure::Error> {
    let broker = Broker {
        tick_rate: 50,
    };

    let test = Board::default();

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
            let root_layout = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Percentage(get_body_margin(f.size(), 128)),
                    Constraint::Length(128),
                    Constraint::Percentage(get_body_margin(f.size(), 128)),
                ].as_ref())
                .split(f.size());
            let body_layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(3),
                    Constraint::Length(9),
                    Constraint::Length(3),
                ].as_ref())
                .split(root_layout[1]);
            Block::default()
                .title(" SimPi Broker ")
                .borders(Borders::ALL)
                .render(&mut f, body_layout[0]);
            
            let gpioregs_layout = Layout::default()
                .direction(Direction::Horizontal)
                .margin(1)
                .constraints([
                    Constraint::Length(10),
                    Constraint::Min(1),
                ].as_ref())
                .split(body_layout[1]);
            Block::default()
                .title(" GPIO Registers ")
                .borders(Borders::ALL)
                .render(&mut f, body_layout[1]);
            let gpioregs_names = [
                Text::raw("\n"),
                Text::raw("INPUT\n"),
                Text::raw("OUTPUT\n"),
                Text::raw("CONFIG\n"),
                Text::raw("INTEN\n"),
                Text::raw("INT0\n"),
                Text::raw("INT1\n"),
            ];
            Paragraph::new(gpioregs_names.iter())
                .block(Block::default())
                .alignment(Alignment::Left)
                .render(&mut f, gpioregs_layout[0]);
            match reg_memory.as_ref() {
                Ok(v) => {
                    let mut data = vec![
                        Text::styled("31 30 29 28 27 26 25 24 23 22 21 20 19 18 17 16 15 14 13 12 11 10 09 08 07 06 05 04 03 02 01 00 \n", Style::default().fg(Color::DarkGray))
                    ];
                    let reg_memory = v.rlock::<RegMemory>(GLOBAL_LOCK_ID).unwrap();
                    for reg in [
                        reg_memory.input,
                        reg_memory.output,
                        reg_memory.config,
                        reg_memory.inten,
                        reg_memory.int0,
                        reg_memory.int1,
                    ].iter() {
                        reg_to_styled(&reg, &mut data);
                    }
                    Paragraph::new(data.iter())
                        .block(Block::default())
                        .alignment(Alignment::Right)
                        .render(&mut f, gpioregs_layout[1]);
                },
                Err(err) => {
                    Paragraph::new([Text::raw(format!("{}", err))].iter())
                        .block(Block::default().borders(Borders::TOP))
                        .alignment(Alignment::Right)
                        .render(&mut f, gpioregs_layout[1]);
                }
            };
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
