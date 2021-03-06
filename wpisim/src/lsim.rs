/*!lsim.rs
 * Module File for the core of the simulating client.
 * 
 * Author: Patrick Goldinger
 * License: GPL 3.0 (see LICENSE file for details)
 */

use std::{thread, time, time::Duration};
use std::sync::{Arc, Mutex};
use utils::{
    gpioregs::*,
    log,
    shared_memory::*,
    ShMem,
};

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
    pub reg_memory: Arc<Mutex<Result<ShMem, SharedMemError>>>,
    pub start_time_us: time::Instant,
    pub isr_routines: Arc<Mutex<[Option<extern "C" fn()>; 32]>>,
    pub is_thread_valid: bool,
}
impl LSimCore {
    pub fn new() -> LSimCore {
        log::init("wpisim");
        return LSimCore {
            reg_memory: Arc::new(Mutex::new(utils::init_shared_memory())),
            start_time_us: time::Instant::now(),
            isr_routines: Arc::new(Mutex::new([None; 32])),
            is_thread_valid: false,
        }
    }

    pub fn setup(&mut self) -> i32 {
        log::info("Init wpisim module...");
        self.start_time_us = time::Instant::now();
        let reg_memory = Arc::clone(&self.reg_memory);
        let isr_routines = Arc::clone(&self.isr_routines);
        thread::spawn(move || {
            let mut old_input = Reg::new();
            loop {
                thread::sleep(Duration::from_millis(50));
                let reg_memory = reg_memory.lock().unwrap();
                let reg_memory = ShMem::rlock(&reg_memory);
                let isr_routines = isr_routines.lock().unwrap();
                for i in MIN_PIN_NUM..=MAX_PIN_NUM {
                    if reg_memory.inten.read_pin(i) == 1 {
                        let v_int0 = reg_memory.int0.read_pin(i) == 1;
                        let v_int1 = reg_memory.int1.read_pin(i) == 1;
                        let v_inp_old = old_input.read_pin(i) == 1;
                        let v_inp_new = reg_memory.input.read_pin(i) == 1;
                        // rising edge
                        if v_int1 && v_int0
                            && !v_inp_old
                            && v_inp_new
                            && isr_routines[i as usize].is_some() {
                            isr_routines[i as usize].unwrap()();
                        }
                        // falling edge
                        else if v_int1 && !v_int0
                            && v_inp_old
                            && !v_inp_new
                            && isr_routines[i as usize].is_some() {
                            isr_routines[i as usize].unwrap()();
                        }
                        // logical change
                        else if !v_int1 && v_int0
                            && (v_inp_old
                            ^ v_inp_new)
                            && isr_routines[i as usize].is_some() {
                            isr_routines[i as usize].unwrap()();
                        }
                    }
                }
                old_input = reg_memory.input.clone();
            }
        });
        return 0;
    }

    pub fn pin_mode(&mut self, pin: u8, pud: u8) {
        let mut reg_memory = self.reg_memory.lock().unwrap();
        let mut reg_memory = ShMem::wlock(&mut reg_memory);
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
        let mut reg_memory = self.reg_memory.lock().unwrap();
        let mut reg_memory = ShMem::wlock(&mut reg_memory);
        if pin >= MIN_PIN_NUM && pin <= MAX_PIN_NUM {
            reg_memory.output.write_pin(pin, val);
        }
    }

    pub fn read_pin(&self, pin: u8) -> u8 {
        let reg_memory = self.reg_memory.lock().unwrap();
        let reg_memory = ShMem::rlock(&reg_memory);
        if pin >= MIN_PIN_NUM && pin <= MAX_PIN_NUM {
            return reg_memory.input.read_pin(pin);
        } else {
            return 0xFF;
        }
    }

    pub fn define_isr_routine(
        &mut self, pin: u8, mode: u8, isr: extern "C" fn()
    ) -> u8 {
        let mut reg_memory = self.reg_memory.lock().unwrap();
        let mut reg_memory = ShMem::wlock(&mut reg_memory);
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
}

#[cfg(test)]
mod tests {
    //use super::*;
}
