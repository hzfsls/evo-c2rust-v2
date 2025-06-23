pub fn bzp_calculate_crc(bwt: &mut BzpBwtInfo) {
    bwt.block_crc = !bwt.block_crc;
    bwt.combined_crc = (bwt.combined_crc << 1) | (bwt.combined_crc >> BZP_CRC_MOVE_RIGHT_VAL);
    bwt.combined_crc ^= bwt.block_crc;
}
