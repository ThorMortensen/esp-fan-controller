use std::thread::sleep;
use std::time::Duration;

use anyhow::bail;
use display::OutputBusPins8Bit;
use display_interface_parallel_gpio::Generic8BitBus;
use display_interface_parallel_gpio::PGPIO8BitInterface;
use display_interface_spi::SPIInterfaceNoCS;
use embedded_graphics::mono_font::ascii::FONT_10X20;
use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::prelude::Dimensions;
use embedded_graphics::prelude::DrawTarget;
use embedded_graphics::prelude::Point;
use embedded_graphics::prelude::RgbColor;
use embedded_graphics::prelude::Size;
use embedded_graphics::primitives::Line;
use embedded_graphics::primitives::Primitive;
use embedded_graphics::primitives::PrimitiveStyleBuilder;
use embedded_graphics::primitives::Rectangle;
use embedded_graphics::text::Text;
use embedded_graphics::Drawable;
use embedded_graphics_sparklines::Sparkline;
use embedded_hal::digital::v2::OutputPin;
use esp_idf_hal::delay;
use esp_idf_hal::gpio;
use esp_idf_hal::gpio::AnyOutputPin;
use esp_idf_hal::gpio::Output;
use esp_idf_hal::prelude::*;
use esp_idf_hal::spi;
use esp_idf_svc::log;
use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported

// use embedded_svc::mqtt::client::utils::ConnState;

use ::log::info;
use mipidsi::Display;
use rand::Rng;
use url;
pub mod display;

#[allow(dead_code)]
const SSID: &str = env!("RUST_ESP32_STD_DEMO_WIFI_SSID");
#[allow(dead_code)]
const PASS: &str = env!("RUST_ESP32_STD_DEMO_WIFI_PASS");

const LCD_WIDTH: u16 = 170; // width
const LCD_HIGHT: u16 = 320; // height

// /*ESP32S3*/
// #define PIN_LCD_BL                   38

// #define PIN_LCD_D0                   39
// #define PIN_LCD_D1                   40
// #define PIN_LCD_D2                   41
// #define PIN_LCD_D3                   42
// #define PIN_LCD_D4                   45
// #define PIN_LCD_D5                   46
// #define PIN_LCD_D6                   47
// #define PIN_LCD_D7                   48

// #define PIN_POWER_ON                 15

// #define PIN_LCD_RES                  5 *
// #define PIN_LCD_CS                   6
// #define PIN_LCD_DC                   7
// #define PIN_LCD_WR                   8
// #define PIN_LCD_RD                   9

// #define PIN_BUTTON_1                 0
// #define PIN_BUTTON_2                 14
// #define PIN_BAT_VOLT                 4

// #define PIN_IIC_SCL                  17
// #define PIN_IIC_SDA                  18

// #define PIN_TOUCH_INT                16
// #define PIN_TOUCH_RES                21

// type LcdDataBus =
//     Generic8BitBus<LcdData0, LcdData1, LcdData2, LcdData3, LcdData4, LcdData5, LcdData6, LcdData7>;

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

    let mut display = display::TDisplayS3::new(
        pins.gpio15.into(),
        pins.gpio38.into(),
        pins.gpio6.into(), // #define PIN_LCD_CS 6
        pins.gpio9.into(), // #define PIN_LCD_RD 9
        pins.gpio7.into(), // #define PIN_LCD_DC 7
        pins.gpio8.into(), // #define PIN_LCD_WR 8
        pins.gpio5.into(),  // #define PIN_LCD_RES 5
        OutputBusPins8Bit {
            d0: pins.gpio39.into(),// #define PIN_LCD_D0 39
            d1: pins.gpio40.into(),// #define PIN_LCD_D1 40
            d2: pins.gpio41.into(),// #define PIN_LCD_D2 41
            d3: pins.gpio42.into(),// #define PIN_LCD_D3 42
            d4: pins.gpio45.into(),// #define PIN_LCD_D4 45
            d5: pins.gpio46.into(),// #define PIN_LCD_D5 46
            d6: pins.gpio47.into(),// #define PIN_LCD_D6 47
            d7: pins.gpio48.into(),// #define PIN_LCD_D7 48
        },
    );

    display.clear(RgbColor::YELLOW);


    // let bbox = Rectangle::new(Point::new(0, 26), Size::new(240, 90));
    // let draw_fn = |lastp, p| Line::new(lastp, p);

    // // create sparkline object
    // let mut sparkline = Sparkline::new(
    //     bbox, // position and size of the sparkline
    //     32,   // max samples to store in memory (and display on graph)
    //     RgbColor::RED,
    //     1, // stroke size
    //     draw_fn,
    // );

    // loop {
    //   let val = rand::thread_rng().gen_range(0..100);
    //   sparkline.add(val);
    //   sparkline.draw(&mut display).unwrap();

    // }
}

