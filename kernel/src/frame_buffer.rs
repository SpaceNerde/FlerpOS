// credits:
// - https://github.com/rust-osdev/bootloader/blob/main/common/src/framebuffer.rs 

// How does it work?
//
// Changes the Color of an given Pixel in Bitmap.....
// Not that hard.....

// TOOD
// [] fully implement Writer
//      [] Global Writer
//      [] Colors
//      [] Memory Padding
//      [] Make this shit stable
// [] implement print and println macro

use bootloader_api::info::{FrameBufferInfo, PixelFormat};
use noto_sans_mono_bitmap::{ get_raster, get_raster_width, FontWeight, RasterHeight, RasterizedChar};
use core::{ptr, fmt};
use lazy_static::lazy_static;

// Paddings
const BORDER_PADDING: usize = 1;
const LINE_SPACING: usize = 2;

// configure noto-sans-mono-bitmap lib
pub const BACKUP_CHAR: char = '#';
pub const RASTER_HEIGHT: RasterHeight = RasterHeight::Size16;
pub const RASTER_WIDTH: usize = get_raster_width(FontWeight::Regular, RASTER_HEIGHT);
pub const FONT_WEIGHT: FontWeight = FontWeight::Regular;

// Returns the raster of the given char or the raster of [`font_constants::BACKUP_CHAR`].
fn get_char_raster(c: char) -> RasterizedChar {
    fn get(c: char) -> Option<RasterizedChar> {
        get_raster(
           c,
            FONT_WEIGHT,
            RASTER_HEIGHT,
        )
    }
    get(c).unwrap_or_else(|| get(BACKUP_CHAR).expect("Should get raster of backup char."))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

pub struct Writer {
    framebuffer: &'static mut [u8],
    info: FrameBufferInfo,
    pos_x: usize,
    pos_y: usize,
}

impl Writer {
    pub fn clear(&mut self) {
        self.pos_x = BORDER_PADDING;
        self.pos_y = BORDER_PADDING;
        self.framebuffer.fill(0)
    }   

    fn newline(&mut self) {
        self.pos_y += RASTER_HEIGHT.val() + LINE_SPACING;
        self.carriage_return()
    }
    
    fn carriage_return(&mut self) {
        self.pos_x = BORDER_PADDING;
    }

    fn width(&self) -> usize {
        self.info.width
    }

    fn height(&self) -> usize {
        self.info.height
    }

    fn write_char(&mut self, c: char) {
        match c {
            '\n' => self.newline(),
            '\r' => self.carriage_return(),
            c => {
                let new_pos_x = self.pos_x + RASTER_WIDTH;
                if new_pos_x >= self.width() {
                    self.newline(); 
                }

                let new_pos_y = self.pos_y + RASTER_HEIGHT.val() + BORDER_PADDING;
                if new_pos_y >= self.height() {
                    self.clear();
                }

                self.render_char(get_char_raster(c));
            }

        }

    }

    fn render_char(&mut self, raster_char: RasterizedChar) {
        // Get the pixel pos of Rasterized Char and Render to Screen with Char Spacing between
        // . . .. .  .  ...
        // ... .  .  .  . .
        // . . .. .. .. ...

        for (y, row) in raster_char.raster().iter().enumerate() {
            for (x, byte) in row.iter().enumerate() {
                self.set_pixel(self.pos_x + x, self.pos_y + y, *byte);
            }
        }
        self.pos_x += raster_char.width();
    }

    // uses pixel position and opacity to place pixel into framebuffer
    // uses bytes_per_pixel and into_bytes_per_pixel to convert pixel with set color
    // into byte code which get written into framebuffer
    fn set_pixel(&mut self, x: usize, y: usize, opacity: u8) {
        let pixel_offset = y * self.info.stride + x;
        // color uses RGB values from 0 - 255 and opacity 
        let color = match self.info.pixel_format {
            PixelFormat::Rgb => [opacity, opacity, opacity / 2, 0],
            other => {
                // avoid double panic
                self.info.pixel_format = PixelFormat::Rgb;
                panic!("pixel format {:?} not supported", other)
            }
        };
        let bytes_per_pixel = self.info.bytes_per_pixel;
        let byte_offset = pixel_offset * bytes_per_pixel;

        self.framebuffer[byte_offset..(byte_offset + bytes_per_pixel)].copy_from_slice(&color[..bytes_per_pixel]);
        let _ = unsafe { ptr::read_volatile(&self.framebuffer[byte_offset]) };
    }  
}


impl fmt::Writer for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            self.write_char(c);
        }
        Ok(())
    }
}


