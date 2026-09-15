const MAX_BITS: usize = 15;
const CODE_TREE_LEN: usize = 288;
const DIST_TREE_LEN: usize = 32;
const CODE_LEN_TREE_LEN: usize = 19;

const CODE_LEN_ORDER: [usize; CODE_LEN_TREE_LEN] = [16, 17, 18, 0, 8, 7, 9, 6, 10, 5, 11, 4, 12, 3,
													13, 2, 14, 1, 15];

const LEN_START: [usize; 29] = [3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 15, 17, 19, 23, 27, 31, 35, 43, 51,
								59, 67, 83, 99, 115, 131, 163, 195, 227, 258];
const LEN_EXTRA: [usize; 29] = [0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4,
								4, 5, 5, 5, 5, 0];

const DIST_START: [usize; 30] = [1, 2, 3, 4, 5, 7, 9, 13, 17, 25, 33, 49, 65, 97, 129, 193, 257,
								 385, 513, 769, 1025, 1537, 2049, 3073, 4097, 6145, 8193, 12289,
								 16385, 24577];
const DIST_EXTRA: [usize; 30] = [0, 0, 0, 0, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9,
								 10, 10, 11, 11, 12, 12, 13, 13];

const DYNAMIC_CODE_START: [usize; 3] = [3, 3, 11];
const DYNAMIC_CODE_EXTRA: [usize; 3] = [2, 3, 7];

#[derive(Clone, Debug)]
struct HuffmanTree<const LENGTH: usize>
{
	len_counts: [u8; MAX_BITS + 1],
	ordered_codes: [u16; LENGTH]
}

impl<const LENGTH: usize> HuffmanTree<LENGTH>
{
    const fn new(code_lens: [u8; LENGTH]) -> Self
	{
		let mut len_counts = [0u8; MAX_BITS + 1];
		let mut ordered_codes = [0u16; LENGTH];

		let mut code: usize = 0;
		while code < LENGTH
		{
			if code_lens[code] == 0
			{
				code += 1;
				continue;
			}
			len_counts[code_lens[code] as usize] += 1;
			code += 1
		}
		
		let mut offsets = [0usize; MAX_BITS + 1];
		let mut len: usize = 1;
		while len < MAX_BITS
		{
			offsets[len + 1]  = offsets[len] + len_counts[len] as usize;
			len += 1;
		}

		let mut code = 0;
		while code < LENGTH
		{
			if code_lens[code] != 0
			{
				ordered_codes[offsets[code_lens[code] as usize]] = code as u16;
				offsets[code_lens[code] as usize] += 1;
			}
			code += 1;
		}
		
		Self
		{
			len_counts,
			ordered_codes
		}
    }

	

	fn decode<T: Bitstream>(&self, bitstream: &mut T) -> u16
	{
		let mut code: u16 = 0;
		let mut first: u16 = 0;
		let mut index: usize = 0;
		
		for len in 1..(MAX_BITS + 1)
		{
			code |= bitstream.next_bit() as u16;
			let count: u16 = self.len_counts[len] as u16;
			// println!("{code} {count} {len}");
			// let a = self.len_counts;
			// println!("{a:?}");
			if count > code || code - count < first as u16
			{
				return self.ordered_codes[index + ((code - first) as usize)];
			}
			index += count as usize;
			first += count as u16;
			first <<= 1;
			code <<= 1;
		}
		
		panic!("No codes left")
	}
	
	const fn get_static_code_lens() -> [u8; CODE_TREE_LEN]
	{
		let mut code_lens = [0u8; CODE_TREE_LEN];
		let mut i = 0;
		while i < CODE_TREE_LEN
		{
			code_lens[i] = match i
			{
				0..144 => 8,
				144..256 => 9,
				256..280 => 7,
				280..CODE_TREE_LEN => 8,
				_ => panic!()
			};
			i += 1
		}
		code_lens
	}
	const fn get_static_dist_code_lens() -> [u8; DIST_TREE_LEN]
	{
		[5u8; DIST_TREE_LEN]
	}
}

static STATIC_HUFFMAN_TREE: HuffmanTree<CODE_TREE_LEN> =
	HuffmanTree::new(HuffmanTree::<CODE_TREE_LEN>::get_static_code_lens());
static STATIC_HUFFMAN_DIST_TREE: HuffmanTree<DIST_TREE_LEN> =
	HuffmanTree::new(HuffmanTree::<DIST_TREE_LEN>::get_static_dist_code_lens());

#[derive(PartialEq, Debug)]
enum CompressionType
{
	None = 0,
	StaticCodes = 1,
	DynamicCodes = 2,
	Reserved = 3,
}

impl CompressionType
{
	fn new(x: u8) -> Self
	{
		match x
		{
			0 => Self::None,
			1 => Self::StaticCodes,
			2 => Self::DynamicCodes,
			3 => Self::Reserved,
			_ => panic!("Compression type should not be greater than 4")
		}
	}
}

pub struct BitBuffer<'a>
{
	buffer: &'a [u8],
	pos: usize,
}

