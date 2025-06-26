pub fn CmptlzWriteLE32Bit(mut addr: Ptr<Void>, mut val: u32) {
    if (CmptlzIsLE() != 0).as_bool() {
        CMPTLZ_WRITE32BIT!(addr, val);
    } else {
        CMPTLZ_WRITE32BIT!(addr, CmptlzSwap32(val));
    }
}
