/*!lib.rs
 * Main library file which declares functions for extern use in C programs.
 */

#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_variables)]

mod gpioregs;
mod lsim;

pub extern fn wiringPiSetupGpio() -> i32 {
    return 0;
}

pub extern fn pinMode(pin: i32, pud: i32) {
    //
}

pub extern fn digitalWrite(pin: i32, value: i32) {
    //
}

pub extern fn wiringPiISR(pin: i32, mode: i32, function: extern "C" fn()) {
    //
}

pub extern fn delay(howLong: u32) {
    //
}

pub extern fn delayMicroseconds(howLong: u32) {
    //
}

pub extern fn millis() -> u32 {
    return 0;
}

pub extern fn micros() -> u32 {
    return 0;
}
