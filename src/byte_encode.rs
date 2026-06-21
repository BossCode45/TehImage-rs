pub trait ByteEncode<const N: usize>: Sized + Copy
{
	fn from_le_bytes(buffer: &[u8; N]) -> Self;
	fn to_le_bytes(&self) -> [u8; N];
}

impl ByteEncode<4> for u32
{
    fn from_le_bytes(buffer: &[u8; 4]) -> Self {
		u32::from_le_bytes(*buffer)
    }
    fn to_le_bytes(&self) -> [u8; 4] {
        u32::to_le_bytes(*self)
    }
}
impl ByteEncode<2> for u16
{
    fn from_le_bytes(buffer: &[u8; 2]) -> Self {
		u16::from_le_bytes(*buffer)
    }
    fn to_le_bytes(&self) -> [u8; 2] {
        u16::to_le_bytes(*self)
    }
}
impl ByteEncode<1> for u8
{
    fn from_le_bytes(buffer: &[u8; 1]) -> Self {
		u8::from_le_bytes(*buffer)
    }
    fn to_le_bytes(&self) -> [u8; 1] {
        u8::to_le_bytes(*self)
    }
}
