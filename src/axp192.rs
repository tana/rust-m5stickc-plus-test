// This is a (partial) Rust translation of AXP192 driver in M5StickC-Plus Arduino library.
// Original code: https://github.com/m5stack/M5StickC-Plus/blob/3d1fd6d535c70dceabef0872aedf3d1092cc0f29/src/AXP192.cpp
//
// MIT License
// 
// Copyright (c) 2020 M5Stack
// 
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
// 
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
// 
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use embedded_hal::i2c::I2c;

const AXP192_ADDR: u8 = 0x34;

pub struct Axp192<T: I2c> {
    i2c: T
}

impl<T: I2c> Axp192<T> {
    pub fn new(i2c: T) -> Self {
        Axp192 { i2c }
    }

    pub fn init(&mut self) -> Result<(), T::Error> {
        // Set LDO2 & LDO3(TFT_LED & TFT) 3.0V
        self.i2c.write(AXP192_ADDR, &[0x28, 0xCC])?;
        // Set ADC to All Enable
        self.i2c.write(AXP192_ADDR, &[0x82, 0xFF])?;
        // Bat charge voltage to 4.2, Current 100
        self.i2c.write(AXP192_ADDR, &[0x33, 0xC0])?;
        // Enable Bat,ACIN,VBUS,APS adc
        self.i2c.write(AXP192_ADDR, &[0x82, 0xFF])?;
        // Enable Ext, LDO2, LDO3, DCDC1
        let mut prev_value: [u8; 1] = Default::default();
        self.i2c.write_read(AXP192_ADDR, &[0x12], &mut prev_value)?;
        self.i2c.write(AXP192_ADDR, &[0x12, prev_value[0] | 0x4D])?;
        // 128ms power on, 4s power off
        self.i2c.write(AXP192_ADDR, &[0x36, 0x0C])?;
        // Set RTC voltage to 3.3V
        self.i2c.write(AXP192_ADDR, &[0x91, 0xF0])?;
        // Set GPIO0 to LDO
        self.i2c.write(AXP192_ADDR, &[0x90, 0x02])?;
        // Disable vbus hold limit
        self.i2c.write(AXP192_ADDR, &[0x30, 0x80])?;
        // Set temperature protection
        self.i2c.write(AXP192_ADDR, &[0x39, 0xFC])?;
        // Enable RTC BAT charge
        self.i2c.write(AXP192_ADDR, &[0x35, 0xA2])?;
        // Enable bat detection
        self.i2c.write(AXP192_ADDR, &[0x32, 0x46])?;

        Ok(())
    }
}