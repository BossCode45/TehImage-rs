use std::io::Write;
use std::{fs::File, io::BufWriter};

use crate::byte_encode::ByteEncode;

pub struct FileWriter
{
	buf_writer: BufWriter<File>
}

impl FileWriter
{
	pub fn new(buf_writer: BufWriter<File>) -> Self
	{
		Self
		{
			buf_writer
		}
	}

	pub fn write_zeros(&mut self, c: usize)
	{
		for _ in 0..c
		{
			self.write(0 as u8);
		}
		// self.buf_writer.write_all(&[0u8; c]).expect("Could not write 0s");
	}

	pub fn write_array<const N: usize, T: ByteEncode<N>, const C: usize>(&mut self, xs: [T; C])
	{
		for x in xs
		{
			self.write(x);
		}
	}
	pub fn write<const N: usize, T: ByteEncode<N>>(&mut self, x: T)
	{
		let data = x.to_le_bytes();
		self.buf_writer.write_all(&data).expect("Could not write");
	}
	
	pub fn flush(&mut self)
	{
		self.buf_writer.flush().expect("Could not flush");
	}
}
