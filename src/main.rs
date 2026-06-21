use crate::{bmp::BMPImage, image::Image, reader::FileReader, writer::FileWriter};
use std::{fs::File, io::{BufReader, BufWriter}};

mod reader;
mod image;
mod bmp;
mod writer;
mod byte_encode;

fn main() {
	let file = File::open("smiley.bmp").expect("File does not exist");
	let mut reader = FileReader::new(BufReader::new(file));
	let _bmp = BMPImage::read_image(&mut reader).expect("Bitmap had error");

	let out_file = File::create("tmp.bmp").expect("Cannot create fiel");
	let mut writer = FileWriter::new(BufWriter::new(out_file));
	BMPImage::write_image(&_bmp, &mut writer);
	// println!("{_bmp:?}");
}
