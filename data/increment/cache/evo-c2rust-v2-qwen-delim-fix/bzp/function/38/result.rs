pub fn BzpDeCodeToStream(mut inData: Ptr<InDeComdata>, mut debwt: Ptr<BzpBwtDecodeInfo>) -> i32 {
    let mut ch: u8 = Default::default();
    let mut ret: i32 = BZP_OK!();
    c_for!(let mut i: i32 = 0; i < debwt.nBlock.cast(); i.suffix_plus_plus(); {
        ch = debwt.deCode[i].cast();
        if (inData.num == BZP_RLC_NUM_4!()).as_bool() {
            c_for!(let mut j: i32 = 0; j < ch.cast(); j.suffix_plus_plus(); {
                BZP_UPDATE_CRC!(inData.blockCRC, inData.lasChar.cast::<u8>());
                ret |= BzpWriteChar(inData.lasChar.cast(), inData.cast()).cast();
            });
            inData.lasChar = BZP_ASCII_SIZE!().cast();
            inData.num = 0;
        } else if (ch == inData.lasChar).as_bool() {
            BZP_UPDATE_CRC!(inData.blockCRC, ch.cast::<u8>());
            ret = BzpWriteChar(ch.cast(), inData.cast()).cast();
            inData.num += 1;
        } else {
            BZP_UPDATE_CRC!(inData.blockCRC, ch.cast::<u8>());
            ret = BzpWriteChar(ch.cast(), inData.cast()).cast();
            inData.lasChar = ch.cast();
            inData.num = 1;
        }
        if (ret != BZP_OK!()).as_bool() {
            break;
        }
    });
    return ret.cast();
}