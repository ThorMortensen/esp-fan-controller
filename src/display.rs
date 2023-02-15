use anyhow::Error;
use display_interface::WriteOnlyDataCommand;
use display_interface_parallel_gpio::Generic8BitBus;
use display_interface_parallel_gpio::PGPIO8BitInterface;
use embedded_graphics::mono_font::ascii::FONT_10X20;
use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::pixelcolor::Rgb555;
use embedded_graphics::pixelcolor::Rgb565;
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
use esp_idf_hal::gpio::PinDriver;
use mipidsi::models::Model;
use mipidsi::models::ST7789;
use mipidsi::Display;

// type Display_I8080_ST7789 = Display<
//     PGPIO8BitInterface<
//         Generic8BitBus<PinOut, PinOut, PinOut, PinOut, PinOut, PinOut, PinOut, PinOut>,
//         PinOut,
//         PinOut,
//     >,
//     ST7789,
//     PinOut,
// >;

// #[derive(Debug)]
pub struct TDisplayS3 {
    display: Display<
        PGPIO8BitInterface<
            Generic8BitBus<
                PinDriver<'static, AnyOutputPin, Output>,
                PinDriver<'static, AnyOutputPin, Output>,
                PinDriver<'static, AnyOutputPin, Output>,
                PinDriver<'static, AnyOutputPin, Output>,
                PinDriver<'static, AnyOutputPin, Output>,
                PinDriver<'static, AnyOutputPin, Output>,
                PinDriver<'static, AnyOutputPin, Output>,
                PinDriver<'static, AnyOutputPin, Output>,
            >,
            PinDriver<'static, AnyOutputPin, Output>,
            PinDriver<'static, AnyOutputPin, Output>,
        >,
        ST7789,
        PinDriver<'static, AnyOutputPin, Output>,
    >,
    power: PinDriver<'static, AnyOutputPin, Output>,
    backlight: PinDriver<'static, AnyOutputPin, Output>,
    cs: PinDriver<'static, AnyOutputPin, Output>,
    rd: PinDriver<'static, AnyOutputPin, Output>,
}

pub struct OutputBusPins8Bit{
   pub d0: gpio::AnyOutputPin,
   pub d1: gpio::AnyOutputPin,
   pub d2: gpio::AnyOutputPin,
   pub d3: gpio::AnyOutputPin,
   pub d4: gpio::AnyOutputPin,
   pub d5: gpio::AnyOutputPin,
   pub d6: gpio::AnyOutputPin,
   pub d7: gpio::AnyOutputPin,
}

impl TDisplayS3 {
    pub fn new(
        power: gpio::AnyOutputPin,
        backlight: gpio::AnyOutputPin,
        cs: gpio::AnyOutputPin,
        rd: gpio::AnyOutputPin,
        dc: gpio::AnyOutputPin,
        wr: gpio::AnyOutputPin,
        rst: gpio::AnyOutputPin,
        data_bus: OutputBusPins8Bit,
    ) -> Self {
        let bus = Generic8BitBus::new((
            gpio::PinDriver::output(data_bus.d0).unwrap(),
            gpio::PinDriver::output(data_bus.d1).unwrap(),
            gpio::PinDriver::output(data_bus.d2).unwrap(),
            gpio::PinDriver::output(data_bus.d3).unwrap(),
            gpio::PinDriver::output(data_bus.d4).unwrap(),
            gpio::PinDriver::output(data_bus.d5).unwrap(),
            gpio::PinDriver::output(data_bus.d6).unwrap(),
            gpio::PinDriver::output(data_bus.d7).unwrap(),
        ))
        .unwrap();

        let di = PGPIO8BitInterface::new(
            bus,
            gpio::PinDriver::output(dc).unwrap(),
            gpio::PinDriver::output(wr).unwrap(),
        );

        let mut cs = gpio::PinDriver::output(cs).unwrap();
        let mut rd = gpio::PinDriver::output(rd).unwrap();

        rd.set_high().unwrap();
        cs.set_low().unwrap();

        let mut display = mipidsi::Builder::st7789(di)
            // .with_display_size(LCD_WIDTH, LCD_HIGHT)
            .init(&mut delay::Ets, Some(gpio::PinDriver::output(rst).unwrap()))
            .unwrap();


        display
            .set_orientation(mipidsi::Orientation::Landscape(true))
            .unwrap();


        TDisplayS3 {
            display,
            power: gpio::PinDriver::output(power).unwrap(),
            backlight: gpio::PinDriver::output(backlight).unwrap(),
            cs,
            rd,
        }
    }

    pub fn clear(&mut self, color:Rgb565) {
        self.display.clear(color).unwrap();
    }
}
