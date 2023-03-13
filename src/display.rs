use std::collections::VecDeque;
use std::str::FromStr;

use display_interface_parallel_gpio::Generic8BitBus;
use display_interface_parallel_gpio::PGPIO8BitInterface;
use embedded_graphics::geometry::AnchorPoint;
use embedded_graphics::mock_display::ColorMapping;
use embedded_graphics::mono_font::ascii::FONT_10X20;
use embedded_graphics::mono_font::ascii::FONT_6X10;
use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::Dimensions;
use embedded_graphics::prelude::DrawTarget;
use embedded_graphics::prelude::Point;
use embedded_graphics::prelude::RgbColor;
use embedded_graphics::prelude::Size;
use embedded_graphics::primitives::Primitive;
use embedded_graphics::primitives::PrimitiveStyle;
use embedded_graphics::primitives::PrimitiveStyleBuilder;
use embedded_graphics::primitives::Rectangle;
use embedded_graphics::primitives::Styled;
use embedded_graphics::text::Alignment;
use embedded_graphics::text::Text;
use embedded_graphics::Drawable;
// use embedded_graphics::Drawable;
use embedded_text::alignment::HorizontalAlignment;
use embedded_text::alignment::VerticalAlignment;
use embedded_text::plugin::NoPlugin;
use embedded_text::style::HeightMode;
use embedded_text::style::TextBoxStyle;
use embedded_text::style::TextBoxStyleBuilder;
use embedded_text::TextBox;
use esp_idf_hal::delay;
use esp_idf_hal::gpio;
use esp_idf_hal::gpio::AnyOutputPin;
use esp_idf_hal::gpio::Output;
use esp_idf_hal::gpio::PinDriver;
use log::info;
use mipidsi::models::ST7789;
use mipidsi::Display;
use ringbuf::HeapRb;
use ringbuf::Rb;

const LCD_WIDTH: u16 = 170; // width
const LCD_HIGHT: u16 = 320; // height

//Screen is horizontal flip dimensions ;-)
pub const SCREEN_WIDTH: u32 = LCD_HIGHT as u32;
pub const SCREEN_HIGHT: u32 = LCD_WIDTH as u32;
pub const SCREEN_BOUNDS: Rectangle = Rectangle::new(
    Point { x: 0, y: 0 },
    Size {
        width: SCREEN_WIDTH,
        height: SCREEN_HIGHT,
    },
);

type DisplayI8080St7789 = Display<
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
>;

type ColType = Rgb565;
pub struct TDisplayS3 {
    pub screen: DisplayI8080St7789,
    power: PinDriver<'static, AnyOutputPin, Output>,
    backlight: PinDriver<'static, AnyOutputPin, Output>,
    cs: PinDriver<'static, AnyOutputPin, Output>,
    rd: PinDriver<'static, AnyOutputPin, Output>,
}

pub struct OutputBusPins8Bit {
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

        TDisplayS3 {
            screen: mipidsi::Builder::st7789(di)
                .with_display_size(LCD_WIDTH, LCD_HIGHT)
                .with_orientation(mipidsi::Orientation::Landscape(true))
                .with_window_offset_handler(|_| -> (u16, u16) { (0, 35) })
                .init(&mut delay::Ets, Some(gpio::PinDriver::output(rst).unwrap()))
                .unwrap(),
            power: gpio::PinDriver::output(power).unwrap(),
            backlight: gpio::PinDriver::output(backlight).unwrap(),
            cs,
            rd,
        }
    }

    pub fn clear(&mut self, color: ColType) {
        self.screen.clear(color).unwrap();
    }

    // pub fn log(logMsg: &str){

    // }
}

// pub struct LayoutManager {
//     display: TDisplayS3,
//     // log: VecDeque<&'a str>,
// }

// impl LayoutManager {
//     pub fn new(display: &'a mut TDisplayS3) -> Self {
//         LayoutManager {
//             display, // log: VecDeque::new(),
//         }
//     }

//     pub fn (&mut self, txt: &'b str, txb: &'b mut FramedTextBox<'b>) {
//         txb.frame.draw(&mut self.display.screen).unwrap();
//         txb.text_box.text = txt;
//         txb.text_box.draw(&mut self.display.screen).unwrap();
//     }
// }

// trait BufferedTextPrinter{
//     // fn
// }

pub struct FramedTextBox<'a> {
    pub frame: Styled<Rectangle, PrimitiveStyle<Rgb565>>,
    pub text_box: TextBox<'a, MonoTextStyle<'a, Rgb565>, NoPlugin<Rgb565>>,
}

pub enum FramedTextBoxAnchor {
    Left,
    Right,
    Up,
    Down,
}

pub struct FramedTextBoxBuilder {
    frame: Rectangle,
    frame_thickness: u32,
    frame_color: ColType,
    txt_color: ColType,
    bg_color: ColType,
    alignment: HorizontalAlignment,
    alignment_vertical: VerticalAlignment,
}

impl FramedTextBoxBuilder {
    pub fn new(frame: Rectangle) -> FramedTextBoxBuilder {
        FramedTextBoxBuilder {
            frame,
            frame_thickness: 2,
            frame_color: RgbColor::RED,
            txt_color: RgbColor::WHITE,
            bg_color: RgbColor::BLACK,
            alignment: HorizontalAlignment::Left,
            alignment_vertical: VerticalAlignment::Top,
        }
    }

    pub fn alignment_vertical(
        &mut self,
        alignment_vertical: VerticalAlignment,
    ) -> &mut FramedTextBoxBuilder {
        self.alignment_vertical = alignment_vertical;
        self
    }

