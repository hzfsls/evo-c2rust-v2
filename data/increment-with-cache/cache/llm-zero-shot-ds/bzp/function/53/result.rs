pub fn bzp_read_u32(in_data: &mut InDeComdata) -> u32 {
    let mut val: u32 = 0;
    let mut ch = bzp_read_bits(BZP_BITS8, in_data);
    val = (val << BZP_BITS8) | (ch as u32);
    ch = bzp_read_bits(BZP_BITS8, in_data);
    val = (val << BZP_BITS8) | (ch as u32);
    ch = bzp_read_bits(BZP_BITS8, in_data);
    val = (val << BZP_BITS8) | (ch as u32);
    ch = bzp_read_bits(BZP_BITS8, in_data);
    val = (val << BZP_BITS8) | (ch as u32);
    val
}
