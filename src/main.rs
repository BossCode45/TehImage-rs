use crate::{bmp::BMPImage, png::PNGImage, image::{Image, ImageBase, RGB}, reader::FileReader, writer::FileWriter};
use std::{env, fs::File, io::{BufReader, BufWriter}};

mod reader;
mod image;
mod bmp;
mod writer;
mod byte_encode;
mod png;
mod crc;
mod zlib;

fn main() {
	let args: Vec<String> = env::args().collect();
	if args.len() != 3
	{
		panic!("USAGE: <program> infline.png outfile.bmp");
	}
	
	let infile = File::open(args[1].clone()).expect("infile does not exist");
	let mut reader = FileReader::new(BufReader::new(infile));
	let png: ImageBase<RGB<u8>> = PNGImage::read_image(&mut reader).expect("PNG had error");

	let out_file = File::create(args[2].clone()).expect("Cannot create outfile");
	let mut writer = FileWriter::new(BufWriter::new(out_file));
	BMPImage::write_image(&png, &mut writer);
	// println!("{_bmp:?}");
}
