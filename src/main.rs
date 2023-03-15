use std::sync::Mutex;
use std::thread::sleep;
use std::time::Duration;

use display::*;
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::Point;
use embedded_graphics::prelude::RgbColor;
use embedded_graphics::prelude::Size;
use embedded_graphics::primitives::Rectangle;
use embedded_graphics::Drawable;
use embedded_hal::prelude::_embedded_hal_blocking_delay_DelayMs;
use embedded_text::alignment::HorizontalAlignment;
use embedded_text::alignment::VerticalAlignment;
use esp_idf_hal::delay;
use esp_idf_hal::gpio;

#[macro_use]
extern crate derive_builder;

use esp_idf_hal::prelude::*;

use esp_idf_sys as _;

// use embedded_svc::mqtt::client::utils::ConnState;

// use ::log::info;
use rand::thread_rng;
// use url;
pub mod display;

#[allow(dead_code)]
const SSID: &str = env!("RUST_ESP32_STD_DEMO_WIFI_SSID");
#[allow(dead_code)]
const PASS: &str = env!("RUST_ESP32_STD_DEMO_WIFI_PASS");
#[macro_use]
extern crate lazy_static;

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_sys::link_patches();
    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();
    let peripherals = Peripherals::take().unwrap();
    let pins = peripherals.pins;

    let mut p1 = gpio::PinDriver::output(pins.gpio1).unwrap();
    let button = gpio::PinDriver::input(pins.gpio14).unwrap();

    let mut display = TDisplayS3::new(
        pins.gpio15.into(),
        pins.gpio38.into(),
        pins.gpio6.into(), // #define PIN_LCD_CS 6
        pins.gpio9.into(), // #define PIN_LCD_RD 9
        pins.gpio7.into(), // #define PIN_LCD_DC 7
        pins.gpio8.into(), // #define PIN_LCD_WR 8
        pins.gpio5.into(), // #define PIN_LCD_RES 5
        OutputBusPins8Bit {
            d0: pins.gpio39.into(), // #define PIN_LCD_D0 39
            d1: pins.gpio40.into(), // #define PIN_LCD_D1 40
            d2: pins.gpio41.into(), // #define PIN_LCD_D2 41
            d3: pins.gpio42.into(), // #define PIN_LCD_D3 42
            d4: pins.gpio45.into(), // #define PIN_LCD_D4 45
            d5: pins.gpio46.into(), // #define PIN_LCD_D5 46
            d6: pins.gpio47.into(), // #define PIN_LCD_D6 47
            d7: pins.gpio48.into(), // #define PIN_LCD_D7 48
        },
    );
    display.clear(Rgb565::BLACK);
    do_stuff(display);
}

fn do_stuff(mut display: TDisplayS3) {
    // let mut screen = LayoutManager::new(&mut display);
    let mut log_box: FramedTextBox = FramedTextBoxBuilder::new(Rectangle::new(
        Point::new(0, 0),
        Size::new(200, display::SCREEN_HIGHT),
    ))
    .build();
    // sleep(Duration::from_millis(1000));

    let mut log_field: TextBoxPrinter = TextBoxPrinter::new(log_box);

    for i in 0..10 {
        log_field.txt(&format!("line {}", i));
        log_field.draw(&mut display);
        sleep(Duration::from_millis(1000));
    }
}

// fn foo<'a>(t: &'a mut TextBoxPrinter, d: &mut TDisplayS3){
//     t.txt("foooo", d);
// }
