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
use embedded_text::alignment::HorizontalAlignment;
use embedded_text::alignment::VerticalAlignment;
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

// lazy_static! {
// static ref display: Mutex<TDisplayS3> = Mutex::new();

// }

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_sys::link_patches();
    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();
    let mut rng = thread_rng();

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

    display.clear(RgbColor::BLACK);

    // let mut screen = LayoutManager::new(&mut display);
    let mut log_box: FramedTextBox<'_> = FramedTextBoxBuilder::new(Rectangle::new(
        Point::new(0, 0),
        Size::new(200, display::SCREEN_HIGHT),
    ))
    .build();

    // let mut num_box = FramedTextBoxBuilder::new_relative_to(
    //     &log_box,
    //     FramedTextBoxAnchor::Right,
    //     2,
    //     Size::new(SCREEN_WIDTH - 200 - 2, (display::SCREEN_HIGHT / 5) - 2),
    // )
    // .alignment(HorizontalAlignment::Center)
    // .alignment_vertical(VerticalAlignment::Middle);
    // let num_box = num_box.build();

    // let mut small_box =
    //     FramedTextBoxBuilder::copy_relative_to(&num_box, FramedTextBoxAnchor::Down, 2).build();

    // let mut medium_box = FramedTextBoxBuilder::new_relative_to(
    //     &small_box,
    //     FramedTextBoxAnchor::Down,
    //     2,
    //     Size::new(SCREEN_WIDTH - 200 - 2, (display::SCREEN_HIGHT / 5) * 3),
    // )
    // .frame_color(RgbColor::GREEN)
    // .alignment_vertical(VerticalAlignment::Middle)
    // .alignment(HorizontalAlignment::Center)
    // .build();

    let mut log_field:TextBoxPrinter = TextBoxPrinter::new(log_box);
    {

        log_field.txt("fooo", &mut display);

    }
    log_field.txt("fooo", &mut display);


    //let mut xx = std::sync::Arc::new(log_box);
    // screen.txt(
    //     "012345678901234567890123456789\n--> This is line 2",
    //     &mut log_box,
    // );
    // screen.txt("42", &mut num_box);
    // screen.txt("small2", &mut small_box);
    // screen.txt("Clock:\n12:42", &mut medium_box);

    // let lineCount = 0;
    // loop {
    //     // log_box.text_box.text = &str;
    //     // log_box.text_box.draw(&mut display.screen);
    //     // log_field.txt(str, &mut log_box, &mut display);
    // //     // let r = rng.gen_range(1..120);
    // //     // let g = rng.gen_range(1..120);
    // //     // let b = rng.gen_range(1..120);
    // //     // display.clear(Rgb565::new(r, g, b).into());
    // //     //TextBoxPrinter::txt(&mut log_field, format!("bingo"));
    // //     // log_field.txt("alsijdsl", &mut display);
    // //     // log_field.flush(&mut display);

    // //     sleep(Duration::from_millis(1000));
    // }
}



// fn foo<'a>(t: &'a mut TextBoxPrinter, d: &mut TDisplayS3){
//     t.txt("foooo", d);
// }
