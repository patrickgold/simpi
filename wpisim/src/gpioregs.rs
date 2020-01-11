/*!gpioregs.rs
 * Module File for Raspberry Pi GPIO Registers.
 * 
 * Author: Patrick Goldinger
 * License: GPL 3.0 (see LICENSE file for details)
 */

pub struct Reg {
    value: u32
}

impl Reg {
    pub fn new() -> Reg {
        return Reg {
            value: 0x00000000
        };
    }
    pub fn from(val: u32) -> Reg {
        return Reg {
            value: val
        };
    }
    pub fn read(&self) -> u32 {
        return self.value;
    }
    pub fn read_pin(&self, pin: u8) -> u8 {
        return if ((self.value >> pin) & 0x1) > 0 { 1 } else { 0 };
    }
    pub fn read_to_str(&self) -> String {
        return format!("{:#010X}", self.value);
    }
    pub fn write(&mut self, val: u32) {
        self.value = val;
    }
    pub fn write_pin(&mut self, pin: u8, val: u8) {
        if val > 0 {
            self.value |= 0x1u32 << pin;
        } else {
            self.value &= !(0x1u32 << pin);
        }
    }
    fn hex2uint(&self, raw: String) -> Result<u32, String> {
        if !raw.is_ascii() {
            return Err("Provided string contains non-ASCII letters.".to_owned());
        }
        let raw = raw.to_ascii_uppercase();
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
    pub fn write_from_str(&mut self, stri: String) {
        let stri = stri.trim_start_matches("0x").to_owned();
        self.value = match self.hex2uint(stri) {
            Ok(num) => num,
            Err(error) => {
                panic!("Problem while converting provided string! ({})", error);
            }
        };
    }
}

pub struct RegMemory {
    // INPUT register (seen from programmer's view)
    pub input: Reg,
    // OUTPUT register (seen from programmer's view)
    pub output: Reg,
    // 1=Input 0=Output (All pins default to input!)
    pub config: Reg,
    // 1=Interrupt 0=No_Interrupt (ignored if pin in config reg is output!)
    // int: 1 0
    //--------------
    //      0 0 ... The low level of the pin generates an interrupt.
    //      0 1 ... Any logical change on the pin generates an interrupt.
    //      1 0 ... The falling edge of the pin generates an interrupt.
    //      1 1 ... The rising edge of the pin generates an interrupt.
    pub inten: Reg,
    // Interrupt config bit 2^0 (ignored if pin in inten is disabled!)
    pub int0: Reg,
    // Interrupt config bit 2^1 (ignored if pin in inten is disabled!)
    pub int1: Reg,
}

impl RegMemory {
    pub fn new() -> RegMemory {
        return RegMemory {
            input:  Reg::from(0x00000000),
            output: Reg::from(0x00000000),
            config: Reg::from(0xFFFFFFFF),
            inten:  Reg::from(0x00000000),
            int0:   Reg::from(0x00000000),
            int1:   Reg::from(0x00000000),
        }
    }
    pub fn reset(&mut self) {
        self.input.write(0x00000000);
        self.output.write(0x00000000);
        self.config.write(0xFFFFFFFF);
        self.inten.write(0x00000000);
        self.int0.write(0x00000000);
        self.int1.write(0x00000000);
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reset() {
        let mut regmem = RegMemory::new();
        regmem.input.write(0x12345678u32);
        regmem.config.write(0x87654321u32);
        regmem.reset();
        assert_eq!(regmem.input.read(), 0x00000000u32);
        assert_eq!(regmem.config.read(), 0xFFFFFFFFu32);
    }

    #[test]
    fn test_read_pin() {
        let reg = Reg::from(0x00FF0000u32);
        assert_eq!(reg.read_pin(23), 1);
    }

    #[test]
    fn test_write_pin() {
        let mut reg = Reg::from(0x00FF0000u32);
        reg.write_pin(23, 0);
        assert_eq!(reg.read(), 0x007F0000u32);
    }

    #[test]
    fn test_reg_to_str() {
        let reg = Reg::from(0x00FF0000u32);
        assert_eq!(reg.read_to_str(), String::from("0x00FF0000"));
    }

    #[test]
    fn test_str_to_reg() {
        let reg_str = String::from("0x00FF0000");
        let mut reg = Reg::from(0x0u32);
        reg.write_from_str(reg_str);
        assert_eq!(reg.read(), 0x00FF0000u32);
    }
    #[test]
    #[should_panic]
    fn test_str_to_reg_should_panic() {
        let reg_str = String::from("0x00XYZ000");
        let mut reg = Reg::from(0x0u32);
        reg.write_from_str(reg_str);
    }
}
