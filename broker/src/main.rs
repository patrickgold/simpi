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
    io::{Error, stdout, Write},
    sync::mpsc,
    thread,
    time::Duration,
};
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

mod app;
mod hardware;

const PROJECT_NAME: &str = "SimPi";
const APP_NAME: &str = "SimPi Broker";
const VERSION: &str = crate_version!();

enum BrokerEvent<I> {
    Input(I),
    Tick,
}
enum BrokerPage {
    GpioRegs,
    Help,
    BoardManager,
    Preferences
}

struct Broker {
    active_page: BrokerPage,
    bm: app::BoardManager,
    is_paused: bool,
    reg_memory: Result<ShMem, SharedMemError>,
    reg_memory_snapshot: RegMemory,
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

fn load_board(broker: &mut Broker, board_file: &str) -> Result<(), Error> {
    let board = hardware::Board::from_file(board_file);
    if board.is_ok() {
        broker.bm.boards.push(board.unwrap());
        Ok(())
    } else {
        Err(board.err().unwrap())
    }
}

pub fn main() -> Result<(), failure::Error> {
    let matches = App::new("SimPi Broker")
        .version(VERSION)
        .author("Patrick Goldinger <@>")
        .about("Simulate the Raspberry Pi GPIO on a PC")
        .arg(Arg::with_name("board")
            .short("b")
            .long("board")
            .value_name("BOARD")
            .help("Space-separated list of boards to load")
            .min_values(1),
        )
        .arg(Arg::with_name("debug")
            .short("d")
            .help("Turn debugging information on [NYI]"),
        )
        .get_matches();
    
    let mut broker = Broker {
        active_page: BrokerPage::GpioRegs,
        bm: app::BoardManager::default(),
        is_paused: false,
        reg_memory: utils::init_shared_memory(),
        reg_memory_snapshot: RegMemory::new(),
        tick_rate: 50,
    };
    
    if matches.is_present("board") {
        let board_files: Vec<_> = matches.values_of("board").unwrap().collect();
        for board in board_files.iter() {
            load_board(&mut broker, board).unwrap_or_default();
        }
    }

    // #region Terminal Setup
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
    // #endregion Terminal Setup

    loop {
        terminal.draw(|mut f| {
            // #region Application Layout
            let root_layout = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Percentage(get_body_margin(f.size(), 128)),
                    Constraint::Length(128),
                    Constraint::Percentage(get_body_margin(f.size(), 128)),
                ].as_ref())
                .split(f.size());
            let body_layout = match broker.active_page {
                BrokerPage::GpioRegs => {
                    Layout::default()
                        .direction(Direction::Vertical)
                        .constraints([
                            Constraint::Length(3),  // SimPi Header
                            Constraint::Length(9),  // GPIO Regs
                            Constraint::Min(1),     // Board
                        ].as_ref())
                        .split(root_layout[1])
                },
                _ => {
                    Layout::default()
                        .direction(Direction::Vertical)
                        .constraints([
                            Constraint::Length(3),  // SimPi Header
                            Constraint::Min(1),     // Page Content
                        ].as_ref())
                        .split(root_layout[1])
                }
            };
            // #endregion Application Layout
            
            // #region Application Header UI
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
                Text::styled(
                    if let BrokerPage::Help = broker.active_page { "Close Help" } else { "Help" },
                header_cmd_style),
                Text::raw(" "),
                Text::styled("F2", header_key_style),
                Text::styled(
                    if let BrokerPage::BoardManager = broker.active_page { "Close Board Manager" } else { "Board Manager" },
                header_cmd_style),
                Text::raw(" "),
                Text::styled("F3", header_key_style),
                Text::styled(
                    if let BrokerPage::Preferences = broker.active_page { "Close Preferences" } else { "Preferences" },
                header_cmd_style),
                Text::raw(" "),
                Text::styled("F7", header_key_style),
                Text::styled(
                    if broker.is_paused { "Play " } else { "Pause" },
                header_cmd_style),
                Text::raw(" "),
                Text::styled("F8", header_key_style),
                Text::styled("Reset", header_cmd_style_red),
                Text::raw(" "),
                Text::styled("F9", header_key_style),
                Text::styled("Quit", header_cmd_style_red),
            ];
            Paragraph::new(header_text.iter())
                .block(Block::default()
                    .title(format!("{}{}{}", " SimPi Broker ", VERSION, " ").as_ref())
                    .borders(Borders::ALL)
                )
                .alignment(Alignment::Right)
                .render(&mut f, body_layout[0]);
            // #endregion Application Header UI
            
            // #region Application Body UI
            match broker.active_page {
                BrokerPage::GpioRegs => {
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
                    if broker.reg_memory.is_ok() {
                        let mut data = vec![
                            Text::styled("31 30 29 28 27 26 25 24 23 22 21 20 19 18 17 16 15 14 13 12 11 10 09 08 07 06 05 04 03 02 01 00 \n", Style::default().fg(Color::DarkGray))
                        ];
                        if broker.is_paused {
                            let reg_memory = broker.reg_memory_snapshot;
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
                        } else {
                            let mut reg_memory = ShMem::wlock(&mut broker.reg_memory);
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
                            for board in broker.bm.boards.iter_mut() {
                                board.sync(&mut reg_memory);
                            }
                        }
                        Paragraph::new(data.iter())
                            .block(Block::default())
                            .alignment(Alignment::Right)
                            .render(&mut f, gpioregs_layout[1]);
                        for board in broker.bm.boards.iter_mut() {
                            board.render(&mut f, body_layout[2]);
                        }
                    } else {
                        Paragraph::new([
                            Text::raw(format!(
                                "{}", broker.reg_memory.as_ref().err().unwrap()
                            ))
                        ].iter())
                            .block(Block::default().borders(Borders::TOP))
                            .alignment(Alignment::Right)
                            .render(&mut f, gpioregs_layout[1]);
                    };
                },
                BrokerPage::Help => {
                    // Placeholder
                    Paragraph::new([
                        Text::raw("This help is not very helpful... yet! (Help [NYI])")
                    ].iter())
                        .block(Block::default().borders(Borders::ALL))
                        .render(&mut f, body_layout[1]);
                },
                BrokerPage::BoardManager => {
                    broker.bm.render(&mut f, body_layout[1]);
                },
                BrokerPage::Preferences => {
                    // Placeholder
                    Paragraph::new([
                        Text::raw("Preferences [NYI]")
                    ].iter())
                        .block(Block::default().borders(Borders::ALL))
                        .render(&mut f, body_layout[1]);
                },
            }
            // #endregion Application Body UI
            
        }).unwrap_or_default();

        // #region Event Handling
        match rx.recv()? {
            BrokerEvent::Input(event) => {
                match event.code {
                    KeyCode::F(inp) => {
                        if inp == 1 {
                            if let BrokerPage::Help = broker.active_page {
                                broker.active_page = BrokerPage::GpioRegs;
                            } else {
                                broker.active_page = BrokerPage::Help;
                                broker.bm.set_active(false);
                            }
                        } else if inp == 2 {
                            if let BrokerPage::BoardManager = broker.active_page {
                                broker.active_page = BrokerPage::GpioRegs;
                                broker.bm.set_active(false);
                            } else {
                                broker.active_page = BrokerPage::BoardManager;
                                broker.bm.set_active(true);
                            }
                        } else if inp == 3 {
                            if let BrokerPage::Preferences = broker.active_page {
                                broker.active_page = BrokerPage::GpioRegs;
                            } else {
                                broker.active_page = BrokerPage::Preferences;
                                broker.bm.set_active(false);
                            }
                        } else if inp == 7 {
                            if broker.is_paused {
                                broker.is_paused = false;
                            } else {
                                broker.is_paused = true;
                                let reg_memory = ShMem::rlock(&broker.reg_memory);
                                broker.reg_memory_snapshot = reg_memory.clone();
                            }
                        } else if inp == 8 {
                            let mut reg_memory = ShMem::wlock(&mut broker.reg_memory);
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
                        broker.bm.event_keypress(inp);
                        for board in broker.bm.boards.iter_mut() {
                            board.event_keypress(inp);
                        }
                    },
                    _ => {}
                }
            },
            BrokerEvent::Tick => {},
            //_ => {}
        }
        // #endregion Event Handling
    }

    Ok(())
}
