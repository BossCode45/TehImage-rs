// #![allow(dead_code)]

use crate::reader::FileReader;
use std::{fs::File, io::BufReader, ops::{self}};

mod reader;

// #[derive(Debug)]
// struct RGB<T: Add<Output = T> + Copy>
// {
// 	r: T,
// 	g: T,
// 	b: T
// }

trait ColorDepth: Clone
{
	const DEFAULT: Self;
}
impl ColorDepth for u8 {
    const DEFAULT: Self = 0;
}
impl ColorDepth for u16 {
    const DEFAULT: Self = 0;
}
impl ColorDepth for u32 {
    const DEFAULT: Self = 0;
}

#[derive(Debug, Clone)]
struct RGBA<T: ColorDepth>
{
	r: T,
	g: T,
	b: T,
	a: T
}
#[derive(Debug, Clone)]
struct RGB<T: ColorDepth>
{
	r: T,
	g: T,
	b: T,
}

trait ColorType: Clone
{
	const DEFAULT: Self;
}
impl<T: ColorDepth> ColorType for RGBA<T>
{
	const DEFAULT: Self = Self { r: T::DEFAULT, g: T::DEFAULT, b: T::DEFAULT, a: T::DEFAULT };
}
impl<T: ColorDepth> ColorType for RGB<T>
{
	const DEFAULT: Self = Self { r: T::DEFAULT, g: T::DEFAULT, b: T::DEFAULT};
}

#[derive(Debug)]
struct PixelArr<T: ColorType>
{
	width: u32,
	height: u32,
	raw: Vec<T>
}
impl<T: ColorType> PixelArr<T> {
    fn new(width: u32, height: u32) -> Self {
		let raw = vec![T::DEFAULT;(width*height) as usize];
        Self { width, height, raw }
    }
}
impl<T: ColorType> ops::Index<(usize, usize)> for PixelArr<T>
{
	type Output = T;
	
    fn index(&self, index: (usize, usize)) -> &Self::Output {
		let (x, y) = index;
		&self.raw[x + y*(self.width as usize)]
    }
}
impl<T: ColorType> ops::IndexMut<(usize, usize)> for PixelArr<T>
{	
    fn index_mut(&mut self, index: (usize, usize)) -> &mut T {
		let (x, y) = index;
		&mut self.raw[x + y*(self.width as usize)]
    }
}

#[derive(Debug)]
struct ImageBase
{
	width: u32,
	height: u32,
	bit_depth: u8,
	pixels: PixelArr<RGB<u8>>
}

struct BMPImage
{
}


trait Image
{
	fn read_image(reader: &mut FileReader) -> Result<ImageBase, &'static str>;
}

impl Image for BMPImage
{
	fn read_image(reader: &mut FileReader) -> Result<ImageBase, &'static str>
	{
		let magic: [u8; 2] = reader.read_array();
		if magic != [0x42, 0x4d]
		{
			return Err("Not a bitmap!");
		}
		
		let _file_size: u32 = reader.read();
		reader.skip(4);
		let _offset: u32 = reader.read();
		
		let header_size: u32 = reader.read();

		// println!("File header: {file_size} {offset} {header_size}");

		if header_size == 40
		{

			let width: u32 = reader.read();
			let height: u32 = reader.read();
			let _color_planes: u16 = reader.read();
			let bpp = reader.read::<2, u16>() as u8;
			let compression_method: u32 = reader.read();
			let _image_size: u32 = reader.read();
			let _horr_res: u32 = reader.read();
			let _vert_res: u32 = reader.read();
			let _color_count: u32 = reader.read();
			let _impotant_color_count: u32 = reader.read();

			if
				bpp != 24
				|| compression_method != 0
			{
				return Err("bpp or compresssion method not supported")
			}

			let row_size = (((bpp as u32) * width + 31)/32) * 4;
			let skip: usize = (row_size - width * 3) as usize;
			let mut pixel_arr = PixelArr::new(width, height);
			for y in (0..height).rev()
			{
				for x in 0..width
				{
					let mut pixel = RGB::DEFAULT;
					pixel.b = reader.read();
					pixel.g = reader.read();
					pixel.r = reader.read();
					pixel_arr[(x as usize, y as usize)] = pixel;
				}
				reader.skip(skip);
			}

			return Ok(ImageBase
					  {
						  width,
						  height,
						  bit_depth: bpp,
						  pixels: pixel_arr,
					  })
		}
		else
		{
			return Err("Header size not supported yet");
		}
		
		
	}
}

fn main() {
	let file = File::open("smiley.bmp").expect("File does not exist");
	let mut reader = FileReader::new(BufReader::new(file));
	let _bmp = BMPImage::read_image(&mut reader).expect("Bitmap had error");
	// println!("{bmp:?}");
}
