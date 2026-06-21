use std::ops;

use crate::{reader::FileReader, writer::FileWriter};

pub trait ColorDepth: Clone
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

#[derive(Debug, Clone, Copy)]
pub struct RGBA<T: ColorDepth>
{
	pub r: T,
	pub g: T,
	pub b: T,
	pub a: T
}
#[derive(Debug, Clone, Copy)]
pub struct RGB<T: ColorDepth>
{
	pub r: T,
	pub g: T,
	pub b: T,
}

pub trait ColorType: Clone
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
pub struct PixelArr<T: ColorType>
{
	pub width: u32,
	pub height: u32,
	raw: Vec<T>
}
impl<T: ColorType> PixelArr<T> {
    pub fn new(width: u32, height: u32) -> Self {
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
pub struct ImageBase
{
	pub bit_depth: u8,
	pub pixels: PixelArr<RGB<u8>>
}

pub trait Image
{
	fn read_image(reader: &mut FileReader) -> Result<ImageBase, &'static str>;
	fn write_image(image: &ImageBase, writer: &mut FileWriter)-> ();
}
