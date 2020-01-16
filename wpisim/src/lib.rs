/*!lib.rs
 * Main library file which declares functions for extern use in C programs.
 */

#![allow(non_snake_case)]
#![allow(dead_code)]

#[macro_use]
extern crate lazy_static;

mod lsim;

use utils::*;
use std::sync::Mutex;

lazy_static! {
    static ref CORE: Mutex<lsim::LSimCore> = Mutex::new(lsim::LSimCore::new());
}

#[no_mangle]
pub extern "C" fn wiringPiSetupGpio() -> i32 {
    let mut core = CORE.lock().unwrap();
    core.setup();
    return 0;
}

#[no_mangle]
pub extern "C" fn pinMode(pin: i32, pud: i32) {
    let mut core = CORE.lock().unwrap();
    core.pin_mode(pin as u8, pud as u8);
}

#[no_mangle]
pub extern "C" fn digitalWrite(pin: i32, value: i32) {
    let mut core = CORE.lock().unwrap();
    core.write_pin(pin as u8, value as u8);
}

#[no_mangle]
pub extern "C" fn digitalRead(pin: i32) -> i32 {
    let core = CORE.lock().unwrap();
    return core.read_pin(pin as u8) as i32;
}

#[no_mangle]
pub extern "C" fn wiringPiISR(pin: i32, mode: i32, function: extern "C" fn()) {
    let mut core = CORE.lock().unwrap();
    core.define_isr_routine(pin as u8, mode as u8, function);
}

#[no_mangle]
pub extern "C" fn delay(howLong: u32) {
    let core = CORE.lock().unwrap();
    core.delay_ms(howLong as u64);
}

#[no_mangle]
pub extern "C" fn delayMicroseconds(howLong: u32) {
    let core = CORE.lock().unwrap();
    core.delay_us(howLong as u64);
}

#[no_mangle]
pub extern "C" fn millis() -> u32 {
    let core = CORE.lock().unwrap();
    return core.get_uptime_ms() as u32;
}

#[no_mangle]
pub extern "C" fn micros() -> u32 {
    let core = CORE.lock().unwrap();
    return core.get_uptime_us() as u32;
}