#[allow(unused)]
fn esp32s3_usb_otg_hello_world(
    // backlight: gpio::Gpio9,
    // dc: gpio::Gpio4,
    // rst: gpio::Gpio8,
    // spi: spi::SPI3,
    // sclk: gpio::Gpio6,
    // sdo: gpio::Gpio7,
    // cs: gpio::Gpio5,
    // bus: [gpio::Gpio39 gpio::Gpio40, gpio::Gpio41, gpio::Gpio42, gpio::Gpio45, gpio::Gpio46, gpio::Gpio47, gpio::Gpio48]
    //
    // #define PIN_LCD_RES                  5
    // #define PIN_LCD_CS                   6
    // #define PIN_LCD_DC                   7
    // #define PIN_LCD_WR                   8
    // #define PIN_LCD_RD                   9
    backlight: gpio::Gpio38,
    dc: gpio::Gpio7,
    rst: gpio::Gpio5,
    spi: spi::SPI3,
    sclk: gpio::Gpio17,
    sdo: gpio::Gpio8,
    cs: gpio::Gpio6,
    sdi: gpio::Gpio9,
    bus: Generic8BitBus<
        gpio::Gpio39,
        gpio::Gpio40,
        gpio::Gpio41,
        gpio::Gpio42,
        gpio::Gpio45,
        gpio::Gpio46,
        gpio::Gpio47,
        gpio::Gpio48,
    >,
) -> Result<(), anyhow::Error> {
    info!("About to initialize the ESP32-S3-USB-OTG SPI LED driver 8-Bit 8080");

    let mut backlight = gpio::PinDriver::output(backlight)?;
    backlight.set_high()?;

    let di = SPIInterfaceNoCS::new(
        spi::SpiDeviceDriver::new_single(
            spi,
            sclk,
            sdo,
            Some(sdi),
            spi::Dma::Disabled,
            Some(cs),
            &spi::SpiConfig::new().baudrate(10.MHz().into()), //.data_mode(embedded_hal_01::spi::MODE_3),
        )?,
        gpio::PinDriver::output(dc)?,
    );

    let mut display = mipidsi::Builder::st7789(di)
        .with_display_size(LCD_WIDTH, LCD_HIGHT)
        .init(&mut delay::Ets, Some(gpio::PinDriver::output(rst)?))
        .map_err(|e| anyhow::anyhow!("Display error: {:?}", e))?;

    if let Err(e) = display
        .set_pixel(20, 20, RgbColor::WHITE)
        .map_err(|e| anyhow::anyhow!("Display error: {:?}", e))
    {
        dbg!(e);
    }

    dbg!(display.bounding_box());

    display
        .set_orientation(mipidsi::options::Orientation::Landscape(false))
        .map_err(|e| anyhow::anyhow!("Display error: {:?}", e))?;

    led_draw(&mut display).map_err(|e| anyhow::anyhow!("Led draw error: {:?}", e))
}

fn led_draw<D>(display: &mut D) -> Result<(), D::Error>
where
    D: DrawTarget + Dimensions,
    D::Color: RgbColor,
{
    display.clear(RgbColor::WHITE)?;

    Rectangle::new(display.bounding_box().top_left, display.bounding_box().size)
        .into_styled(
            PrimitiveStyleBuilder::new()
                .fill_color(RgbColor::BLUE)
                .stroke_color(RgbColor::BLACK)
                .stroke_width(1)
                .build(),
        )
        .draw(display)?;

    Text::new(
        "Hellasdafo Rust!",
        Point::new(10, (display.bounding_box().size.height - 10) as i32 / 2),
        MonoTextStyle::new(&FONT_10X20, RgbColor::WHITE),
    )
    .draw(display)?;

    Text::new(
        "This is the corner",
        Point::new(0, 10),
        MonoTextStyle::new(&FONT_10X20, RgbColor::WHITE),
    )
    .draw(display)?;

    info!("LED rendering done");

    Ok(())
}
