pub struct BzpOutComdata {
    pub buf: u32,
    pub n_buf: u32,
    pub num: usize,
    pub out: Vec<u8>,
}

pub const BZP_BITS8: u32 = 8;
pub const BZP_BITS24: u32 = 24;
pub const BZP_BITS32: u32 = 32;

pub fn bzp_write_to_array(val: i32, n: i32, data: &mut BzpOutComdata) {
    while data.n_buf >= BZP_BITS8 {
        data.out[data.num] = (data.buf >> BZP_BITS24) as u8;
        data.num += 1;
        data.n_buf -= BZP_BITS8;
        data.buf <<= BZP_BITS8;
    }
    data.buf |= ((val as u32) << (BZP_BITS32 - n as u32 - data.n_buf));
    data.n_buf += n as u32;
}
