/*!gpioregs.rs
 * Module File for Raspberry Pi GPIO Registers.
 * 
 * Author: Patrick Goldinger
 * License: GPL 3.0 (see LICENSE file for details)
 */

const MIN_PIN_NUM: u8 = 2;
const MAX_PIN_NUM: u8 = 27;

pub struct RegMemory {
    // INPUT register (seen from programmer's view)
    pub input: u32,
    // OUTPUT register (seen from programmer's view)
    pub output: u32,
    // 1=Input 0=Output (All pins default to input!)
    pub config: u32,
    // 1=Interrupt 0=No_Interrupt (ignored if pin in config reg is output!)
    // int: 1 0
    //--------------
    //      0 0 ... The low level of the pin generates an interrupt.
    //      0 1 ... Any logical change on the pin generates an interrupt.
    //      1 0 ... The falling edge of the pin generates an interrupt.
    //      1 1 ... The rising edge of the pin generates an interrupt.
    pub inten: u32,
    // Interrupt config bit 2^0 (ignored if pin in inten is disabled!)
    pub int0: u32,
    // Interrupt config bit 2^1 (ignored if pin in inten is disabled!)
    pub int1: u32,
}

impl RegMemory {
    pub fn new() -> RegMemory {
        return RegMemory {
            input:  0x00000000,
            output: 0x00000000,
            config: 0xFFFFFFFF,
            inten:  0x00000000,
            int0:   0x00000000,
            int1:   0x00000000,
        }
    }
    pub fn reset(&mut self) {
        self.input =    0x00000000;
        self.output =   0x00000000;
        self.config =   0xFFFFFFFF;
        self.inten =    0x00000000;
        self.int0 =     0x00000000;
        self.int1 =     0x00000000;
    }
}

pub fn read_pin(pin: u8, reg: &u32) -> u8 {
    return if (((*reg) >> pin) & 0x1) > 0 { 1 } else { 0 };
}

pub fn write_pin(pin: u8, val: u8, reg: &mut u32) {
    if val > 0 {
        (*reg) |= 0x1u32 << pin;
    } else {
        (*reg) &= !(0x1u32 << pin);
    }
}

pub fn reg_to_str(reg: &u32) -> String {
    return format!("{:#010X}", *reg);
}

fn hex2uint(raw: String) -> Result<u32, String> {
    if !raw.is_ascii() {
        return Err("Provided string contains non-ASCII letters.".to_owned());
    }
    let mut ret: u32 = 0;
    let mut i: u32 = 0;
    let mut v: u8;
    for byte in raw.bytes().rev() {
        if byte >= b'A' && byte <= b'F' {
            v = byte - b'A' + 10;
        } else if byte >= b'0' && byte <= b'9' {
            v = byte - b'0';
        } else {
            return Err("Invalid token in provided string!".to_owned());
        }
        ret += (v as u32) * 16u32.pow(i);
        i += 1;
    }
    return Ok(ret);
}
pub fn str_to_reg(stri: String, reg: &mut u32) {
    let stri = stri.trim_start_matches("0x").to_owned();
    *reg = match hex2uint(stri) {
        Ok(num) => num,
        Err(error) => {
            panic!("Problem while converting provided string! ({})", error);
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reset() {
        let mut regmem = RegMemory::new();
        regmem.input = 0x12345678u32;
        regmem.config = 0x87654321u32;
        regmem.reset();
        assert_eq!(regmem.input, 0x00000000u32);
        assert_eq!(regmem.config, 0xFFFFFFFFu32);
    }

    #[test]
    fn test_read_pin() {
        let test_reg = 0x00FF0000u32;
        assert_eq!(read_pin(23, &test_reg), 1);
    }

    #[test]
    fn test_write_pin() {
        let mut test_reg = 0x00FF0000u32;
        write_pin(23, 0, &mut test_reg);
        assert_eq!(test_reg, 0x007F0000u32);
    }

    #[test]
    fn test_reg_to_str() {
        let test_reg = 0x00FF0000u32;
        assert_eq!(reg_to_str(&test_reg), String::from("0x00FF0000"));
    }

    #[test]
    fn test_str_to_reg() {
        let test_reg_str = String::from("0x00FF0000");
        let mut test_reg = 0x0u32;
        str_to_reg(test_reg_str, &mut test_reg);
        assert_eq!(test_reg, 0x00FF0000u32);
    }
    #[test]
    #[should_panic]
    fn test_str_to_reg_should_panic() {
        let test_reg_str = String::from("0x00XYZ000");
        let mut test_reg = 0x0u32;
        str_to_reg(test_reg_str, &mut test_reg);
    }
}
