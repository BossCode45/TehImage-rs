use std::{fs::File, io::{BufReader, Read}};

pub struct FileReader<const BUFF_SIZE: usize = 1024>
{
	buf_reader: BufReader<File>
}

impl<const BUFF_SIZE: usize> FileReader<BUFF_SIZE> {
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
	pub fn read_array<const N: usize, T: FromBytes<N> + Sized, const C: usize>(&mut self) -> [T; C]
	{
		std::array::from_fn(|_| self.read())
	}

	pub fn read<const N: usize, T: FromBytes<N> + Sized>(&mut self) -> T
	{
		let mut data = [0u8; N];
		self.buf_reader.read_exact(&mut data).expect("Bad read");
		T::from_le_bytes(&data)
	}
}

pub trait FromBytes<const N: usize>: Sized
{
	fn from_le_bytes(buffer: &[u8; N]) -> Self;
}

impl FromBytes<4> for u32
{
    fn from_le_bytes(buffer: &[u8; 4]) -> Self {
		u32::from_le_bytes(*buffer)
    }
}
impl FromBytes<2> for u16
{
    fn from_le_bytes(buffer: &[u8; 2]) -> Self {
		u16::from_le_bytes(*buffer)
    }
}
impl FromBytes<1> for u8
{
    fn from_le_bytes(buffer: &[u8; 1]) -> Self {
		u8::from_le_bytes(*buffer)
    }
}
