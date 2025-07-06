pub fn BzpDeCodeToStream(mut inData: Ptr<InDeComdata>, mut debwt: Ptr<BzpBwtDecodeInfo>) -> i32 {
    let mut ch: u8 = Default::default();
    let mut ret: i32 = BZP_OK!();
    c_for!(let mut i: i32 = 0; i < debwt.nBlock; i.suffix_plus_plus(); {
        ch = debwt.deCode[i];
        if (inData.num == BZP_RLC_NUM_4!()) {
            c_for!(let mut j: i32 = 0; j < ch.cast::<i32>(); j.suffix_plus_plus(); {
                BZP_UPDATE_CRC!(inData.blockCRC, inData.lasChar.cast::<u32>());
                ret |= BzpWriteChar(inData.lasChar.cast(), inData);
            });
            inData.lasChar = BZP_ASCII_SIZE!();
            inData.num = 0;
        } else if (ch == inData.lasChar.cast::<u8>()) {
            BZP_UPDATE_CRC!(inData.blockCRC, ch.cast::<u32>());
            ret = BzpWriteChar(ch, inData);
            inData.num += 1;
        } else {
            BZP_UPDATE_CRC!(inData.blockCRC, ch.cast::<u32>());
            ret = BzpWriteChar(ch, inData);
            inData.lasChar = ch.cast();
            inData.num = 1;
        }
        if (ret != BZP_OK!()) {
            break;
        }
    });
    return ret;
}
