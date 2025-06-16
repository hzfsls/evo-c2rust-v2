use crate::translation_utils::*;
pub use crate::src::public::bzp_decompress_stream_h::*;

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


pub fn BzpInDeComdataFinish(mut inData: Ptr<InDeComdata>) {
    if (inData != NULL!()).as_bool() {
        c_free!(inData);
        inData = NULL!();
    }
}


pub fn BzpReadBits(mut nBit: i32, mut inData: Ptr<InDeComdata>) -> u32 {
    let mut res: u32 = 0;
    while (inData.nBuf < nBit) {
        if (inData.input.nBuf == inData.input.pos) {
            inData.input.nBuf = c_fread!(inData.input.buf, c_sizeof!(char), c_sizeofval!(inData.input.buf), inData.input.filePtr);
            inData.input.pos = 0;
        }
        let tmp0 = inData.input.pos;
        let mut data: u32 = (inData.input.buf[tmp0]).cast::<u32>();
        inData.buf = (inData.buf << BZP_BITS8!()) | data.cast::<u32>();
        inData.input.pos += 1;
        inData.nBuf += BZP_BITS8!();
    }
    res = (inData.buf >> (inData.nBuf - nBit));
    res = (res & ((1 << nBit) - 1));
    inData.nBuf -= nBit;
    return res;
}


pub fn BzpWriteChar(mut ch: u8, mut inData: Ptr<InDeComdata>) -> i32 {
    let mut ret: i32 = BZP_OK!();
    if (inData.output.nBuf >= BZP_BUF_SIZE!()) {
        let mut n2: i32 = c_fwrite!(inData.output.buf.cast::<Ptr<Void>>(), c_sizeof!(u8), inData.output.nBuf, inData.output.filePtr);
        if (n2 != inData.output.nBuf) {
            ret = BZP_ERROR_IO!();
        }
        inData.output.nBuf = 0;
    }
    let tmp0 = inData.output.nBuf.suffix_plus_plus();
    inData.output.buf[tmp0] = ch;
    return ret;
}


pub fn BzpHuffmanDecodeStep(mut huffman: Ptr<BzpHuffmanDecode>, mut inData: Ptr<InDeComdata>) -> i32 {
    if (huffman.deCodeNum == BZP_ELEMS_NUM_IN_ONE_GROUP!()) {
        huffman.deCodeNum = 0;
        huffman.selectCnt += 1;
    }
    let tmp0 = huffman.selectCnt;
    let mut gid: i32 = huffman.select[tmp0];
    let mut chlen: i32 = huffman.minLens[gid];
    let mut val: i32 = BzpReadBits(chlen, inData).cast();
    while (chlen < BZP_HUFFMAN_LEN_UPPER_LIMIT!()) && (val > huffman.limit[gid][chlen]) {
        chlen += 1;
        let mut nxtbit: i32 = BzpReadBits(1, inData).cast();
        val = (val << 1) | nxtbit;
    }
    if (chlen > BZP_HUFFMAN_LEN_UPPER_LIMIT!()) {
        return -1;
    }
    val = (val - huffman.base[gid][chlen]);
    val = huffman.perm[gid][val];
    huffman.deCodeNum += 1;
    return val;
}


pub fn BzpCheckFileHead(mut inData: Ptr<InDeComdata>) -> i32 {
    let mut ch: u8 = Default::default();
    ch = BzpReadBits(BZP_BITS8!(), inData.cast()).cast();
    if (ch != BZP_BLOCK_HEAD_1!()).as_bool() {
        return BZP_ERROR_DATA!();
    }
    ch = BzpReadBits(BZP_BITS8!(), inData.cast()).cast();
    if (ch != BZP_BLOCK_HEAD_2!()).as_bool() {
        return BZP_ERROR_DATA!();
    }
    ch = BzpReadBits(BZP_BITS8!(), inData.cast()).cast();
    if (ch != BZP_BLOCK_HEAD_3!()).as_bool() {
        return BZP_ERROR_DATA!();
    }
    ch = BzpReadBits(BZP_BITS8!(), inData.cast()).cast();
    if (ch != BZP_BLOCK_HEAD_4!()).as_bool() {
        return BZP_ERROR_DATA!();
    }
    ch = BzpReadBits(BZP_BITS8!(), inData.cast()).cast();
    if (ch != BZP_BLOCK_HEAD_5!()).as_bool() {
        return BZP_ERROR_DATA!();
    }
    return BZP_OK!();
}


