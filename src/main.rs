use std::thread::sleep;
use std::time::Duration;

use display_interface_spi::SPIInterfaceNoCS;
use embedded_graphics::mono_font::ascii::FONT_10X20;
use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics::prelude::Dimensions;
use embedded_graphics::prelude::DrawTarget;
use embedded_graphics::prelude::Point;
use embedded_graphics::prelude::RgbColor;
use embedded_graphics::primitives::Primitive;
use embedded_graphics::primitives::PrimitiveStyleBuilder;
use embedded_graphics::primitives::Rectangle;
use embedded_graphics::text::Text;
use embedded_graphics::Drawable;
use esp_idf_hal::adc;
use esp_idf_hal::delay;
use esp_idf_hal::gpio;
use esp_idf_hal::i2c;
use esp_idf_hal::prelude::*;
use esp_idf_hal::spi;
use esp_idf_svc::log;
use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported

use anyhow::bail;

// use embedded_svc::mqtt::client::utils::ConnState;

use ::log::info;
use mipidsi::Display;
use url;

#[allow(dead_code)]
#[cfg(not(feature = "qemu"))]
const SSID: &str = env!("RUST_ESP32_STD_DEMO_WIFI_SSID");
#[allow(dead_code)]
#[cfg(not(feature = "qemu"))]
const PASS: &str = env!("RUST_ESP32_STD_DEMO_WIFI_PASS");

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

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_sys::link_patches();
    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take().unwrap();
    let pins = peripherals.pins;
    // dbg!("Hello, world!");
    let mut p1 = gpio::PinDriver::output(pins.gpio1).unwrap();
    let button = gpio::PinDriver::input(pins.gpio14).unwrap();

    loop {
        p1.set_level(!button.get_level()).unwrap();
        sleep(Duration::from_millis(200));
    }

    // esp32s3_usb_otg_hello_world(
    //     // pins.gpio9,
    //     // pins.gpio4,
    //     // pins.gpio8,
    //     // peripherals.spi3,
    //     // pins.gpio6,
    //     // pins.gpio7,
    //     // pins.gpio5,
    //     pins.gpio38,
    //     pins.gpio7,
    //     pins.gpio5,
    //     peripherals.spi3,
    //     pins.gpio17,
    //     pins.gpio8,
    //     pins.gpio6,
    // )
    // .unwrap();
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
) -> Result<(), anyhow::Error> {
    info!("About to initialize the ESP32-S3-USB-OTG SPI LED driver ST7789VW");

    let mut backlight = gpio::PinDriver::output(backlight)?;
    backlight.set_high()?;

    let di = SPIInterfaceNoCS::new(
        spi::SpiDeviceDriver::new_single(
            spi,
            sclk,
            sdo,
            Option::<gpio::AnyIOPin>::None,
            spi::Dma::Disabled,
            Some(cs),
            &spi::SpiConfig::new().baudrate(10.MHz().into()),
        )?,
        gpio::PinDriver::output(dc)?,
    );

    let mut display = mipidsi::Builder::st7789(di)
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
                .stroke_color(RgbColor::YELLOW)
                .stroke_width(1)
                .build(),
        )
        .draw(display)?;

    Text::new(
        "Hello Rust!",
        Point::new(10, (display.bounding_box().size.height - 10) as i32 / 2),
        MonoTextStyle::new(&FONT_10X20, RgbColor::WHITE),
    )
    .draw(display)?;

    info!("LED rendering done");

    Ok(())
}
