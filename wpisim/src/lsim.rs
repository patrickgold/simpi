/*!lsim.rs
 * Module File for the core of the simulating client.
 * 
 * Author: Patrick Goldinger
 * License: GPL 3.0 (see LICENSE file for details)
 */

extern crate websocket;

use std::{thread, time};
use crate::gpioregs;
use websocket::sync::Server;
use websocket::OwnedMessage;

const INPUT: u8 =               0;
const OUTPUT: u8 =              1;
const PWM_OUTPUT: u8 =          2;
const LOW: u8 =                 0;
const HIGH: u8 =                1;
const INT_EDGE_SETUP: u8 =      0;
const INT_EDGE_FALLING: u8 =    1;
const INT_EDGE_RISING: u8 =     2;
const INT_EDGE_BOTH: u8 =       3;
const MIN_PIN_NUM: u8 =         2;
const MAX_PIN_NUM: u8 =         27;

pub struct LSimCore {
    pub reg_memory: gpioregs::RegMemory,
    pub start_time_us: time::Instant,
    pub isr_routines: [Option<extern "C" fn()>; 32],
    // pub sync_thread_handle: Option<thread::JoinHandle>,
    pub is_thread_valid: bool,
}

impl LSimCore {
    pub fn new() -> LSimCore {
        return LSimCore {
            reg_memory: gpioregs::RegMemory::new(),
            start_time_us: time::Instant::now(),
            isr_routines: [None; 32],
            // sync_thread_handle: None,
            is_thread_valid: false,
        }
    }

    pub fn setup(&mut self) -> i32 {
        self.start_time_us = time::Instant::now();
        thread::spawn(move || {
            let server = Server::bind("127.0.0.1:32001").unwrap();
            for request in server.filter_map(Result::ok) {
                thread::spawn(move || {
                    if !request.protocols().contains(&"rust-websocket".to_string()) {
                        request.reject().unwrap();
                        return;
                    }
        
                    let mut client = request.use_protocol("rust-websocket").accept().unwrap();
        
                    let ip = client.peer_addr().unwrap();
        
                    println!("Connection from {}", ip);
        
                    let message = OwnedMessage::Text("Hello".to_string());
                    client.send_message(&message).unwrap();
        
                    let (mut receiver, mut sender) = client.split().unwrap();
        
                    for message in receiver.incoming_messages() {
                        let message = message.unwrap();
        
                        match message {
                            OwnedMessage::Close(_) => {
                                let message = OwnedMessage::Close(None);
                                sender.send_message(&message).unwrap();
                                println!("Client {} disconnected", ip);
                                return;
                            }
                            OwnedMessage::Ping(ping) => {
                                let message = OwnedMessage::Pong(ping);
                                sender.send_message(&message).unwrap();
                            }
                            _ => sender.send_message(&message).unwrap(),
                        }
                    }
                });
            }
        });
        return 0;
    }

    pub fn pin_mode(&mut self, pin: u8, pud: u8) {
        if pin >= MIN_PIN_NUM && pin <= MAX_PIN_NUM {
            if pud == INPUT || pud == OUTPUT {
                let mode = if pud == INPUT { 1 } else { 0 };
                self.reg_memory.config.write_pin(pin, mode);
            } else if pud == PWM_OUTPUT {
                self.reg_memory.config.write_pin(pin, 0);
                // process PWM here!!
            }
        }
    }

    pub fn write_pin(&mut self, pin: u8, val: u8) {
        if pin >= MIN_PIN_NUM && pin <= MAX_PIN_NUM {
            self.reg_memory.output.write_pin(pin, val);
        }
    }

    pub fn read_pin(&self, pin: u8) -> u8 {
        if pin >= MIN_PIN_NUM && pin <= MAX_PIN_NUM {
            return self.reg_memory.input.read_pin(pin);
        } else {
            return 0xFF;
        }
    }

    pub fn define_isr_routine(
        &mut self, pin: u8, mode: u8, isr: extern "C" fn()
    ) -> u8 {
        if pin >= MIN_PIN_NUM && pin <= MAX_PIN_NUM {
            let v_int0 = if mode == INT_EDGE_RISING || mode == INT_EDGE_BOTH { 1 } else { 0 };
            let v_int1 = if mode == INT_EDGE_RISING || mode == INT_EDGE_FALLING { 1 } else { 0 };
            self.reg_memory.int0.write_pin(pin, v_int0);
            self.reg_memory.int1.write_pin(pin, v_int1);
            self.reg_memory.inten.write_pin(pin, 1);
            self.isr_routines[pin as usize] = Some(isr);
            return 0;
        } else {
            return 0xFF;
        }
    }

    pub fn delay_ms(&self, duration: u64) {
        let duration = time::Duration::from_millis(duration);
        thread::sleep(duration);
    }

    pub fn delay_us(&self, duration: u64) {
        let duration = time::Duration::from_micros(duration);
        thread::sleep(duration);
    }

    pub fn get_uptime_ms(&self) -> u64 {
        return (time::Instant::now() - self.start_time_us).as_millis() as u64;
    }

    pub fn get_uptime_us(&self) -> u64 {
        return (time::Instant::now() - self.start_time_us).as_micros() as u64;
    }
}

pub struct RetDataSingle {
    status: String,
    key: String,
    value: String,
}

pub struct RetData {
    operation: String,
    data: Vec<RetDataSingle>,
}

#[cfg(test)]
mod tests {
    use super::*;

}
