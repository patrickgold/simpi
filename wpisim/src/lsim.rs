/*!lsim.rs
 * Module File for the core of the simulating client.
 * 
 * Author: Patrick Goldinger
 * License: GPL 3.0 (see LICENSE file for details)
 */

extern crate tungstenite;

use std::{thread, time};
use std::sync::{Mutex, Arc};
use std::net::TcpListener;
use tungstenite::{server::accept, Message};
use utils::log;
use utils::gpioregs::*;
use utils::shared_memory::*;

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

static GLOBAL_LOCK_ID: usize = 0;

pub struct LSimCore {
    pub reg_memory: Result<SharedMem, SharedMemError>,
    pub start_time_us: time::Instant,
    pub isr_routines: Arc<Mutex<[Option<extern "C" fn()>; 32]>>,
    //pub sync_thread_handle: Option<thread::JoinHandle<_>>,
    pub is_thread_valid: bool,
}
unsafe impl Send for LSimCore {}
impl LSimCore {
    pub fn new() -> LSimCore {
        return LSimCore {
            reg_memory: utils::init_shared_memory(),
            start_time_us: time::Instant::now(),
            isr_routines: Arc::new(Mutex::new([None; 32])),
            //sync_thread_handle: None,
            is_thread_valid: false,
        }
    }