pub fn BzpReadUInt24(mut inData: Ptr<InDeComdata>) -> u32 {
    let mut ch: u8 = Default::default();
    let mut val: u32 = 0;
    ch = BzpReadBits(BZP_BITS8!(), inData.cast()).cast();
    val = (val << BZP_BITS8!()) | ch.cast::<u32>();
    ch = BzpReadBits(BZP_BITS8!(), inData.cast()).cast();
    val = (val << BZP_BITS8!()) | ch.cast::<u32>();
    ch = BzpReadBits(BZP_BITS8!(), inData.cast()).cast();
    val = (val << BZP_BITS8!()) | ch.cast::<u32>();
    return val.cast();
}


pub fn BzpReadUInt32(mut inData: Ptr<InDeComdata>) -> u32 {
    let mut ch: u8 = Default::default();
    let mut val: u32 = 0;
    ch = BzpReadBits(BZP_BITS8!(), inData.cast()).cast();
    val = (val << BZP_BITS8!()) | ch.cast::<u32>();
    ch = BzpReadBits(BZP_BITS8!(), inData.cast()).cast();
    val = (val << BZP_BITS8!()) | ch.cast::<u32>();
    ch = BzpReadBits(BZP_BITS8!(), inData.cast()).cast();
    val = (val << BZP_BITS8!()) | ch.cast::<u32>();
    ch = BzpReadBits(BZP_BITS8!(), inData.cast()).cast();
    val = (val << BZP_BITS8!()) | ch.cast::<u32>();
    return val.cast();
}


pub fn BzpDeHuffmanSelect(mut inData: Ptr<InDeComdata>, mut huffman: Ptr<BzpHuffmanDecode>) -> i32 {
    let mut ch: u8 = Default::default();
    let mut selectmtf: Array<i32, { BZP_HUFFMAN_MAX_SIZE_SELECT!() }> = Default::default();
    c_for!(let mut i: i32 = 0; i < huffman.nSelect; i.suffix_plus_plus(); {
        let mut j: i32 = -1;
        c_do!({
            ch = BzpReadBits(BZP_BIT!(), inData.cast()).cast();
            j.suffix_plus_plus();
        } while ch != 0);
        if (j >= huffman.nGroups).as_bool() {
            return BZP_ERROR_DATA!();
        }
        selectmtf[i] = j.cast();
    });
    let mut listSelect: Array<i32, { BZP_MAX_GROUPS_NUM!() }> = Default::default();
    c_for!(let mut i: i32 = 0; i < BZP_MAX_GROUPS_NUM!(); i.suffix_plus_plus(); {
        listSelect[i] = i.cast();
    });
    c_for!(let mut i: i32 = 0; i < huffman.nSelect; i.suffix_plus_plus(); {
        let mut pos: i32 = selectmtf[i].cast();
        let mut tmpv: i32 = listSelect[pos].cast();
        c_for!(let mut j: i32 = pos; j > 0; j.suffix_minus_minus(); {
            listSelect[j] = listSelect[j - 1].cast();
        });
        listSelect[0] = tmpv.cast();
        huffman.select[i] = tmpv.cast();
    });
    return BZP_OK!();
}


pub fn BzpDeHuffmanLen(mut inData: Ptr<InDeComdata>, mut huffman: Ptr<BzpHuffmanDecode>) -> i32 {
    let mut ch: u8 = Default::default();
    c_for!(let mut i: i32 = 0; i < huffman.nGroups; i.suffix_plus_plus(); {
        let mut val: i32 = BzpReadBits(BZP_BITS5!(), inData.cast()).cast();
        c_for!(let mut j: i32 = 0; j < huffman.alphaSize; j.suffix_plus_plus(); {
            ch = BzpReadBits(BZP_BIT!(), inData.cast()).cast();
            while (ch != 0).as_bool() {
                ch = BzpReadBits(BZP_BIT!(), inData.cast()).cast();
                val += if ch == 0 { 1 } else { -1 };
                ch = BzpReadBits(BZP_BIT!(), inData.cast()).cast();
            }
            if (val < 1 || val > BZP_HUFFMAN_LEN_UPPER_LIMIT!()).as_bool() {
                return BZP_ERROR_DATA!();
            }
            huffman.len[i][j] = val.cast();
        });
    });
    return BZP_OK!();
}


