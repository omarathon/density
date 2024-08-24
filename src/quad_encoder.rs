use crate::encode_buffer::EncodeBuffer;
use crate::signature::Signature;

pub trait QuadEncoder {
    fn encode_quad(&mut self, quad: u32, buffer: &mut EncodeBuffer, signature: &mut Signature);
}