    pub fn alignment(&mut self, alignment: HorizontalAlignment) -> &mut FramedTextBoxBuilder {
        self.alignment = alignment;
        self
    }
    pub fn frame_spacing(&mut self, frame_spacing: u32) -> &mut FramedTextBoxBuilder {
        self.frame_thickness = frame_spacing;
        self
    }

    pub fn frame_color(&mut self, frame_color: ColType) -> &mut FramedTextBoxBuilder {
        self.frame_color = frame_color;
        self
    }

    pub fn txt_color(&mut self, txt_color: ColType) -> &mut FramedTextBoxBuilder {
        self.txt_color = txt_color;
        self
    }

    pub fn bg_color(&mut self, bg_color: ColType) -> &mut FramedTextBoxBuilder {
        self.bg_color = bg_color;
        self
    }

    pub fn copy_relative_to(
        from: &FramedTextBox,
        anchor: FramedTextBoxAnchor,
        spacing: i32,
    ) -> FramedTextBoxBuilder {
        FramedTextBoxBuilder::new_relative_to(from, anchor, spacing, from.frame.bounding_box().size)
    }

    pub fn new_relative_to(
        from: &FramedTextBox,
        anchor: FramedTextBoxAnchor,
        spacing: i32,
        size: Size,
    ) -> FramedTextBoxBuilder {
        let bb = from.frame.bounding_box();
        let frame = match anchor {
            FramedTextBoxAnchor::Left => Rectangle::new(
                Point::new(bb.top_left.x - size.width as i32 - spacing, bb.top_left.y),
                size,
            ),
            FramedTextBoxAnchor::Right => Rectangle::new(
                Point::new(
                    bb.top_left.x + bb.size.width as i32 + spacing,
                    bb.top_left.y,
                ),
                size,
            ),
            FramedTextBoxAnchor::Up => Rectangle::new(
                Point::new(bb.top_left.x, bb.top_left.y - spacing - size.height as i32),
                size,
            ),
            FramedTextBoxAnchor::Down => Rectangle::new(
                Point::new(
                    bb.top_left.x,
                    bb.top_left.y + spacing + bb.size.height as i32,
                ),
                size,
            ),
        };

        FramedTextBoxBuilder::new(frame)
    }

    pub fn build<'a>(self) -> FramedTextBox<'a> {
        let character_style: MonoTextStyle<ColType> =
            MonoTextStyle::new(&FONT_10X20, self.txt_color);

        let textbox_style = TextBoxStyleBuilder::new()
            // .height_mode(HeightMode::FitToText)
            .alignment(self.alignment)
            .vertical_alignment(self.alignment_vertical)
            .paragraph_spacing(6)
            .build();

        let frame_style = PrimitiveStyleBuilder::new()
            .stroke_color(self.frame_color)
            .stroke_width(1)
            .fill_color(self.bg_color)
            .build();

        let text_field = Rectangle::new(
            Point::new(
                self.frame.bounding_box().top_left.x + self.frame_thickness as i32,
                self.frame.bounding_box().top_left.y + self.frame_thickness as i32,
            ),
            Size::new(
                self.frame.bounding_box().size.width - (self.frame_thickness * 2),
                self.frame.bounding_box().size.height - (self.frame_thickness * 2),
            ),
        );
        let text_box = TextBox::with_textbox_style("", text_field, character_style, textbox_style);

        FramedTextBox {
            frame: self.frame.into_styled(frame_style),
            text_box,
        }
    }
}

// pub struct FramedTextBox<'a> {
//     pub frame: Styled<Rectangle, PrimitiveStyle<Rgb565>>,
//     pub text_box: TextBox<'a, MonoTextStyle<'a, Rgb565>, NoPlugin<Rgb565>>,
// }


pub struct TextBoxPrinter<'a> {
    txt_field: FramedTextBox<'a>,
    str_buf: HeapRb<String>,
    line_length: usize,
    lines: usize,
    disp_str: String,
}

impl<'a> TextBoxPrinter<'a> {
    pub fn new(txt_field: FramedTextBox<'a>) -> Self {
        let char_width = txt_field.text_box.character_style.font.character_size.width
            + txt_field.text_box.character_style.font.character_spacing;
        let char_hight = txt_field
            .text_box
            .character_style
            .font
            .character_size
            .height
            + txt_field.text_box.style.paragraph_spacing;
        let bounds = txt_field.text_box.bounding_box();
        let line_length = (bounds.size.width / char_width) as usize;
        let lines = (bounds.size.height / char_hight) as usize;

        TextBoxPrinter {
            txt_field,
            str_buf: HeapRb::<String>::new(lines as usize),
            line_length,
            lines,
            disp_str: String::new(),
        }
    }

    pub fn txt(mut self, input_str: &str, display: &mut TDisplayS3) {
        let len = input_str.len();
        let mut disp_str = "> ".to_owned() + input_str;
        self.str_buf.push_overwrite(input_str.to_string());
        let mut line_budget = self.lines - (len / self.line_length) + 1;
    
        for s in self.str_buf.iter().rev() {
            if line_budget <= 0 {
                break;
            };
            disp_str.push_str(&("> ".to_owned() + s));
            disp_str.push('\n');
            line_budget -= (s.len() / self.line_length) + 1;
            if line_budget <= 0 {
                break;
            };
        }
    
        self.disp_str = disp_str;
        self.txt_field.text_box.text = "";
        self.txt_field.text_box.text = &self.disp_str;
        self.txt_field.text_box.draw(&mut display.screen).unwrap();
        self.txt_field.frame.draw(&mut display.screen).unwrap();
    }
}
