use esp_idf_sys; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported
use esp_idf_hal::{prelude::*, gpio::{PinDriver, Gpio0}, delay::{self}, spi::{self, SpiDeviceDriver}, i2c::{self, I2cDriver}, units::MegaHertz};
use embedded_hal::delay::DelayUs;
use mipidsi::{Builder, Orientation};
use display_interface_spi::SPIInterface;
use embedded_graphics::{prelude::*, pixelcolor::Rgb565, text::{Text, Alignment, Baseline, TextStyleBuilder}, mono_font::{ascii::FONT_10X20, MonoTextStyle}};
use mpu6050::Mpu6886;

mod axp192;

fn main() {
    // Temporary. Will disappear once ESP-IDF 4.4 is released, but for now it is necessary to call this function once,
    // or else some patches to the runtime implemented by esp-idf-sys might not link properly.
    esp_idf_sys::link_patches();

    let mut delay = delay::FreeRtos;

    let peripherals = Peripherals::take().unwrap();

    let i2c_config = i2c::config::Config {
        baudrate: KiloHertz(400).into(),
        sda_pullup_enabled: true,
        scl_pullup_enabled: true
    };
    let i2c = I2cDriver::new(peripherals.i2c1, peripherals.pins.gpio21, peripherals.pins.gpio22, &i2c_config).unwrap();
    let i2c_bus = shared_bus::BusManagerSimple::new(i2c);

    // Configure MPU-6886 IMU
    let mut imu = Mpu6886::new(i2c_bus.acquire_i2c());
    imu.init(&mut delay).unwrap();

    // Configure power management IC (AXP192)
    let mut pmic = axp192::Axp192::new(i2c_bus.acquire_i2c());
    pmic.init().unwrap();

    // Init SPI and display
    // Reference: https://github.com/esp-rs/esp-idf-hal/blob/master/examples/spi_st7789.rs
    let spi_config = spi::config::Config::new()
        .baudrate(MegaHertz(50).into())
        .data_mode(embedded_hal::spi::MODE_3)
        .write_only(true);
    let spi = SpiDeviceDriver::new_single(
        peripherals.spi2, peripherals.pins.gpio13, peripherals.pins.gpio15, None as Option<Gpio0>,
        spi::Dma::Auto(1024), None as Option<Gpio0>, &spi_config
    ).unwrap();
    let lcd_dc = PinDriver::output(peripherals.pins.gpio23).unwrap();
    let lcd_cs = PinDriver::output(peripherals.pins.gpio5).unwrap();
    let display_interface = SPIInterface::new(spi, lcd_dc, lcd_cs);
    let lcd_rst = PinDriver::output(peripherals.pins.gpio18).unwrap();
    // Display settings for Pico1 variant of ST7789 miraculously matches with M5StickC Plus
    let mut display = Builder::st7789_pico1(display_interface)
        .with_orientation(Orientation::Landscape(true))
        .init(&mut delay, Some(lcd_rst)).unwrap();

    let char_style = MonoTextStyle::new(&FONT_10X20, Rgb565::WHITE);
    let text_style = TextStyleBuilder::new().alignment(Alignment::Center).baseline(Baseline::Middle).build();

    loop {
        display.clear(Rgb565::BLACK).unwrap();

        Text::with_text_style(
            "Hello Rust on M5!",
            display.bounding_box().center() - Point::new(0, 10),
            char_style, text_style
        ).draw(&mut display).unwrap();

        let accel = imu.get_acc().unwrap();
        let gyro = imu.get_gyro().unwrap();

        Text::with_text_style(
            format!("{:.2},{:.2},{:.2}", accel.x, accel.y, accel.z).as_str(),
            display.bounding_box().center() + Point::new(0, 10),
            char_style, text_style
        ).draw(&mut display).unwrap();

        Text::with_text_style(
            format!("{:.2},{:.2},{:.2}", gyro.x, gyro.y, gyro.z).as_str(),
            display.bounding_box().center() + Point::new(0, 30),
            char_style, text_style
        ).draw(&mut display).unwrap();

        delay.delay_ms(500).unwrap();
    }
}
