pub fn BzpInDeComdataInit() -> Ptr<InDeComdata> {
    let mut inData: Ptr<InDeComdata> = c_malloc!(c_sizeof!(InDeComdata));
    if (inData == NULL!()).as_bool() {
        return NULL!();
    }
    inData.input = NULL!();
    inData.output = NULL!();
    inData.num = 0;
    inData.lasChar = BZP_ASCII_SIZE!();
    inData.nBuf = 0;
    inData.buf = 0;
    inData.num = 0;

    inData.blockCRC = BZP_INIT_BLOCK_CRC!();
    return inData.cast();
}
