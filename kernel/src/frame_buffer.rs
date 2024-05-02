// How does it work?
//
// Changes the Color of an given Pixel in Bitmap.....
// Not that hard.....

use bootloader_api::info::FrameBuffer;
use noto_sans_mono_bitmap::{ get_raster, get_raster_width, FontWeight, RasterHeight };

// configure noto-sans-mono-bitmap lib
pub const BACKUP_CHAR: char = '#';
pub const RASTER_HEIGHT: RasterHeight = RasterHeight::Size16;
pub const RASTER_WIDHT: usize = get_raster_width(FontWeight::Regular, RASTER_HEIGHT);
pub const FONT_HEIGHT: FontWeight = FontWeight::Regular;

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
    pos_x: usize,
    pos_y: usize,
}

impl Writer {
    
}
