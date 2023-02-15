use anyhow::Error;
use display_interface::WriteOnlyDataCommand;
use display_interface_parallel_gpio::Generic8BitBus;
use display_interface_parallel_gpio::PGPIO8BitInterface;
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
use esp_idf_hal::gpio::Output;
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
pub struct TDisplayS3<PinOut>
where
    <PinOut as embedded_hal::digital::v2::OutputPin>::Error: std::fmt::Debug,
    PinOut: OutputPin,
{
    display: Display<
        PGPIO8BitInterface<
            Generic8BitBus<PinOut, PinOut, PinOut, PinOut, PinOut, PinOut, PinOut, PinOut>,
            PinOut,
            PinOut,
        >,
        ST7789,
        PinOut,
    >,
    power: Output,
    backlight: PinOut,
    cs: PinOut,
    rd: PinOut,
}

impl<PinOut> TDisplayS3<PinOut>
where
    <PinOut as embedded_hal::digital::v2::OutputPin>::Error: std::fmt::Debug,
    PinOut: OutputPin + Copy,
{
    pub fn new(
        mut power: Output,
        mut backlight: PinOut,
        mut cs: PinOut,
        mut rd: PinOut,
        dc: PinOut,
        wr: PinOut,
        rst: PinOut,
        data_bus: [PinOut; 8],
    ) -> Self {
        power.set_high().unwrap();
        backlight.set_high().unwrap();
        cs.set_low().unwrap();
        rd.set_high().unwrap();

        let bus = Generic8BitBus::new((
            data_bus[0], data_bus[1], data_bus[2], data_bus[3], data_bus[4], data_bus[5], data_bus[6],
            data_bus[7],
        ))
        .unwrap();
        let di = PGPIO8BitInterface::new(bus, dc, wr);

        let mut display = mipidsi::Builder::st7789(di)
            // .with_display_size(LCD_WIDTH, LCD_HIGHT)
            .init(&mut delay::Ets, Some(rst))
            .unwrap();

        display
            .set_orientation(mipidsi::Orientation::Landscape(true))
            .unwrap();

        TDisplayS3 {
            display,
            power,
            backlight,
            cs,
            rd,
        }
    }

    pub fn clear(&mut self) {
        self.display.clear(RgbColor::BLACK).unwrap();
    }
}
