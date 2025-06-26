pub fn bzp_calculate_crc(bwt: &mut BzpBwtInfo) {
    bwt.blockCRC = !bwt.blockCRC;
    bwt.combinedCRC = (bwt.combinedCRC << 1) | (bwt.combinedCRC >> BZP_CRC_MOVE_RIGHT_VAL);
    bwt.combinedCRC ^= bwt.blockCRC;
}
