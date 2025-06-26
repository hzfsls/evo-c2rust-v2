pub fn BzpDeCodeToStream(mut inData: Ptr<InDeComdata>, mut debwt: Ptr<BzpBwtDecodeInfo>) -> i32 {
    let mut ch: u8;
    let mut ret: i32 = BZP_OK!();
    c_for!(let mut i: i32 = 0; i < debwt.nBlock; i += 1; {
        ch = debwt.deCode[i];
        if inData.num == BZP_RLC_NUM_4!() {
            c_for!(let mut j: i32 = 0; j < ch.cast(); j += 1; {
                BZP_UPDATE_CRC!(inData.blockCRC, inData.lasChar);
                ret |= BzpWriteChar(inData.lasChar.cast(), inData);
            });
            inData.lasChar = BZP_ASCII_SIZE!();
            inData.num = 0;
        } else if ch == inData.lasChar as u8 {
            BZP_UPDATE_CRC!(inData.blockCRC, ch);
            ret = BzpWriteChar(ch, inData);
            inData.num += 1;
        } else {
            BZP_UPDATE_CRC!(inData.blockCRC, ch);
            ret = BzpWriteChar(ch, inData);
            inData.lasChar = ch.cast();
            inData.num = 1;
        }
        if ret != BZP_OK!() {
            break;
        }
    });
    return ret;
}