pub fn BzpMTFDeCode(mut inData: Ptr<InDeComdata>, mut huffman: Ptr<BzpHuffmanDecode>, mut debwt: Ptr<BzpBwtDecodeInfo>) -> i32 {
    debwt.nBlock = 0;
    let mut ch: u8 = Default::default();
    let mut ninUse: i32 = (huffman.alphaSize - BZP_EXTRA_CHARS_NUM!());
    let mut eob: i32 = (ninUse + 1);
    let mut val: i32 = BzpHuffmanDecodeStep(huffman, inData);
    while (val != eob) && (val != -1) {
        if (val == 0) || (val == 1) {
            let mut res: i32 = 0;
            let mut basenum: i32 = 1;
            while (val == 0) || (val == 1) {
                res = (res + (val + 1) * basenum);
                basenum <<= 1;
                val = BzpHuffmanDecodeStep(huffman, inData);
            }
            c_for!(let mut j: i32 = 0; j < res; j.suffix_plus_plus(); {
                let tmp0 = debwt.nBlock;
                debwt.block[tmp0] = inData.list[0];
                debwt.nBlock += 1;
            });
        } else {
            let mut pos: i32 = (val - 1);
            ch = inData.list[pos].cast();
            debwt.block[debwt.nBlock] = ch;
            debwt.nBlock += 1;

            c_for!(let mut j: i32 = pos; j > 0; j.suffix_minus_minus(); {
                inData.list[j] = inData.list[j - 1];
            });
            inData.list[0] = ch.cast();
            val = BzpHuffmanDecodeStep(huffman, inData);
        }
    }
    if (val == -1) {
        return BZP_ERROR_DATA!();
    }
    return BZP_OK!();
}


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


pub fn BzpGetDictionaryList(mut inData: Ptr<InDeComdata>) -> i32 {
    let mut ninUse: i32 = 0;
    let mut use16: Array<bool, 16> = arr![false; 16];
    let mut inUse: Array<bool, { BZP_ASCII_SIZE!() }> = arr![false; BZP_ASCII_SIZE!()];
    c_for!(let mut i: i32 = 0; i < BZP_GROUPS_ASCII!().cast(); i.suffix_plus_plus(); {
        use16[i] = BzpReadBits(BZP_BIT!(), inData.cast()).cast();
    });
    c_for!(let mut i: i32 = 0; i < BZP_GROUPS_ASCII!().cast(); i.suffix_plus_plus(); {
        if use16[i].as_bool() {
            c_for!(let mut j: i32 = 0; j < BZP_CHARS_PER_GROUP_ASCII!(); j.suffix_plus_plus(); {
                inUse[i * BZP_GROUPS_ASCII!() + j] = BzpReadBits(BZP_BIT!(), inData.cast()).cast();
            });
        }
    });
    c_for!(let mut i: i32 = 0; i < BZP_ASCII_SIZE!().cast(); i.suffix_plus_plus(); {
        if inUse[i].as_bool() {
            inData.list[ninUse.suffix_plus_plus()] = i.cast();
        }
    });
    return ninUse.cast();
}


