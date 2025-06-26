pub fn BzpResetCompress(mut bwt: Ptr<BzpBwtInfo>, mut outData: Ptr<BzpOutComdata>) {
    outData.num = 0;
    bwt.nBlock = 0;
    bwt.blockCRC = BZP_INIT_BLOCK_CRC!();
    c_memset_s!(bwt.inUse, c_sizeofval!(bwt.inUse), 0, c_sizeofval!(bwt.inUse)).cast::<Void>();
    let mut n: i32 = outData.blockSize * BZP_BASE_BLOCK_SIZE!() * c_sizeof!(i32);
    c_memset_s!(bwt.isStartPos, n.cast(), 0, n.cast()).cast::<Void>();
    bwt.blockId += 1;
}