    pub fn setup(&mut self) -> i32 {
        log::info("Init wpisim module...");
        self.start_time_us = time::Instant::now();
        // let reg_memory = Arc::clone(&self.reg_memory);
        // let isr_routines = Arc::clone(&self.isr_routines);
        // thread::spawn(move || {
        //     let server = TcpListener::bind("127.0.0.1:32001").unwrap();
        //     for stream in server.incoming() {
        //         let reg_memory = Arc::clone(&reg_memory);
        //         let isr_routines = Arc::clone(&isr_routines);
        //         thread::spawn(move || {
        //             let stream = stream.unwrap();
        //             let ip = stream.peer_addr().unwrap().to_string();
        //             log::info(format!("Connection from {}", ip).as_str());
        //             let mut websocket = accept(stream).unwrap();
        //             loop {
        //                 let msg = websocket.read_message().unwrap();
        //                 match msg {
        //                     Message::Close(_) => {
        //                         let message = Message::Close(None);
        //                         websocket.write_message(message).unwrap_or_default();
        //                         log::info(format!("Client {} disconnected", ip).as_str());
        //                         return;
        //                     }
        //                     Message::Ping(ping) => {
        //                         let message = Message::Pong(ping);
        //                         websocket.write_message(message).unwrap_or_default();
        //                     }
        //                     Message::Text(request) => {
        //                         let mut reg_memory = reg_memory.lock().unwrap();
        //                         let request = LSimCore::parse_request_str(request);
        //                         let mut response = RegTransferData::new();
        //                         response.status = "FAIL".to_owned();
        //                         match request {
        //                             Result::Ok(req_data) => {
        //                                 response.command = req_data.command.clone();
        //                                 response.key = req_data.key.clone();
        //                                 if req_data.command == "getreg".to_owned() || req_data.command == "setreg".to_owned() {
        //                                     let reg = reg_memory.get(req_data.key.clone());
        //                                     match reg {
        //                                         Result::Ok(reg) => {
        //                                             response.value = req_data.value.clone();
        //                                             if req_data.command == "getreg".to_owned() {
        //                                                 response.value = reg.read_to_str();
        //                                                 response.status = "SUCC".to_owned();
        //                                             } else {
        //                                                 let old_input = reg.clone();
        //                                                 reg.write_from_str(req_data.value);
        //                                                 if req_data.key.to_ascii_lowercase() == "input".to_owned() {
        //                                                     let isr_routines = isr_routines.lock().unwrap();
        //                                                     for i in MIN_PIN_NUM..=MAX_PIN_NUM {
        //                                                         if reg_memory.inten.read_pin(i) == 1 {
        //                                                             let v_int0 = reg_memory.int0.read_pin(i) == 1;
        //                                                             let v_int1 = reg_memory.int1.read_pin(i) == 1;
        //                                                             let v_inp_old = old_input.read_pin(i) == 1;
        //                                                             let v_inp_new = reg_memory.input.read_pin(i) == 1;
        //                                                             // rising edge
        //                                                             if v_int1 && v_int0
        //                                                                 && !v_inp_old
        //                                                                 && v_inp_new
        //                                                                 && isr_routines[i as usize].is_some() {
        //                                                                 isr_routines[i as usize].unwrap()();
        //                                                             }
        //                                                             // falling edge
        //                                                             else if v_int1 && !v_int0
        //                                                                 && v_inp_old
        //                                                                 && !v_inp_new
        //                                                                 && isr_routines[i as usize].is_some() {
        //                                                                 isr_routines[i as usize].unwrap()();
        //                                                             }
        //                                                             // logical change
        //                                                             else if !v_int1 && v_int0
        //                                                                 && (v_inp_old
        //                                                                 ^ v_inp_new)
        //                                                                 && isr_routines[i as usize].is_some() {
        //                                                                 isr_routines[i as usize].unwrap()();
        //                                                             }
        //                                                         }
        //                                                     }
        //                                                 }
        //                                                 response.status = "SUCC".to_owned();
        //                                             }
        //                                         },
        //                                         Result::Err(err) => {
        //                                             //log::error(err.clone().as_str());
        //                                             response.value = err;
        //                                         }
        //                                     }
        //                                 } else if req_data.command == "reset".to_owned() {
        //                                     log::info("Resetted registers.");
        //                                     reg_memory.reset();
        //                                     response.value = "Reset done".to_owned();
        //                                     response.status = "SUCC".to_owned();
        //                                 } else if req_data.command == "terminate".to_owned() {
        //                                     log::info("Simpi Broker terminated.");
        //                                     //std::process::exit(0);
        //                                 } else {
        //                                     response.value = "Unknown action".to_owned();
        //                                 }
        //                             },
        //                             Result::Err(err) => {
        //                                 //log::error(err.clone().as_str());
        //                                 response.value = err;
        //                             }
        //                         }
        //                         let resp = Message::Text(LSimCore::pack_request_str(response));
        //                         websocket.write_message(resp).unwrap_or_default();
        //                     }
        //                     _ => {
        //                         log::warning("Unknwon Message type received.");
        //                     }
        //                 }
        //             }
        //         });
        //     }
        // });
        return 0;
    }

    fn parse_request_str(req: String) -> Result<RegTransferData, String> {
        if !req.is_ascii() {
            return Err("Given request contains non-ASCII characters!!".to_owned());
        }
        let mut ret = RegTransferData::new();
        let mut i = 0;
        for req_p in req.split("/") {
            if i == 0 {
                let mut j = 0;
                for req_pp in req_p.split(":") {
                    if j == 0 {
                        ret.command = req_pp.to_owned();
                    } else if j == 1 {
                        ret.status = req_pp.to_owned();
                    } else {
                        return Err("Invalid request syntax!!".to_owned());
                    }
                    j += 1;
                }
            } else if i == 1 {
                let mut j = 0;
                for req_pp in req_p.split("=") {
                    if j == 0 {
                        ret.key = req_pp.to_owned();
                    } else if j == 1 {
                        ret.value = req_pp.to_owned();
                    } else {
                        return Err("Invalid request syntax!!".to_owned());
                    }
                    j += 1;
                }
            } else {
                return Err("Invalid request syntax!!".to_owned());
            }
            i += 1;
        }
        return Ok(ret);
    }
    fn pack_request_str(data: RegTransferData) -> String {
        let mut ret: String = "".to_owned();
        ret += ">";
        ret += data.command.as_str();
        if data.status.len() > 0 {
            ret += ":";
            ret += data.status.as_str();
        }
        ret += "/";
        ret += data.key.as_str();
        if data.value.len() > 0 {
            ret += "=";
            ret += data.value.as_str();
        }
        return ret;
    }