pub fn BzpDeCompressOneBlock(mut inData: Ptr<InDeComdata>, mut huffman: Ptr<BzpHuffmanDecode>, mut debwt: Ptr<BzpBwtDecodeInfo>) -> i32 {
    let mut ret: i32 = BZP_OK!();
    BzpCheckFileHead(inData);
    let mut blockCRC: u32 = BzpReadUInt32(inData);
    BzpReadBits(BZP_BIT!(), inData).cast::<Void>();
    let mut oriPtr: i32 = BzpReadUInt24(inData).cast();
    if (oriPtr < 0 || oriPtr > BZP_BASE_BLOCK_SIZE!() * inData.blockSize) {
        return BZP_ERROR_DATA!();
    }
    let mut ninUse: i32 = BzpGetDictionaryList(inData);
    huffman.alphaSize = (ninUse + BZP_EXTRA_CHARS_NUM!());
    huffman.nGroups = BzpReadBits(BZP_BITS3!(), inData).cast::<i32>();
    if (huffman.nGroups < BZP_NGROUPS_NUM_0!() || huffman.nGroups > BZP_NGROUPS_NUM_4!()) {
        return BZP_ERROR_DATA!();
    }
    huffman.nSelect = BzpReadBits(BZP_BITS15!(), inData).cast();
    let mut nSelectUpperLimit: i32 = (inData.blockSize * BZP_BASE_BLOCK_SIZE!() / BZP_ELEMS_NUM_IN_ONE_GROUP!() + 1);
    if (huffman.nSelect < 1 || huffman.nSelect > nSelectUpperLimit) {
        return BZP_ERROR_DATA!();
    }
    ret |= BzpDeHuffmanSelect(inData, huffman);
    ret |= BzpDeHuffmanLen(inData, huffman);
    if (ret != BZP_OK!()) {
        return ret;
    }
    BzpGenerateDecodeTable(huffman);
    debwt.oriPtr = oriPtr;
    ret = BzpMTFDeCode(inData, huffman, debwt);
    if (ret != BZP_OK!() || (debwt.nBlock >= BZP_BASE_BLOCK_SIZE!() * inData.blockSize)) {
        return BZP_ERROR_DATA!();
    }
    BzpBwtDecode(debwt);
    ret = BzpDeCodeToStream(inData, debwt);
    if (ret != BZP_OK!()) {
        return ret;
    }
    inData.blockCRC = !(inData.blockCRC);
    if (blockCRC != inData.blockCRC) {
        ret = BZP_ERROR_DATA!();
    }
    return ret;
}


pub fn BZPReadFileEnd(mut inData: Ptr<InDeComdata>, mut caltotalCRC: u32) -> i32 {
    let mut ch: u8 = BzpReadBits(BZP_BITS8!(), inData.cast()).cast();
    if (ch != BZP_FILE_END_1!()).as_bool() {
        return BZP_ERROR_DATA!();
    }
    ch = BzpReadBits(BZP_BITS8!(), inData.cast()).cast();
    if (ch != BZP_FILE_END_2!()).as_bool() {
        return BZP_ERROR_DATA!();
    }
    ch = BzpReadBits(BZP_BITS8!(), inData.cast()).cast();
    if (ch != BZP_FILE_END_3!()).as_bool() {
        return BZP_ERROR_DATA!();
    }
    ch = BzpReadBits(BZP_BITS8!(), inData.cast()).cast();
    if (ch != BZP_FILE_END_4!()).as_bool() {
        return BZP_ERROR_DATA!();
    }
    ch = BzpReadBits(BZP_BITS8!(), inData.cast()).cast();
    if (ch != BZP_FILE_END_5!()).as_bool() {
        return BZP_ERROR_DATA!();
    }
    let mut storedcombinedcrc: u32 = BzpReadUInt32(inData.cast()).cast();
    if (caltotalCRC != storedcombinedcrc).as_bool() {
        return BZP_ERROR_DATA!();
    }
    return BZP_OK!();
}


pub fn BzpReadFileHead(mut inData: Ptr<InDeComdata>) -> i32 {
    let mut ch: u8 = BzpReadBits(BZP_BITS8!(), inData.cast()).cast();
    if (ch != BZP_HDR_B!()).as_bool() {
        return BZP_ERROR_DATA_MAGIC!();
    }
    ch = BzpReadBits(BZP_BITS8!(), inData.cast()).cast();
    if (ch != BZP_HDR_Z!()).as_bool() {
        return BZP_ERROR_DATA_MAGIC!();
    }
    ch = BzpReadBits(BZP_BITS8!(), inData.cast()).cast();
    if (ch != BZP_HDR_H!()).as_bool() {
        return BZP_ERROR_DATA_MAGIC!();
    }
    ch = BzpReadBits(BZP_BITS8!(), inData.cast()).cast();
    let mut blockSize: i32 = (ch - BZP_HDR_0!()).cast();
    if BZP_INVALID_BLOCK_SIZE!(blockSize).as_bool() {
        return BZP_ERROR_DATA_MAGIC!();
    }
    inData.blockSize = blockSize.cast();
    return BZP_OK!();
}


