pub fn bzp_write_int32(val: i32, data: &mut BzpOutComdata) {
    bzp_write_to_array((val >> BZP_BITS24) & 0xff, BZP_BITS8, data);
    bzp_write_to_array((val >> BZP_BITS16) & 0xff, BZP_BITS8, data);
    bzp_write_to_array((val >> BZP_BITS8) & 0xff, BZP_BITS8, data);
    bzp_write_to_array(val & 0xff, BZP_BITS8, data);
}
