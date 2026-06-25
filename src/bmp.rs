use crate::{image::{ColorType, Image, ImageBase, PixelArr, RGB, RGBA}, reader::FileReader, writer::FileWriter};

impl<T: ColorType> Image<T> for BMPImage
{
	fn read_image(reader: &mut FileReader) -> Result<ImageBase<T>, &'static str>
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

		let width: u32;
		let height: u32;
		let bpp: u8;

		if header_size == 40
		{
			width = reader.read();
			height = reader.read();
			let _color_planes: u16 = reader.read();
			bpp = reader.read::<2, u16>() as u8;
			let compression_method: u32 = reader.read();
			let _image_size: u32 = reader.read();
			let _horr_res: u32 = reader.read();
			let _vert_res: u32 = reader.read();
			let _color_count: u32 = reader.read();
			let _impotant_color_count: u32 = reader.read();

			if compression_method != 0
			{
				return Err("Compresssion method not supported");
			}
		}
		else if header_size == 12
		{
			width = reader.read::<2, u16>() as u32;
			height = reader.read::<2, u16>() as u32;
			let _color_planes: u16 = reader.read();
			bpp = reader.read::<2, u16>() as u8;
		}
		else
		{
			return Err("Header size not supported yet");
		}

		if bpp != 24
		{
			return Err("Only bpp of 24 is supported so far");
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
					  bit_depth: bpp,
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
