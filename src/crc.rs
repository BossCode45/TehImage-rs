const CRC_TABLE: [u32; 256] = make_crc_table();

const fn make_crc_table() -> [u32; 256]
{
	let mut table =  [0u32; 256];
	let mut c: u32;
	let mut k: i32;
	let mut n: u32 = 0;
	while n < 256
	{
		c = n;
		k = 0;
		while k < 8
		{
			if c & 1 == 1
			{
				c = 0xedb88320 ^ (c >> 1);
			}
			else
			{
				c = c >> 1;
			}
			k += 1;
		}
		table[n as usize] = c;
		n += 1;
	}
	table
}

pub fn update_crc(curr_crc: &mut u32, buff: &[u8]) -> ()
{
	for x in buff
	{
		*curr_crc = CRC_TABLE[((*curr_crc as u8 ^ x) & 0xff) as usize] ^ (*curr_crc >> 8);
	}
}
