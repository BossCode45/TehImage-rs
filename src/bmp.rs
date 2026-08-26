use crate::{byte_encode::ByteEncode, image::{ColorType, Image, ImageBase, PixelArr, RGB, RGBA}, reader::FileReader, writer::FileWriter};

// extern crate byte_encode_derive;
use byte_encode_derive::ByteEncode;


trait BitmapHeader
{
	fn get_width(&self) -> u32;
	fn get_height(&self) -> u32;
	fn get_bpp(&self) -> u8;
}

#[derive(Clone, ByteEncode)]
struct BITMAPINFOHEADER
{
	width: u32,
	height: u32,
	_color_planes: u16,
	bpp: u16,
	compression_method: u32,
	_image_size: u32,
	_horr_res: u32,
	_vert_res: u32,
	_palette_size: u32,
	_important_colors: u32
}

impl BitmapHeader for BITMAPINFOHEADER
{
    fn get_width(&self) -> u32 {
        self.width
    }
    fn get_height(&self) -> u32 {
        self.height
    }
    fn get_bpp(&self) -> u8 {
        self.bpp as u8
    }
}


impl<T: ColorType> Image<T> for BMPImage
{
	fn read_image(reader: &mut FileReader) -> Result<ImageBase<T>, String>
	{
		let magic: [u8; 2] = reader.read_array();
		if magic != [0x42, 0x4d]
		{
			return Err("Not a bitmap!".to_owned());
		}
		
		let _file_size: u32 = reader.read();
		reader.skip(4);
		let _offset: u32 = reader.read();
		
		let header_size: u32 = reader.read();

		// println!("File header: {file_size} {offset} {header_size}");

		let width: u32;
		let height: u32;
		let bpp: u8;

		
		if header_size == 12 // BITMAPCOREHEADER
		{
			width = reader.read::<2, u16>() as u32;
			height = reader.read::<2, u16>() as u32;
			let _color_planes: u16 = reader.read();
			bpp = reader.read::<2, u16>() as u8;
		}
		else if header_size == 40 // BITMAPINFOHEADER
		{
			let header: BITMAPINFOHEADER = reader.read();
			width = header.get_width();
			height = header.get_height();
			bpp = header.get_bpp();

			if header.compression_method != 0
			{
				return Err(format!("Bitmap compresssion method {0} not supported", header.compression_method));
			}
		}
		else
		{
			return Err(format!("Bitmap header size {header_size} not supported yet"));
		}

		if bpp != 24
		{
			return Err(format!("bpp {bpp} not supported yet, only 24 is supported"));
		}

		let row_size = (((bpp as u32) * width + 31)/32) * 4;
		let skip: usize = (row_size - width * 3) as usize;
		let mut pixel_arr = PixelArr::new(width, height);
		for y in (0..height).rev()
		{
			for x in 0..width
			{
				let mut pixel = RGBA::<u8>::DEFAULT;
				pixel.b = reader.read();
				pixel.g = reader.read();
				pixel.r = reader.read();
				pixel.a = 255;
				pixel_arr[(x as usize, y as usize)] = pixel.convert();
			}
			reader.skip(skip);
		}

		return Ok(ImageBase
				  {
					  bpp,
					  pixels: pixel_arr,
				  })
	}

	fn write_image(image: &ImageBase<T>, writer: &mut FileWriter) -> () {
		writer.write_array([0x42, 0x4d] as [u8; 2]);

		let row_size = (((24) * image.pixels.width + 31)/32) * 4;
		let file_size: u32 = 14 + 12 + row_size*image.pixels.height;
		let offset: u32 = 26;
		writer.write(file_size as u32);
		writer.write_zeros(4);
		writer.write(offset as u32);
		writer.write(12 as u32); // Header size

		writer.write(image.pixels.width as u16);
		writer.write(image.pixels.height as u16);
		writer.write(1 as u16); // Color planes
		writer.write(24 as u16); // bpp

		let skip: usize = (row_size - image.pixels.width * 3) as usize;
		for y in (0..image.pixels.height).rev()
		{
			for x in 0..image.pixels.width
			{
				let pixel: RGB<u8> = image.pixels[(x as usize, y as usize)].clone().convert();
				writer.write(pixel.b);
				writer.write(pixel.g);
				writer.write(pixel.r);
			}
			writer.write_zeros(skip);
		}
		writer.flush();
	}
}


pub struct BMPImage
{
}
