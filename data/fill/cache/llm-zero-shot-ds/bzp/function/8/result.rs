pub fn bzp_read_uint24(in_data: &mut InDeComdata) -> u32 {
    let mut val: u32 = 0;
    let ch = bzp_read_bits(BZP_BITS8, in_data);
    val = (val << BZP_BITS8) | (ch as u32);
    let ch = bzp_read_bits(BZP_BITS8, in_data);
    val = (val << BZP_BITS8) | (ch as u32);
    let ch = bzp_read_bits(BZP_BITS8, in_data);
    val = (val << BZP_BITS8) | (ch as u32);
    val
}