impl<'a> BitBuffer<'a>
{
	pub fn new(buffer: &'a [u8]) -> Self
	{
		Self
		{
			buffer,
			pos: 0
		}
	}
}

pub trait Bitstream
{
	fn next_bit(&mut self) -> bool;

	fn next_bits(&mut self, c: usize) -> u16
	{
		if c > 16
		{
			panic!("Cannot fetch more than 16 bits at a time");
		}
		let mut bits = 0u16;

		for i in 0..c
		{
			bits |= (self.next_bit() as u16) << i;
		}
		bits
	}

	fn skip_to_byte_boundary(&mut self);
}

impl<'a> Bitstream for BitBuffer<'a>
{
	fn next_bit(&mut self) -> bool
	{
		let bit = (self.buffer[self.pos/8 as usize] & (1 << (self.pos%8))) != 0;
		self.pos += 1;
		bit
	}

	fn skip_to_byte_boundary(&mut self)
	{
		// println!("{}", (8 - self.pos%8)%8);
		self.pos += (8 - self.pos%8)%8;
    }
}

pub fn zlib_decode<T: Bitstream>(mut instream: T) -> Vec<u8>
{
	let mut output_buffer: Vec<u8> = Vec::new();
	// let mut output_pos: usize = 0;

	let mut final_chunk = false;
	while !final_chunk
	{
		final_chunk = instream.next_bit();
		let compression_type = CompressionType::new(instream.next_bits(2) as u8);
		let t = output_buffer.len();
		// println!("New chunk at pos: {t}, compression type: {compression_type:?}");
		if compression_type == CompressionType::Reserved
		{
			panic!("Reserved compression type should not be used");
		}
		else if compression_type == CompressionType::None
		{
			instream.skip_to_byte_boundary();
			let len = instream.next_bits(16);
			let nlen = instream.next_bits(16);
			if len != !nlen
			{
				panic!("len and nlen don't match");
			}
			for _ in 0..len
			{
				output_buffer.push(instream.next_bits(8) as u8);
			}
			// todo!("No compression");
		}
		else
		{
			let (code_tree, dist_tree) =
				if compression_type == CompressionType::StaticCodes
			{
				(STATIC_HUFFMAN_TREE.clone(), STATIC_HUFFMAN_DIST_TREE.clone())
			}
			else // Dynamic codes
			{
				let hlit = (instream.next_bits(5) + 257) as usize;
				let hdist = (instream.next_bits(5) + 1) as usize;
				let hclen = (instream.next_bits(4) + 4) as usize;
				
				let mut code_len_tree_lens = [0u8; CODE_LEN_TREE_LEN];
				for i in CODE_LEN_ORDER[0..hclen].into_iter()
				{
					let x = instream.next_bits(3) as u8;
					// println!("{x}");
					code_len_tree_lens[*i] = x;
				}
				let code_len_tree = HuffmanTree::new(code_len_tree_lens);

				let mut code_dist_tree_lens = vec![0u8; hlit + hdist];

				let mut i: usize = 0;
				while i < hlit + hdist
				{
					let mut code = code_len_tree.decode(&mut instream);
					if code < 16
					{
						code_dist_tree_lens[i] = code as u8;
						i += 1;
						continue;
					}
					else if code < 19
					{
						code -= 16;
						let reps = DYNAMIC_CODE_START[code as usize]
							+ instream.next_bits(DYNAMIC_CODE_EXTRA[code as usize]) as usize;
						let mut rep = 0;
						if code == 0
						{
							rep = code_dist_tree_lens[i-1];
						}
						for _ in 0..reps
						{
							code_dist_tree_lens[i] = rep;
							i += 1;
							continue;
						}
					}
					else
					{
						panic!("Code can never be more than 19");
					}
				}

				let mut code_tree_lens = [0u8; CODE_TREE_LEN];
				code_tree_lens[0..hlit].copy_from_slice(&code_dist_tree_lens[0..hlit]);

				let mut dist_tree_lens = [0u8; DIST_TREE_LEN];
				dist_tree_lens[0..hdist].copy_from_slice(&code_dist_tree_lens[hlit..(hlit + hdist)]);

				// println!("{code_tree_lens:?}\n{dist_tree_lens:?}");
				
				(HuffmanTree::new(code_tree_lens),
				 HuffmanTree::new(dist_tree_lens))
			};

			// println!("{code_tree:?}\n{dist_tree:?}");
			

			let mut code = 0u16;
			while code != 256
			{
				code = code_tree.decode(&mut instream);
				if code < 256
				{
					output_buffer.push(code as u8);
				}
				else if code > 256
				{
					let len = LEN_START[code as usize - 257]
						+ instream.next_bits(LEN_EXTRA[code as usize - 257]) as usize;

					let dist_code = dist_tree.decode(&mut instream);
					let dist = DIST_START[dist_code as usize]
						+ instream.next_bits(DIST_EXTRA[dist_code as usize]) as usize;

					output_buffer.reserve(len);
					let out_pos = output_buffer.len();
					for i in 0..len
					{
						output_buffer.push(output_buffer[out_pos - dist + i]);
					}
				}
			}
		}
	}
	
	output_buffer
}
