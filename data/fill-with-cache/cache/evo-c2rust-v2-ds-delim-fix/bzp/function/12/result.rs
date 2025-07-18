pub fn BzpMTFDeCode(mut inData: Ptr<InDeComdata>, mut huffman: Ptr<BzpHuffmanDecode>, mut debwt: Ptr<BzpBwtDecodeInfo>) -> i32 {
    debwt.nBlock = 0;
    let mut ch: u8 = Default::default();
    let mut ninUse: i32 = huffman.alphaSize - BZP_EXTRA_CHARS_NUM!();
    let mut eob: i32 = ninUse + 1;
    let mut val: i32 = BzpHuffmanDecodeStep(huffman.cast(), inData.cast()).cast();
    while (val != eob).as_bool() && (val != -1).as_bool() {
        if (val == 0).as_bool() || (val == 1).as_bool() {
            let mut res: i32 = 0;
            let mut basenum: i32 = 1;
            while (val == 0).as_bool() || (val == 1).as_bool() {
                res = res + (val + 1) * basenum;
                basenum <<= 1;
                val = BzpHuffmanDecodeStep(huffman.cast(), inData.cast()).cast();
            }
            c_for!(let mut j: i32 = 0; j < res; j.suffix_plus_plus(); {
                debwt.block[debwt.nBlock] = inData.list[0].cast();
                debwt.nBlock += 1;
            });
        } else {
            let mut pos: i32 = val - 1;
            ch = inData.list[pos].cast();
            debwt.block[debwt.nBlock] = ch.cast();
            debwt.nBlock += 1;
            c_for!(let mut j: i32 = pos; j > 0; j.suffix_minus_minus(); {
                inData.list[j] = inData.list[j - 1].cast();
            });
            inData.list[0] = ch.cast();
            val = BzpHuffmanDecodeStep(huffman.cast(), inData.cast()).cast();
        }
    }
    if (val == -1).as_bool() {
        return BZP_ERROR_DATA!();
    }
    return BZP_OK!();
}
