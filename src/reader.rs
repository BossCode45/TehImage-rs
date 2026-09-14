use std::{fs::File, io::{BufReader, Read}};

use crate::byte_encode::ByteEncode;

pub struct FileReader
{
	buf_reader: BufReader<File>,
	le: bool,
	checksum_func: Option<fn(&mut u32, &[u8]) -> ()>,
	curr_checksum: u32,
	checksum_start: u32,
}

impl FileReader {
    pub fn new(buf_reader: BufReader<File>) -> Self
	{
        Self
		{
			buf_reader,
			le: true,
			checksum_func: None,
			curr_checksum: 0,
			checksum_start: 0
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

	pub fn get_bytes(&mut self, n: usize) -> Vec<u8>
	{
		let mut buff = vec![0u8; n];
		self.buf_reader.read_exact(&mut buff).expect("get_bytes failed to read");
		if let Some(ref f) = self.checksum_func
		{
			f(&mut self.curr_checksum, buff.as_slice());
		}
		buff
	}

	pub fn read_bytes<const N: usize>(&mut self) -> [u8; N]
	{
		let mut data = [0u8; N];
		self.buf_reader.read_exact(&mut data).expect("Bad read");
		if let Some(ref f) = self.checksum_func
		{
			f(&mut self.curr_checksum, &data);
		}
		data
	}

	pub fn read<const N: usize, T: ByteEncode<N>>(&mut self) -> T
	{
		let mut data = self.read_bytes::<N>();
		if self.le
		{
			T::from_le_bytes(&data)
		}
		else
		{
			T::from_be_bytes(&mut data)
		}
	}

	pub fn set_endianness(&mut self, le: bool)
	{
		self.le = le;
	}

	pub fn set_checksum_handler(&mut self, checksum_func: fn(&mut u32, &[u8]) -> (), checksum_start: u32)
	{
		self.checksum_func = Some(checksum_func);
		self.checksum_start = checksum_start;
		self.curr_checksum = checksum_start;
	}

	pub fn reset_checksum(&mut self) -> u32
	{
		let old_checksum = self.curr_checksum.clone();
		self.curr_checksum = self.checksum_start.clone();
		old_checksum
	}

	pub fn get_checksum(&self) -> u32
	{
		self.curr_checksum
	}
}
