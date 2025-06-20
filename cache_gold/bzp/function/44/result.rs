pub fn BzpResetCompress(mut bwt: Ptr<BzpBwtInfo>, mut outData: Ptr<BzpOutComdata>) {
    outData.num = 0;
    bwt.nBlock = 0;
    bwt.blockCRC = BZP_INIT_BLOCK_CRC!();
    c_memset_s!(bwt.inUse, c_sizeofval!(bwt.inUse), 0, c_sizeofval!(bwt.inUse));
    let mut n: i32 = (outData.blockSize as u32 * BZP_BASE_BLOCK_SIZE!() * c_sizeof!(i32)).cast();
    c_memset_s!(bwt.isStartPos, n, 0, n);
    bwt.blockId += 1;
}