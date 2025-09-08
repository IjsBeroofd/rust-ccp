use bytes::{BufMut, BytesMut};

pub fn encode_frame(ver: u8, flags: u8, typ: u16, seq: u64, payload: &[u8]) -> BytesMut {
    let mut buf = BytesMut::with_capacity(16 + payload.len());
    buf.put_u8(ver);
    buf.put_u8(flags);
    buf.put_u16(typ);
    buf.put_u32(payload.len() as u32);
    buf.put_u64(seq);
    buf.put_slice(payload);
    buf
}