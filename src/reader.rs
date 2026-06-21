use std::{fs::File, io::{BufReader, Read}};

use crate::byte_encode::ByteEncode;

pub struct FileReader
{
	buf_reader: BufReader<File>
}

impl FileReader {
    pub fn new(buf_reader: BufReader<File>) -> Self
	{
        Self
		{
			buf_reader
		}
    }

	pub fn skip(&mut self, c: usize)
	{
		self.buf_reader.seek_relative(c as i64).expect("Cannot seek!");
	}
	pub fn read_array<const N: usize, T: ByteEncode<N> + Sized, const C: usize>(&mut self) -> [T; C]
	{
		std::array::from_fn(|_| self.read())
	}

	pub fn read<const N: usize, T: ByteEncode<N>>(&mut self) -> T
	{
		let mut data = [0u8; N];
		self.buf_reader.read_exact(&mut data).expect("Bad read");
		T::from_le_bytes(&data)
	}
}
