use std::ops;

use crate::{reader::FileReader, writer::FileWriter};

type FullColorDepth = u32;
pub trait ColorDepth: Clone
{
	const DEFAULT: Self;
	fn from_full_color(x: FullColorDepth) -> Self;
	fn to_full_color(&self) -> FullColorDepth;
	fn convert<T: ColorDepth>(&self) -> T
	{
		T::from_full_color(self.to_full_color())
	}
}
impl ColorDepth for u8 {
    const DEFAULT: Self = 0;
    fn from_full_color(x: FullColorDepth) -> Self {(x>>24) as Self}
    fn to_full_color(&self) -> FullColorDepth { (*self as FullColorDepth) << 24 }
}
impl ColorDepth for u16 {
    const DEFAULT: Self = 0;
    fn from_full_color(x: FullColorDepth) -> Self {(x>>16) as Self}
	fn to_full_color(&self) -> FullColorDepth { (*self as FullColorDepth) << 16 }
}
impl ColorDepth for u32 {
    const DEFAULT: Self = 0;
    fn from_full_color(x: FullColorDepth) -> Self {x}
	fn to_full_color(&self) -> FullColorDepth { *self }
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

type FullColorPixel = RGBA<FullColorDepth>;
pub trait ColorType: Clone
{
	const DEFAULT: Self;
	fn from_full_color(x: FullColorPixel) -> Self;
	fn to_full_color(&self) -> FullColorPixel;
	fn convert<T: ColorType>(&self) -> T
	{
		T::from_full_color(self.to_full_color())
	}
}
impl<T: ColorDepth> ColorType for RGBA<T>
{
	const DEFAULT: Self = Self { r: T::DEFAULT, g: T::DEFAULT, b: T::DEFAULT, a: T::DEFAULT };
	fn from_full_color(x: FullColorPixel) -> Self {
		Self
		{
			r: T::from_full_color(x.r),
			g: T::from_full_color(x.g),
			b: T::from_full_color(x.b),
			a: T::from_full_color(x.a),
		}
	}
	fn to_full_color(&self) -> FullColorPixel {
		RGBA {
			r: T::to_full_color(&self.r),
			g: T::to_full_color(&self.g),
			b: T::to_full_color(&self.b),
			a: T::to_full_color(&self.a),
		}
    }
}
impl<T: ColorDepth> ColorType for RGB<T>
{
	const DEFAULT: Self = Self { r: T::DEFAULT, g: T::DEFAULT, b: T::DEFAULT};
	fn from_full_color(x: FullColorPixel) -> Self {
		Self
		{
			r: T::from_full_color(x.r),
			g: T::from_full_color(x.g),
			b: T::from_full_color(x.b)
		}
	}
	fn to_full_color(&self) -> FullColorPixel {
		RGBA {
			r: T::to_full_color(&self.r),
			g: T::to_full_color(&self.g),
			b: T::to_full_color(&self.b),
			a: u32::max_value()
		}
    }
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
pub struct ImageBase<T: ColorType>
{
	pub bit_depth: u8,
	pub pixels: PixelArr<T>
}

pub trait Image<T: ColorType>
{
	fn read_image(reader: &mut FileReader) -> Result<ImageBase<T>, &'static str>;
	fn write_image(image: &ImageBase<T>, writer: &mut FileWriter)-> ();
}
