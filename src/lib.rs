#![no_std]

extern crate embedded_hal;

use embedded_hal::blocking::delay::DelayMs;
use embedded_hal::blocking::delay::DelayUs;
use embedded_hal::digital::OutputPin;

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

/// Used in the direction argument for shifting the cursor and the display
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
    /// Create an instance of a `HD44780` from 8 data pins, a register select
    /// pin, an enable pin and a struct implementing the delay trait.
    /// - The delay instance is used to sleep between commands to 
    /// ensure the `HD44780` has enough time to process commands.
    /// - The eight db0..db7 pins are used to send and recieve with
    ///  the `HD44780`.
    /// - The register select pin is used to tell the `HD44780` 
    /// if incoming data is a command or data.
    /// - The enable pin is used to tell the `HD44780` that there 
    /// is data on the 8 data pins and that it should read them in.
    /// 
    pub fn new_8bit(
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

    /// Unshifts the display and sets the cursor position to 0
    /// 
    /// ```
    /// lcd.reset();
    /// ```
    pub fn reset(&mut self) {
        self.send_byte(0b0000_0010);

        // Wait for the command to be processed
        self.delay.delay_us(50);
    }

    /// Set if the display should be on, if the cursor should be visible, and if the cursor should blink
    /// 
    /// ```
    /// // Set the display to be on, the cursor to be visible, and the cursor to be blinking.
    /// lcd.set_display_mode(true, true, true);
    /// ```
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

        // Wait for the command to be processed
        self.delay.delay_us(50);
    }

    /// Clear the entire display
    /// 
    /// ```
    /// lcd.clear();
    /// ```
    pub fn clear(&mut self) {
        self.send_byte(0b0000_0001);

        // Wait for the command to be processed
        self.delay.delay_us(50);
    }

    /// Set the cursor position
    /// 
    /// ```
    /// // Move to line 2
    /// lcd.set_cursor_pos(40)
    /// ```
    pub fn set_cursor_pos(&mut self, position : u8) {
        
        let lower_7_bits = 0b0111_1111 & position;
        
        self.send_byte(0b1000_0000 | lower_7_bits);

        // Wait for the command to be processed
        self.delay.delay_us(50);
    }

    /// Shift just the cursor to the left or the right
    /// 
    /// ```
    /// lcd.shift_cursor(Direction::Left);
    /// lcd.shift_cursor(Direction::Right);
    /// ```
    pub fn shift_cursor(&mut self, dir: Direction) {
        let bits = match dir {
            Direction::Left => 0b0000_0000,
            Direction::Right => 0b0000_0100,
        };

        self.send_byte(0b0001_0000 | bits);

        // Wait for the command to be processed
        self.delay.delay_us(50);
    }

    /// Shift the entire display to the left or the right
    /// 
    /// ```
    /// lcd.shift_display(Direction::Left);
    /// lcd.shift_display(Direction::Right);
    /// ```
    pub fn shift_display(&mut self, dir: Direction) {
        let bits = match dir {
            Direction::Left => 0b0000_0000,
            Direction::Right => 0b0000_0100,
        };

        self.send_byte(0b0001_1000 | bits);

        // Wait for the command to be processed
        self.delay.delay_us(50);
    }

    // Follow the 8-bit setup procedure as specified in the HD44780 datasheet
    fn init_8bit(&mut self) {
        // Wait for the LCD to wakeup if it was off
        self.delay.delay_ms(15u8);

        // Initialize Lcd in 8-bit mode
        self.send_byte(0b0011_0000);

        // Wait for the command to be processed
        self.delay.delay_ms(5u8);

        // Enable 5x7 mode for chars
        self.send_byte(0b0011_1000);

        // Wait for the command to be processed
        self.delay.delay_us(100);

        self.send_byte(0b0000_1110);

        // Wait for the command to be processed
        self.delay.delay_us(100);

        // Clear Display
        self.send_byte(0b0000_0001);

        // Wait for the command to be processed
        self.delay.delay_us(100);

        // Move the cursor to beginning of first line
        self.send_byte(0b000_0111);

        // Wait for the command to be processed
        self.delay.delay_us(100);

        // Set entry mode to increment by one
        self.send_byte(0b000_0110);

        // Wait for the command to be processed
        self.delay.delay_us(100);
    }

    /// Writes an entire string to the `HD44780`
    /// 
    /// ```
    /// lcd.write_str("Hello, world!");
    /// ```
    pub fn write_str(&mut self, string: &str) {
        for c in string.chars() {
            self.write_char(c);
        }
    }

    /// Write a single character to the `HD44780`
    /// 
    /// ```
    /// lcd.write_char('A');
    /// ```
    pub fn write_char(&mut self, data: char) {
        self.rs.set_high();

        self.send_byte(data as u8);

        self.rs.set_low();

        // Wait for the command to be processed
        self.delay.delay_us(100);
    }

    // Send a byte to the HD44780 by setting the data on the bus and
    // also pulsing the enable pin
    fn send_byte(&mut self, data: u8) {
        // Pulse the enable pin
        self.set_bus_bits(data);
        self.pulse_enable();
    }

    // Pulse the enable pin telling the HD44780 that we something for it
    fn pulse_enable(&mut self) {
        self.en.set_high();
        self.delay.delay_ms(15u8);
        self.en.set_low();
    }

    // Set the pins on the data bus
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
