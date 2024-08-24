use crate::algorithms::cheetah::BYTE_SIZE_U64;
use crate::signature::Signature;

pub struct EncodeBuffer<'a> {
    pub buffer: &'a mut [u8],
    pub index: usize,
}

impl<'a> EncodeBuffer<'a> {
    pub fn new(buffer: &'a mut [u8], index: usize) -> Self {
        EncodeBuffer {
            buffer,
            index,
        }
    }

    #[inline(always)]
    pub fn write_at(&mut self, index: usize, bytes: &[u8]) -> usize {
        let future_index = index + bytes.len();
        self.buffer[index..future_index].copy_from_slice(bytes);
        future_index
    }

    #[inline(always)]
    pub fn ink(&mut self, signature: &mut Signature) {
        self.write_at(signature.pos, &signature.value.to_le_bytes());
        signature.init(self.index);
        self.skip(BYTE_SIZE_U64);
    }

    #[inline(always)]
    pub fn push(&mut self, bytes: &[u8]) {
        self.index = self.write_at(self.index, bytes);
    }

    #[inline(always)]
    pub fn skip(&mut self, length: usize) {
        self.index += length;
    }

    #[inline(always)]
    pub fn rewind(&mut self, length: usize) {
        self.index -= length;
    }
}