pub fn BZPDeCompressData(mut inData: Ptr<InDeComdata>) -> i32 {
    let mut ret: i32 = BZP_OK!();
    let mut caltotalCRC: u32 = 0;
    let mut ch: u8;
    ret = BzpReadFileHead(inData);
    if (ret != BZP_OK!()) {
        return ret;
    }
    let mut huffman: Ptr<BzpHuffmanDecode> = BzpHuffmanDecodeInit(inData.blockSize);
    let mut debwt: Ptr<BzpBwtDecodeInfo> = BzpBwtDecodeInit(inData.blockSize);

    while {
        ch = BzpReadBits(BZP_BITS8!(), inData).cast::<u8>();
        ch != BZP_FILE_END_0!()
    } {
        if (ch != BZP_BLOCK_HEAD_0!()) {
            ret = BZP_ERROR_DATA!();
            break;
        }
        BzpHuffmanDecodeReset(huffman);
        inData.blockCRC = BZP_INIT_BLOCK_CRC!();

        ret = BzpDeCompressOneBlock(inData, huffman, debwt);
        if (ret != BZP_OK!()) {
            break;
        }

        caltotalCRC = (caltotalCRC << 1) | (caltotalCRC >> BZP_CRC_MOVE_RIGHT_VAL!());
        caltotalCRC ^= inData.blockCRC;
    }
    if (ret == BZP_OK!()) {
        ret = BZPReadFileEnd(inData, caltotalCRC);
    }
    BzpHuffmanDecodeFinish(huffman);
    BzpBwtDecodeFinish(debwt);
    return ret;
}


pub fn BzpDeComStreamFinish(mut inData: Ptr<InDeComdata>, mut inStream: Ptr<BzpStream>, mut outStream: Ptr<BzpStream>) {
    if (inStream.filePtr != NULL!()).as_bool() {
        c_fclose!(inStream.filePtr);
        inStream.filePtr = NULL!();
    }
    if (outStream.filePtr != NULL!()).as_bool() {
        c_fclose!(outStream.filePtr);
        outStream.filePtr = NULL!();
    }
    BzpStreamFinish(inStream.cast());
    BzpStreamFinish(outStream.cast());
    BzpInDeComdataFinish(inData.cast());
}


pub fn BzpDeCompressStream(mut inName: Ptr<u8>, mut outName: Ptr<u8>) -> i32 {
    let mut ret: i32 = BZP_OK!();
    if (inName == NULL!()).as_bool() || (outName == NULL!()).as_bool() {
        return BZP_ERROR_PARAM!();
    }

    let mut inStream: Ptr<BzpStream> = BzpStreamInit();
    let mut outStream: Ptr<BzpStream> = BzpStreamInit();
    if (inStream == NULL!()).as_bool() || (outStream == NULL!()).as_bool() {
        BzpStreamFinish(inStream.cast());
        BzpStreamFinish(outStream.cast());
        return BZP_ERROR_MEMORY_OPER_FAILURE!();
    }
    inStream.filePtr = c_fopen!(inName, cstr!("rb"));
    outStream.filePtr = c_fopen!(outName, cstr!("wb"));
    if (inStream.filePtr == NULL!()).as_bool() || (outStream.filePtr == NULL!()).as_bool() {
        c_free!(inStream);
        inStream = NULL!();
        c_free!(outStream);
        outStream = NULL!();
        c_remove!(outName);
        return BZP_ERROR_IO!();
    }
    let mut inData: Ptr<InDeComdata> = BzpInDeComdataInit();
    if (inData == NULL!()).as_bool() {
        BzpDeComStreamFinish(inData.cast(), inStream.cast(), outStream.cast());
        c_remove!(outName);
        return BZP_ERROR_MEMORY_OPER_FAILURE!();
    }
    inData.input = inStream.cast();
    inData.output = outStream.cast();

    ret = BZPDeCompressData(inData.cast()).cast();

    if (inData.output.nBuf > 0).as_bool() {
        let mut n2: i32 = c_fwrite!(inData.output.buf.cast::<Ptr<Void>>(), c_sizeof!(u8), inData.output.nBuf, inData.output.filePtr);
        if (n2 != inData.output.nBuf).as_bool() {
            ret = BZP_ERROR_IO!();
        }
        inData.output.nBuf = 0;
    }

    BzpDeComStreamFinish(inData.cast(), inStream.cast(), outStream.cast());
    if (ret != BZP_OK!()).as_bool() {
        c_remove!(outName);
    }
    return ret.cast();
}


