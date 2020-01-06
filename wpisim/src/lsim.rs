/*!lsim.rs
 * Module File for the core of the simulating client.
 * 
 * Author: Patrick Goldinger
 * License: GPL 3.0 (see LICENSE file for details)
 */

use std::thread;
use crate::gpioregs;

const MIN_PIN_NUM: u8 = 2;
const MAX_PIN_NUM: u8 = 27;

pub struct LSimCore {
    pub reg_memory: gpioregs::RegMemory,
    pub start_time_us: u32,
    pub isr_functions: [Option<extern "C" fn()>; 32],
    pub sync_thread_handle: Option<thread::JoinHandle>,
    pub is_thread_valid: bool,
}

impl LSimCore {
    pub fn new() -> LSimCore {
        return LSimCore {
            reg_memory: gpioregs::RegMemory::new(),
            start_time_us: 0,
            isr_functions: [None, 32],
            sync_thread_handle: None,
            is_thread_valid: false,
        }
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
