use std::{any::TypeId, collections::HashMap};

use byte_encode_derive::ByteEncode;

use crate::{byte_encode::ByteEncode, crc::update_crc, image::{RGB, ColorType, Image, ImageBase, PixelArr}, reader::FileReader, zlib::{BitBuffer, zlib_decode}};

#[derive(Default)]
pub struct PNGImage
{
	IHDR: Option<IHDR>,
	IDAT: Option<IDAT>,
	IEND: Option<IEND>,
	tEXt: Option<tEXt>,
}

fn paeth_filter(a: &u8, b: &u8, c: &u8) -> u8 {
	let p = (*a as i32 + *b as i32 - *c as i32) as u8;
	let pa = p.abs_diff(*a);
	let pb = p.abs_diff(*b);
	let pc = p.abs_diff(*c);
	if pa <= pb && pa <= pc
	{
		*a
	}
	else if pb <= pc
	{
		*b
	}
	else
	{
		*c
	}
}
#[derive(Debug)]
enum FilterMethod
{
	None,
	Sub,
	Up,
	Average,
	Paeth
}
impl FilterMethod
{
	fn from_byte(x: u8) -> Self {
		match x {
			0 => Self::None,
			1 => Self::Sub,
			2 => Self::Up,
			3 => Self::Average,
			4 => Self::Paeth,
			_ => panic!("Filter {x} is invalid!\nOnly 5 filters exist")
		}
	}
	fn apply(&self, a: &u8, b: &u8, c: &u8, x: &u8) -> u8 {
		match self {
			FilterMethod::None    => *x,
			FilterMethod::Sub     => x.wrapping_add(*a),
			FilterMethod::Up      => x.wrapping_add(*b),
			FilterMethod::Average => x.wrapping_add((*a + *b)/2),
			FilterMethod::Paeth   => x.wrapping_add(paeth_filter(a, b, c)),
		}
	}
}

trait Mergable: Sized
{
	fn merge(a: Self, b: Self) -> Self;
}

impl<T: Mergable + Sized + Clone> Mergable for Option<T>
{
	fn merge(a: Option<T>, b: Option<T>) -> Option<T>
	{
		if let Some(ref a_content) = a
		{
			if let Some(b_content) = b
			{
				Some(T::merge(a_content.clone(), b_content))
			}
			else
			{
				a
			}
		}
		else
		{
			b
		}
	}
}

trait PNGChunk: Mergable
{
	fn read(reader: &mut FileReader, length: usize) -> Self;
	const CHUNK_TYPE: [char; 4];
}

#[derive(ByteEncode, Clone)]
struct IHDR
{
	pub width: u32,
	pub height: u32,
	pub bit_depth: u8,
	pub color_type: u8,
	pub compression_method: u8,
	pub filter_method: u8,
	pub interlace_method: u8
}
impl PNGChunk for IHDR
{
	const CHUNK_TYPE: [char; 4] = ['I', 'H', 'D', 'R'];
    fn read(reader: &mut FileReader, _: usize) -> Self {
		reader.read()
    }
}
impl Mergable for IHDR {
    fn merge(a: Self, b: Self) -> Self {
        todo!()
    }
}
#[derive(ByteEncode, Clone)]
struct IEND
{
}
impl PNGChunk for IEND
{
	const CHUNK_TYPE: [char; 4] = ['I', 'E', 'N', 'D'];
    fn read(_: &mut FileReader, _: usize) -> Self {
		Self {}
    }
}
impl Mergable for IEND {
    fn merge(a: Self, b: Self) -> Self {
        todo!()
    }
}

#[derive(Clone)]
struct tEXt
{
	keywords: HashMap<String, String>
}
impl PNGChunk for tEXt
{
	const CHUNK_TYPE: [char; 4] = ['t', 'E', 'X', 't'];
	fn read(reader: &mut FileReader, length: usize) -> Self
	{
		let mut i = 0;
		let mut keyword = String::new();
		let mut c: u8 = reader.read();
		i += 1;
		while c != 0
		{
			keyword.push(c as char);
			c = reader.read::<1, u8>();
			i += 1;
		}

		let mut text_string = String::new();
		while i < length
		{
			text_string.push(reader.read::<1, u8>() as char);
			i += 1;
		}

		let mut keywords = HashMap::new();
		keywords.insert(keyword, text_string);
		
		Self
		{
			keywords
		}
	}
}
impl Mergable for tEXt {
    fn merge(a: Self, b: Self) -> Self {
        let mut keywords = HashMap::new();
		keywords.extend(a.keywords);
		keywords.extend(b.keywords);
		Self
		{
			keywords
		}
    }
}

#[derive(Clone)]
struct IDAT
{
	data: Vec<u8>
}
impl PNGChunk for IDAT
{
    fn read(reader: &mut FileReader, length: usize) -> Self {
		let data = reader.get_bytes(length);
		Self
		{
			data
		}
    }