    pub fn pin_mode(&mut self, pin: u8, pud: u8) {
        let mut reg_memory = self.reg_memory.as_mut().unwrap()
            .wlock::<RegMemory>(GLOBAL_LOCK_ID).unwrap();
        if pin >= MIN_PIN_NUM && pin <= MAX_PIN_NUM {
            if pud == INPUT || pud == OUTPUT {
                let mode = if pud == INPUT { 1 } else { 0 };
                reg_memory.config.write_pin(pin, mode);
            } else if pud == PWM_OUTPUT {
                reg_memory.config.write_pin(pin, 0);
                // process PWM here!!
            }
        }
    }

    pub fn write_pin(&mut self, pin: u8, val: u8) {
        let mut reg_memory = self.reg_memory.as_mut().unwrap()
            .wlock::<RegMemory>(GLOBAL_LOCK_ID).unwrap();
        if pin >= MIN_PIN_NUM && pin <= MAX_PIN_NUM {
            reg_memory.output.write_pin(pin, val);
        }
    }

    pub fn read_pin(&self, pin: u8) -> u8 {
        let reg_memory = self.reg_memory.as_ref().unwrap()
            .rlock::<RegMemory>(GLOBAL_LOCK_ID).unwrap();
        if pin >= MIN_PIN_NUM && pin <= MAX_PIN_NUM {
            return reg_memory.input.read_pin(pin);
        } else {
            return 0xFF;
        }
    }

    pub fn define_isr_routine(
        &mut self, pin: u8, mode: u8, isr: extern "C" fn()
    ) -> u8 {
        let mut reg_memory = self.reg_memory.as_mut().unwrap()
            .wlock::<RegMemory>(GLOBAL_LOCK_ID).unwrap();
        let mut isr_routines = self.isr_routines.lock().unwrap();
        if pin >= MIN_PIN_NUM && pin <= MAX_PIN_NUM {
            let v_int0 = if mode == INT_EDGE_RISING || mode == INT_EDGE_BOTH { 1 } else { 0 };
            let v_int1 = if mode == INT_EDGE_RISING || mode == INT_EDGE_FALLING { 1 } else { 0 };
            reg_memory.int0.write_pin(pin, v_int0);
            reg_memory.int1.write_pin(pin, v_int1);
            reg_memory.inten.write_pin(pin, 1);
            isr_routines[pin as usize] = Some(isr);
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

    const LL_INFO: u8 = 1;
    const LL_WARNING: u8 = 2;
    const LL_ERROR: u8 = 3;
}

pub struct RegTransferData {
    command: String,
    status: String,
    key: String,
    value: String,
}
impl RegTransferData {
    pub fn new() -> RegTransferData {
        return RegTransferData {
            command: "".to_owned(),
            status: "".to_owned(),
            key: "".to_owned(),
            value: "".to_owned(),
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn LSimCore__parse_request_str() {
        let req = "command:status/key=value".to_owned();
        let req_parsed = LSimCore::parse_request_str(req).unwrap();
        assert_eq!(req_parsed.command, "command");
        assert_eq!(req_parsed.status, "status");
        assert_eq!(req_parsed.key, "key");
        assert_eq!(req_parsed.value, "value");
    }

    #[test]
    fn LSimCore__pack_request_str() {
        let mut req = RegTransferData::new();
        req.command = "command".to_owned();
        req.status = "status".to_owned();
        req.key = "key".to_owned();
        req.value = "value".to_owned();
        let req_packed = LSimCore::pack_request_str(req);
        assert_eq!(req_packed, ">command:status/key=value".to_owned());
    }
}
