use crate::{bmp::BMPImage, image::{Image, ImageBase, RGB}, reader::FileReader, writer::FileWriter};
use std::{env, fs::File, io::{BufReader, BufWriter}};

mod reader;
mod image;
mod bmp;
mod writer;
mod byte_encode;

fn main() {
	let args: Vec<String> = env::args().collect();
	if args.len() != 3
	{
		panic!("USAGE: <program> infline.bmp outfile.bmp");
	}
	
	let file = File::open(args[1].clone()).expect("File does not exist");
	let mut reader = FileReader::new(BufReader::new(file));
	let _bmp: ImageBase<RGB<u8>> = BMPImage::read_image(&mut reader).expect("Bitmap had error");

	let out_file = File::create(args[2].clone()).expect("Cannot create file");
	let mut writer = FileWriter::new(BufWriter::new(out_file));
	BMPImage::write_image(&_bmp, &mut writer);
	// println!("{_bmp:?}");
}
