pub fn bzp_write_to_array(val: i32, n: i32, data: &mut BzpOutComdata) {
    while data.n_buf >= BZP_BITS8 {
        data.out[data.num] = (data.buf >> BZP_BITS24) as u8;
        data.num += 1;
        data.n_buf -= BZP_BITS8;
        data.buf <<= BZP_BITS8;
    }
    data.buf |= val << (BZP_BITS32 - n - data.n_buf);
    data.n_buf += n;
}