    const CHUNK_TYPE: [char; 4] = ['I', 'D', 'A', 'T'];
}
impl Mergable for IDAT
{
	fn merge(a: Self, b: Self) -> Self {
        let mut data = Vec::new();
		data.extend(a.data);
		data.extend(b.data);
		Self
		{
			data
		}
    }
}

trait RGBu8: ColorType {}
impl RGBu8 for RGB<u8> {}

impl<T: RGBu8> Image<T> for PNGImage
	where T: 'static
{
    fn read_image(reader: &mut FileReader) -> Result<ImageBase<T>, String> {
		assert!(
			TypeId::of::<T>() == TypeId::of::<RGB<u8>>(),
			"PNG only working for rgb u8's currently sorry"
		);
		let magic: [u8; 8] = reader.read_array();
		if magic != [0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A]
		{
			return Err("Not a PNG!".to_owned());
		}

		reader.set_endianness(false);
		reader.set_checksum_handler(update_crc, 0xffffffffu32);

		let mut p = PNGImage::default();

		while p.IEND.is_none()
		{
			let length: usize = reader.read::<4, u32>() as usize;
			reader.reset_checksum();
			let chunk_type: [char; 4] = reader.read_array::<1, u8, 4>().map(|x| x as char);
			let chunk_flags = chunk_type.map(|x| x.is_lowercase());
			println!("type: {chunk_type:?}");
			println!("length: {length}");
			match chunk_type
			{
				IHDR::CHUNK_TYPE => { p.IHDR = Option::<IHDR>::merge(p.IHDR, Some(IHDR::read(reader, length))) }
				IDAT::CHUNK_TYPE => { p.IDAT = Option::<IDAT>::merge(p.IDAT, Some(IDAT::read(reader, length))) }
				IEND::CHUNK_TYPE => { p.IEND = Option::<IEND>::merge(p.IEND, Some(IEND::read(reader, length))) }
				tEXt::CHUNK_TYPE => { p.tEXt = Option::<tEXt>::merge(p.tEXt, Some(tEXt::read(reader, length))) }
				_ => {
					if chunk_flags[0]
					{
						println!("Skipping ancillary chunk");
						println!("{}", if chunk_flags[1] { "Private (can find definition)" } else { "Public" })
					}
					else
					{
						println!("Skipping important chunk!!!");
					}
					reader.skip(length)
				}
			}
			let mut calculated_crc = reader.get_checksum();
			let crc: u32 = reader.read();
			if crc ^ calculated_crc != 0xffffffffu32 && !chunk_flags[0]
			{
				calculated_crc ^= 0xffffffffu32;
				println!("Bad crc: {crc} {calculated_crc}");
			}
		}

		if let Some(ref tEXt) = p.tEXt
		{
			for x in tEXt.keywords.iter()
			{
				println!("{0}: {1}", x.0, x.1);
			}
		}

		if p.IEND.is_none()
		{
			return Err("IEND not present".to_owned());
		}

		let Some(IHDR) = p.IHDR else { return Err("IHDR not present".to_owned()); };
		let Some(IDAT) = p.IDAT else { return Err("IDAT not present".to_owned()); };

		let colors_per_pixel: u8 = if IHDR.color_type == 4 || IHDR.color_type == 6 { 4 } else { 3 };
		let bpp: u8 = IHDR.bit_depth * colors_per_pixel;
		let width: u32 = IHDR.width;
		let height: u32 = IHDR.height;
		println!("{bpp}, {width}, {height}");

		let _compression_method: u8 = IDAT.data[0];
		let _additional_flags: u8 = IDAT.data[1];
		
		let decoded = zlib_decode(BitBuffer::new(&IDAT.data[2..]));

		let mut pixels = PixelArr::<T>::new(width, height);
		let mut rows: Vec<Vec<u8>> = Vec::new();

		let mut i = 0;
		for y in 0..(height as usize)
		{
			let filter = FilterMethod::from_byte(decoded[i]);
			i += 1;
			let mut row: Vec<u8> = Vec::with_capacity(width as usize * 3);
			for x in 0..(width as usize * 3)
			{
				let a: &u8 = if x < 3 { &0 } else { &row[x - 3] };
				let b: &u8 = if y < 1 { &0 } else { &rows[y - 1][x] };
				let c: &u8 = if y < 1 || x < 3 { &0 } else { &rows[y - 1][x - 3] };
				let x = &decoded[i];
				i += 1;
				row.push(filter.apply(a, b, c, x));
			}
			rows.push(row);
		}

		for y in 0..(height as usize)
		{
			for x in 0..(width as usize)
			{
				let row = &rows[y];
				let r = row[x * 3];
				let g = row[x * 3 + 1];
				let b = row[x * 3 + 2];
				pixels[(x, y)] = (RGB { r, g, b}).convert();
			}
		}
		
		Ok(ImageBase
		{
			bpp,
			pixels
		})
    }

    fn write_image(image: &crate::image::ImageBase<T>, writer: &mut crate::writer::FileWriter)-> () {
        todo!()
    }
}
