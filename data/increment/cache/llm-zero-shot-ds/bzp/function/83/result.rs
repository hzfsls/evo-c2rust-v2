pub fn bzp_flushbuf(out_data: &mut BzpOutComdata) {
    while out_data.n_buf > 0 {
        out_data.out[out_data.num] = (out_data.buf >> BZP_BITS24) as u8;
        out_data.num += 1;
        out_data.n_buf -= BZP_BITS8;
        out_data.buf <<= BZP_BITS8;
    }
}
