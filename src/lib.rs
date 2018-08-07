#![no_std]

extern crate embedded_hal;

use embedded_hal::digital::OutputPin;
use embedded_hal::blocking::delay::DelayUs;
use embedded_hal::blocking::delay::DelayMs;

pub struct HD44780<
    D: DelayUs<u16> + DelayMs<u8>,
    O1: OutputPin,
    O2: OutputPin,
    O3: OutputPin,
    O4: OutputPin,
    O5: OutputPin,
    O6: OutputPin,
    O7: OutputPin,
    O8: OutputPin,
    O9: OutputPin,
    O10: OutputPin,
> {
    rs: O1,
    en: O2,
    db0: O3,
    db1: O4,
    db2: O5,
    db3: O6,
    db4: O7,
    db5: O8,
    db6: O9,
    db7: O10,
    delay: D,
}

pub enum Direction {
    Left,
    Right,
}

impl<
        D: DelayUs<u16> + DelayMs<u8>,
        O1: OutputPin,
        O2: OutputPin,
        O3: OutputPin,
        O4: OutputPin,
        O5: OutputPin,
        O6: OutputPin,
        O7: OutputPin,
        O8: OutputPin,
        O9: OutputPin,
        O10: OutputPin,
    > HD44780<D, O1, O2, O3, O4, O5, O6, O7, O8, O9, O10>
{
    pub fn new(
        rs: O1,
        en: O2,
        db0: O3,
        db1: O4,
        db2: O5,
        db3: O6,
        db4: O7,
        db5: O8,
        db6: O9,
        db7: O10,
        delay: D,
    ) -> HD44780<D, O1, O2, O3, O4, O5, O6, O7, O8, O9, O10> {
        let delay = delay;

        let mut hd = HD44780 {
            rs,
            en,
            db0,
            db1,
            db2,
            db3,
            db4,
            db5,
            db6,
            db7,
            delay,
        };

        hd.init_8bit();

        return hd;
    }

    /// Sets DDRAM address 0 in
    /// address counter. Also
    /// returns display from being
    /// shifted to original position.
    /// DDRAM contents remain
    /// unchanged.
    pub fn reset(&mut self) {
        self.send_byte(0b0000_0010);
        self.delay.delay_us(50);
    }

    
    pub fn set_display_mode(&mut self, display_on: bool, cursor_visible: bool, cursor_blink: bool) {
        let display_bit = {
            if display_on {
                0b0000_0100
            } else {
                0b0000_0000
            }
        };

        let cursor_visible_bit = {
            if cursor_visible {
                0b0000_0010
            } else {
                0b0000_0000
            }
        };

        let cursor_blink_bit = {
            if cursor_blink {
                0b0000_0001
            } else {
                0b0000_0000
            }
        };

        let cmd_byte = 0b0000_1000 | display_bit | cursor_visible_bit | cursor_blink_bit;

        self.send_byte(cmd_byte);
        self.delay.delay_us(50);
    }

    pub fn clear(&mut self) {
        self.send_byte(0b0000_0001);
        self.delay.delay_us(50);
    }

    pub fn shift_cursor(&mut self, dir: Direction) {
        let bits = match dir {
            Direction::Left => 0b0000_0000,
            Direction::Right => 0b0000_0100,
        };

        self.send_byte(0b0001_0000 | bits);
        self.delay.delay_us(50);
    }

    pub fn shift_display(&mut self, dir: Direction) {
        let bits = match dir {
            Direction::Left => 0b0000_0000,
            Direction::Right => 0b0000_0100,
        };

        self.send_byte(0b0001_1000 | bits);

        self.delay.delay_us(50);
    }

    fn init_8bit(&mut self) {
        self.delay.delay_ms(15u8);

        self.set_bus_bits(0b0011_0000); // Initialize Lcd in 8-bit mode
        self.pulse_enable();

        self.delay.delay_ms(5u8);

        self.set_bus_bits(0b0011_1000); // enable 5x7 mode for chars
        self.pulse_enable();

        self.delay.delay_us(100);

        self.send_byte(0b0000_1110);
        self.pulse_enable();

        self.delay.delay_us(100);

        self.send_byte(0b0000_0001); // Clear Display
        self.pulse_enable();

        self.delay.delay_us(100);

        self.send_byte(0b000_0111); // Move the cursor to beginning of first line
        self.pulse_enable();

        self.delay.delay_us(100);

        self.send_byte(0b000_0110); // Set entry mode to increment by one
        self.pulse_enable();

        self.delay.delay_us(100);

        self.shift_cursor(Direction::Left);
    }

    ///
    /// Read busy flag and address reads the busy flag (BF) indicating that the system is now internally operating
    /// on a previously received instruction. If BF is 1, the internal operation is in progress. The next instruction
    /// will not be accepted until BF is reset to 0. Check the BF status before the next write operation. At the same
    /// time, the value of the address counter in binary AAAAAAA is read out. This address counter is used by
    /// both  CG  and  DDRAM  addresses,  and  its  value  is  determined  by  the  previous  instruction.  The  address
    /// contents are the same as for instructions set CGRAM address and set DDRAM address.
    /*fn read_busy_flag(&mut self) -> bool {

        self.rw.set_high(); // read

        let busy_flag = self.db7.is_high();

        self.rw.set_low();

        return busy_flag;
    }*/

    fn pulse_enable(&mut self) {
        self.en.set_high();
        self.delay.delay_ms(15u8);
        self.en.set_low();
    }

    pub fn write_str(&mut self, string: &str) {
        for c in string.chars() {
            self.write_char(c);
        }
    }

    pub fn write_char(&mut self, data: char) {
        self.rs.set_high();

        self.send_byte(data as u8);

        self.rs.set_low();

        self.delay.delay_us(100);
    }

    fn send_byte(&mut self, data: u8) {
        // 8-bit mode

        // Pulse the enable pin
        self.set_bus_bits(data);
        self.pulse_enable();

        /*let high_nibble = (data & 0xF0); 

        self.send_nibble(high_nibble);

        let lower_nibble = (data << 4) & 0xF0;

        self.send_nibble(lower_nibble);*/
    }

    fn set_bus_bits(&mut self, data: u8) {
        let db0: bool = (0b0000_0001 & data) != 0;
        let db1: bool = (0b0000_0010 & data) != 0;
        let db2: bool = (0b0000_0100 & data) != 0;
        let db3: bool = (0b0000_1000 & data) != 0;
        let db4: bool = (0b0001_0000 & data) != 0;
        let db5: bool = (0b0010_0000 & data) != 0;
        let db6: bool = (0b0100_0000 & data) != 0;
        let db7: bool = (0b1000_0000 & data) != 0;

        if db0 {
            self.db0.set_high();
        } else {
            self.db0.set_low();
        }

        if db1 {
            self.db1.set_high();
        } else {
            self.db1.set_low();
        }

        if db2 {
            self.db2.set_high();
        } else {
            self.db2.set_low();
        }

        if db3 {
            self.db3.set_high();
        } else {
            self.db3.set_low();
        }

        if db4 {
            self.db4.set_high();
        } else {
            self.db4.set_low();
        }

        if db5 {
            self.db5.set_high();
        } else {
            self.db5.set_low();
        }

        if db6 {
            self.db6.set_high();
        } else {
            self.db6.set_low();
        }

        if db7 {
            self.db7.set_high();
        } else {
            self.db7.set_low();
        }
    }
}
