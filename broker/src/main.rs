/*!main.rs
 * Broker which reads and prints the current state of the GPIO registers,
 * as well as takes user input.
 * 
 * Author: Patrick Goldinger
 * License: GPL 3.0 (see LICENSE file for details)
 */

#![allow(dead_code)]

#[macro_use]
extern crate clap;
extern crate tui;

use clap::{App, Arg};
use std::{
    io::{stdout, Write},
    sync::mpsc,
    thread,
    time::Duration,
};
use std::fs::File;
use std::io::Read;
use utils::{
    gpioregs::{Reg, RegMemory},
    shared_memory::*,
    ShMem
};
use tui::Terminal;
use tui::backend::CrosstermBackend;
use tui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Borders, Paragraph, Text, Widget};
use crossterm::{
    event::{self, Event, KeyCode, EnableMouseCapture, DisableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen},
};
use crossterm::terminal::LeaveAlternateScreen;

mod hardware;

const PROJECT_NAME: &str = "SimPi";
const APP_NAME: &str = "SimPi Broker";
const VERSION: &str = "0.1.0";

static GLOBAL_LOCK_ID: usize = 0;

enum BrokerEvent<I> {
    Input(I),
    Tick,
}

struct Broker {
    boards: Vec<hardware::Board>,
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
    let mut broker = Broker {
        boards: vec![],
        tick_rate: 50,
    };

    let matches = App::new("SimPi Broker")
        .version(crate_version!())
        .author("Patrick Goldinger <@>")
        .about("Simulate the Raspberry Pi GPIO on a PC.")
        .arg(Arg::with_name("board")
            .short("b")
            .long("board")
            .value_name("BOARD")
            .help("Specify board(s) to load on startup")
            .min_values(1),
        )
        .arg(Arg::with_name("debug")
            .short("d")
            .help("Turn debugging information on [NYI]"),
        )
        .get_matches();
    
    if matches.is_present("board") {
        let board_files: Vec<_> = matches.values_of("board").unwrap().collect();
        for board in board_files.iter() {
            let file = File::open(board);
            if file.is_ok() {
                let mut file = file.unwrap();
                let mut data = String::new();
                file.read_to_string(&mut data).unwrap();
                let data = serde_json::from_str(data.as_ref()).unwrap();
                broker.boards.push(hardware::Board::from_json(data).unwrap());
            }
        }
    }

    enable_raw_mode()?;

    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen)?;
    execute!(stdout, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);

    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;

    // Setup input handling
    let (tx, rx) = mpsc::channel();

    let tick_rate = broker.tick_rate;

    thread::spawn(move || {
        loop {
            // poll for tick rate duration, if no events, sent tick event.
            if event::poll(Duration::from_millis(tick_rate)).unwrap() {
                if let Event::Key(key) = event::read().unwrap() {
                    tx.send(BrokerEvent::Input(key)).unwrap_or_default();
                }
            }
            tx.send(BrokerEvent::Tick).unwrap_or_default();
        }
    });

    terminal.clear()?;

    let mut reg_memory = utils::init_shared_memory();

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
                    Constraint::Length(3),  // SimPi Header
                    Constraint::Length(9),  // GPIO Regs
                    Constraint::Length(32), // Board
                    Constraint::Length(3),  // Footer
                ].as_ref())
                .split(root_layout[1]);
            let header_cmd_style = Style::default()
                .fg(Color::Black)
                .bg(Color::White);
            let header_cmd_style_red = Style::default()
                .fg(Color::White)
                .bg(Color::LightRed);
            let header_key_style = Style::default()
                .fg(Color::White)
                .bg(Color::Black);
            let header_text = [
                Text::raw(" "),
                Text::styled("F1", header_key_style),
                Text::styled("Help[NYI]", header_cmd_style),
                Text::raw(" "),
                Text::styled("F2", header_key_style),
                Text::styled("Board Manager[NYI]", header_cmd_style),
                Text::raw(" "),
                Text::styled("F6", header_key_style),
                Text::styled("Preferences[NYI]", header_cmd_style),
                Text::raw(" "),
                Text::styled("F7", header_key_style),
                Text::styled("Pause/Play[NYI]", header_cmd_style),
                Text::raw(" "),
                Text::styled("F8", header_key_style),
                Text::styled("Reset", header_cmd_style_red),
                Text::raw(" "),
                Text::styled("F9", header_key_style),
                Text::styled("Quit", header_cmd_style_red),
            ];
            Paragraph::new(header_text.iter())
                .block(Block::default()
                    .title(format!("{}{}{}", " SimPi Broker ", crate_version!(), " ").as_ref())
                    .borders(Borders::ALL)
                )
                .alignment(Alignment::Right)
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
            match reg_memory.as_mut() {
                Ok(v) => {
                    let mut data = vec![
                        Text::styled("31 30 29 28 27 26 25 24 23 22 21 20 19 18 17 16 15 14 13 12 11 10 09 08 07 06 05 04 03 02 01 00 \n", Style::default().fg(Color::DarkGray))
                    ];
                    let mut reg_memory = v.mem.wlock::<RegMemory>(GLOBAL_LOCK_ID).unwrap();
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
                    for board in broker.boards.iter_mut() {
                        board.sync(&mut reg_memory);
                    }
                    drop(reg_memory);
                    Paragraph::new(data.iter())
                        .block(Block::default())
                        .alignment(Alignment::Right)
                        .render(&mut f, gpioregs_layout[1]);
                    for board in broker.boards.iter_mut() {
                        board.render(&mut f, body_layout[2]);
                    }
                },
                Err(err) => {
                    Paragraph::new([Text::raw(format!("{}", err))].iter())
                        .block(Block::default().borders(Borders::TOP))
                        .alignment(Alignment::Right)
                        .render(&mut f, gpioregs_layout[1]);
                }
            };
        }).unwrap_or_default();
        match rx.recv()? {
            BrokerEvent::Input(event) => {
                match event.code {
                    KeyCode::F(inp) => {
                        if inp == 8 {
                            let mut reg_memory = ShMem::wlock(&mut reg_memory);
                            reg_memory.reset();
                        } else if inp == 9 {
                            disable_raw_mode()?;
                            execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
                            execute!(terminal.backend_mut(), DisableMouseCapture)?;
                            terminal.show_cursor()?;
                            break;
                        }
                    },
                    KeyCode::Char(inp) => {
                        for board in broker.boards.iter_mut() {
                            board.event_keypress(inp);
                        }
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
