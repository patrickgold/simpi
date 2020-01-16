/*!log.rs
 * Module File for logging events of wpisim.
 * 
 * Author: Patrick Goldinger
 * License: GPL 3.0 (see LICENSE file for details)
 */

lazy_static! {
    static ref SHOULD_LOG: bool = std::env::var("WPISIM_LOG").unwrap_or("0".to_owned()) == "1".to_owned();
}

fn _log(level: u8, msg: &str) {
    if *SHOULD_LOG {
        let pre_esc_seq = match level {
            1 => "\x1b[0;90m", // Info: Dark Gray
            2 => "\x1b[0;33m", // Warning: Yellow
            3 => "\x1b[0;31m", // Error: Red
            _ => "\x1b[0m",    // Other level: Default Color
        };
        eprintln!("{}[wpisim] {}{}", pre_esc_seq, msg, "\x1b[0m");
    }
}
pub fn info(msg: &str) {
    _log(1, msg);
}
pub fn warning(msg: &str) {
    _log(2, msg);
}
pub fn error(msg: &str) {
    _log(3, msg);
}
