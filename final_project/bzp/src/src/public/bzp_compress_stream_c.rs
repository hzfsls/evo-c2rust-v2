use crate::translation_utils::*;
pub use crate::src::public::bzp_compress_stream_h::*;

pub fn BzpAlgorithmInfoInit(mut blockSize: i32) -> Ptr<BzpAlgorithmInfo> {
    let mut bzpInfo: Ptr<BzpAlgorithmInfo> = c_malloc!(c_sizeof!(BzpAlgorithmInfo));
    if (bzpInfo == NULL!()).as_bool() {
        return NULL!();
    }
    bzpInfo.bwt = BzpBlockSortInit(blockSize.cast());
    bzpInfo.mtf = BzpMtfInit(blockSize.cast());
    bzpInfo.huffman = BzpHuffmanGroupsInit(blockSize.cast());
    bzpInfo.outData = BzpOutComDataInit(blockSize.cast());
    bzpInfo.compressFile = BzpFileInit();
    if (bzpInfo.bwt == NULL!()).as_bool() || (bzpInfo.outData == NULL!()).as_bool() || (bzpInfo.compressFile == NULL!()).as_bool() || (bzpInfo.mtf == NULL!()).as_bool() || (bzpInfo.huffman == NULL!()).as_bool() {
        BzpAlgorithmInfoFinish(bzpInfo.cast());
        return NULL!();
    }
    return bzpInfo.cast();
}


pub fn BzpOpenFile(mut bzpInfo: Ptr<BzpAlgorithmInfo>, mut inName: Ptr<u8>, mut outName: Ptr<u8>) -> i32 {
    if (bzpInfo == NULL!()).as_bool() {
        return BZP_ERROR_PARAM!();
    }
    bzpInfo.compressFile.input.filePtr = c_fopen!(inName, cstr!("rb"));
    bzpInfo.compressFile.output.filePtr = c_fopen!(outName, cstr!("wb"));
    if (bzpInfo.compressFile.input.filePtr == NULL!() || bzpInfo.compressFile.output.filePtr == NULL!()).as_bool() {
        BzpAlgorithmInfoFinish(bzpInfo.cast());
        c_remove!(outName);
        return BZP_ERROR_IO!();
    }
    return BZP_OK!();
}


pub fn BzpAlgorithmInfoFinish(mut bzpInfo: Ptr<BzpAlgorithmInfo>) {
    if (bzpInfo != NULL!()).as_bool() {
        BzpBwtFinish(bzpInfo.bwt.cast());
        BzpMtfFinish(bzpInfo.mtf.cast());
        BzpBzpHuffmanGroupsFinish(bzpInfo.huffman.cast());
        BzpFileFinish(bzpInfo.compressFile.cast());
        BzpOutComDataFinish(bzpInfo.outData.cast());
        c_free!(bzpInfo);
    }
}


pub fn BzpFileInit() -> Ptr<BzpFile> {
    let mut compressFile: Ptr<BzpFile> = c_malloc!(c_sizeof!(BzpFile));
    let mut inStream: Ptr<BzpStream> = BzpStreamInit();
    let mut outStream: Ptr<BzpStream> = BzpStreamInit();
    if (compressFile == NULL!()).as_bool() || (inStream == NULL!()).as_bool() || (outStream == NULL!()).as_bool() {
        BzpStreamFinish(inStream.cast());
        BzpStreamFinish(outStream.cast());
        BzpFileFinish(compressFile.cast());
        return NULL!();
    }
    compressFile.input = inStream.cast();
    compressFile.output = outStream.cast();
    compressFile.input.pos = 0;
    compressFile.output.pos = 0;
    compressFile.num = 0;
    compressFile.lasChar = BZP_ASCII_SIZE!();
    compressFile.state = BZP_INPUT_COMPRESS!();
    return compressFile.cast();
}


pub fn BzpFileFinish(mut bzpF: Ptr<BzpFile>) {
    if (bzpF != NULL!()).as_bool() {
        BzpStreamFinish(bzpF.input.cast());
        BzpStreamFinish(bzpF.output.cast());
        c_free!(bzpF);
        bzpF = NULL!();
    }
}


pub fn BzpOutComDataInit(mut blockSize: i32) -> Ptr<BzpOutComdata> {
    let mut outData: Ptr<BzpOutComdata> = c_malloc!(c_sizeof!(BzpOutComdata));
    if (outData == NULL!()).as_bool() {
        return NULL!();
    }
    outData.out = NULL!();

    outData.out = c_malloc!(blockSize * BZP_BASE_BLOCK_SIZE!() * c_sizeof!(u32));
    if (outData.out == NULL!()).as_bool() {
        c_free!(outData);
        return NULL!();
    }
    outData.nBuf = 0;
    outData.buf = 0;
    outData.num = 0;
    outData.blockSize = blockSize;
    return outData.cast();
}


pub fn BzpOutComDataFinish(mut data: Ptr<BzpOutComdata>) {
    if (data != NULL!()).as_bool() {
        if (data.out != NULL!()).as_bool() {
            c_free!(data.out);
            data.out = NULL!();
        }
        c_free!(data);
        data = NULL!();
    }
}


pub fn BzpWriteToArray(mut val: i32, mut n: i32, mut data: Ptr<BzpOutComdata>) {
    while (data.nBuf >= BZP_BITS8!()) {
        let tmp0 = data.num;
        data.out[tmp0] = (data.buf >> BZP_BITS24!()).cast::<u8>();
        data.num += 1;
        data.nBuf -= BZP_BITS8!();
        data.buf <<= BZP_BITS8!();
    }
    data.buf |= (val << (BZP_BITS32!() - n - data.nBuf)).cast::<u32>();
    data.nBuf += n;
}


pub fn BzpWriteInt32(mut val: i32, mut data: Ptr<BzpOutComdata>) {
    BzpWriteToArray(((val >> BZP_BITS24!()) & 0xff).cast(), BZP_BITS8!(), data.cast());
    BzpWriteToArray(((val >> BZP_BITS16!()) & 0xff).cast(), BZP_BITS8!(), data.cast());
    BzpWriteToArray(((val >> BZP_BITS8!()) & 0xff).cast(), BZP_BITS8!(), data.cast());
    BzpWriteToArray((val & 0xff).cast(), BZP_BITS8!(), data.cast());
}


pub fn BzpFileEOF(mut f: FilePtr) -> bool {
    let mut c: i32 = c_fgetc!(f);
    if (c == BZP_EOF!()).as_bool() {
        return true;
    }
    c_ungetc!(c, f).cast::<Void>();
    return false;
}


pub fn BzpWriteFileHead(mut outData: Ptr<BzpOutComdata>, mut blockId: i32) {
    if (blockId == 0).as_bool() {
        BzpWriteToArray(BZP_HDR_B!(), BZP_BITS8!(), outData.cast());
        BzpWriteToArray(BZP_HDR_Z!(), BZP_BITS8!(), outData.cast());
        BzpWriteToArray(BZP_HDR_H!(), BZP_BITS8!(), outData.cast());
        BzpWriteToArray((BZP_HDR_0!() + outData.blockSize).cast(), BZP_BITS8!(), outData.cast());
    }
}


pub fn BzpCalculateCRC(mut bwt: Ptr<BzpBwtInfo>) {
    bwt.blockCRC = !bwt.blockCRC;
    bwt.combinedCRC = (bwt.combinedCRC << 1) | (bwt.combinedCRC >> BZP_CRC_MOVE_RIGHT_VAL!());
    bwt.combinedCRC ^= bwt.blockCRC;
}


pub fn BzpWriteBlockHead(mut outData: Ptr<BzpOutComdata>, mut bwt: Ptr<BzpBwtInfo>) {
    BzpWriteToArray(BZP_BLOCK_HEAD_0!(), BZP_BITS8!(), outData.cast());
    BzpWriteToArray(BZP_BLOCK_HEAD_1!(), BZP_BITS8!(), outData.cast());
    BzpWriteToArray(BZP_BLOCK_HEAD_2!(), BZP_BITS8!(), outData.cast());
    BzpWriteToArray(BZP_BLOCK_HEAD_3!(), BZP_BITS8!(), outData.cast());
    BzpWriteToArray(BZP_BLOCK_HEAD_4!(), BZP_BITS8!(), outData.cast());
    BzpWriteToArray(BZP_BLOCK_HEAD_5!(), BZP_BITS8!(), outData.cast());
    BzpWriteInt32(bwt.blockCRC.cast(), outData.cast());
    BzpWriteToArray(0, BZP_BIT!(), outData.cast());
    BzpWriteToArray(bwt.oriPtr.cast(), BZP_BITS24!(), outData.cast());
}


pub fn BzpWriteValidASCII(mut outData: Ptr<BzpOutComdata>, mut bwt: Ptr<BzpBwtInfo>) {
    let mut validGid: Array<i32, { BZP_ASCII_SIZE!() }> = Default::default();
    let mut cnt: i32 = 0;
    let mut use16: Array<bool, { BZP_ASCII_SIZE!() }> = Default::default();
    c_memset_s!(use16, c_sizeofval!(use16), 0, c_sizeofval!(use16)).cast::<Void>();

    c_for!(let mut i: i32 = 0; i < BZP_ASCII_SIZE!().cast(); i.suffix_plus_plus(); {
        let mut gid: i32 = i / BZP_CHARS_PER_GROUP_ASCII!();
        use16[gid] |= bwt.inUse[i];
    });
    c_for!(let mut i: i32 = 0; i < BZP_GROUPS_ASCII!().cast(); i.suffix_plus_plus(); {
        BzpWriteToArray(use16[i].cast::<i32>(), BZP_BIT!(), outData.cast());
        if use16[i].as_bool() {
            validGid[cnt] = i.cast();
            cnt += 1;
        }
    });
    c_for!(let mut i: i32 = 0; i < cnt; i.suffix_plus_plus(); {
        c_for!(let mut j: i32 = 0; j < BZP_CHARS_PER_GROUP_ASCII!(); j.suffix_plus_plus(); {
            let mut valid: i32 = validGid[i] * BZP_CHARS_PER_GROUP_ASCII!() + j;
            BzpWriteToArray(bwt.inUse[valid].cast::<i32>(), BZP_BIT!(), outData.cast());
        });
    });
}


pub fn BzpWriteSelect(mut outData: Ptr<BzpOutComdata>, mut huffman: Ptr<BzpHuffmanGroups>) {
    BzpWriteToArray(huffman.nSelect.cast(), BZP_BITS15!(), outData.cast());
    c_for!(let mut i: i32 = 0; i < huffman.nSelect; i.suffix_plus_plus(); {
        c_for!(let mut j: i32 = 0; j < huffman.selectMTF[i]; j.suffix_plus_plus(); {
            BzpWriteToArray(1, BZP_BIT!(), outData.cast());
        });
        BzpWriteToArray(0, BZP_BIT!(), outData.cast());
    });
}


pub fn BzpWriteLen(mut outData: Ptr<BzpOutComdata>, mut huffman: Ptr<BzpHuffmanGroups>) {
    c_for!(let mut i: i32 = 0; i < huffman.nGroups; i.suffix_plus_plus(); {
        let mut val: i32 = huffman.huffmanGroups[i].len[0];
        BzpWriteToArray(val.cast(), BZP_BITS5!(), outData.cast());
        c_for!(let mut j: i32 = 0; j < huffman.alphaSize; j.suffix_plus_plus(); {
            let mut tar: i32 = huffman.huffmanGroups[i].len[j];
            let mut deta: i32 = 0;
            let mut saveVal: i32 = 0;
            if (val < tar).as_bool() {
                saveVal = BZP_HUFFMAN_LEN_INCREASE!();
                deta = 1;
            } else if (val > tar).as_bool() {
                saveVal = BZP_HUFFMAN_LEN_REDUCED!();
                deta = -1;
            }
            while (val != tar).as_bool() {
                BzpWriteToArray(saveVal.cast(), BZP_BITS2!(), outData.cast());
                val += deta;
            }
            BzpWriteToArray(0, BZP_BIT!(), outData.cast());
        });
    });
}


pub fn BzpWriteInputEncode(mut outData: Ptr<BzpOutComdata>, mut mtf: Ptr<BzpMtfInfo>, mut huffman: Ptr<BzpHuffmanGroups>) {
    c_for!(let mut i: i32 = 0; i < mtf.nMtf; i.suffix_plus_plus(); {
        let mut val: i32 = mtf.mtfV[i].cast();
        let mut gid: i32 = huffman.select[i / BZP_ELEMS_NUM_IN_ONE_GROUP!()].cast();
        let mut code: i32 = huffman.huffmanGroups[gid].table[val].cast();
        let mut len: i32 = huffman.huffmanGroups[gid].len[val].cast();
        BzpWriteToArray(code.cast(), len.cast(), outData.cast());
    });
}


pub fn BzpWriteFileEnd(mut outData: Ptr<BzpOutComdata>, mut combinedCRC: i32) {
    BzpWriteToArray(BZP_FILE_END_0!(), BZP_BITS8!(), outData.cast());
    BzpWriteToArray(BZP_FILE_END_1!(), BZP_BITS8!(), outData.cast());
    BzpWriteToArray(BZP_FILE_END_2!(), BZP_BITS8!(), outData.cast());
    BzpWriteToArray(BZP_FILE_END_3!(), BZP_BITS8!(), outData.cast());
    BzpWriteToArray(BZP_FILE_END_4!(), BZP_BITS8!(), outData.cast());
    BzpWriteToArray(BZP_FILE_END_5!(), BZP_BITS8!(), outData.cast());
    BzpWriteInt32(combinedCRC.cast(), outData.cast());
}


pub fn BzpFlushbuf(mut outData: Ptr<BzpOutComdata>) {
    while (outData.nBuf > 0) {
        let tmp0 = outData.num;
        outData.out[tmp0] = (outData.buf >> BZP_BITS24!()).cast::<u8>();
        outData.num += 1;
        outData.nBuf -= BZP_BITS8!();
        outData.buf <<= BZP_BITS8!();
    }
}


pub fn BzpCompressOneBlock(mut bzpInfo: Ptr<BzpAlgorithmInfo>, mut outData: Ptr<BzpOutComdata>) -> i32 {
    let mut bwt: Ptr<BzpBwtInfo> = bzpInfo.bwt.cast();
    let mut mtf: Ptr<BzpMtfInfo> = bzpInfo.mtf.cast();
    let mut huffman: Ptr<BzpHuffmanGroups> = bzpInfo.huffman.cast();
    let mut ret: i32 = BZP_OK!();
    if (bwt.nBlock == 0).as_bool() {
        return BZP_OK!();
    }
    BzpWriteFileHead(outData.cast(), bwt.blockId.cast());
    if (bwt.nBlock > 0).as_bool() {
        BzpCalculateCRC(bwt.cast());
        BzpBlockSortMain(bwt.cast());
        BzpMtfReSet(mtf.cast());
        mtf.block = bwt.block.cast();
        mtf.map = bwt.sortBlock.cast();
        mtf.inUse = bwt.inUse.cast();
        mtf.nBlock = bwt.nBlock.cast();
        BzpMtfMain(mtf.cast());
        ret = BzpHuffmanGroupsReset(huffman.cast(), (mtf.nUse + BZP_EXTRA_CHARS_NUM!()).cast()).cast();
        if (ret != BZP_OK!()).as_bool() {
            return ret;
        }
        huffman.block = mtf.mtfV.cast();
        huffman.mtfFreq = mtf.mtfFreq.cast();
        huffman.nBlock = mtf.nMtf.cast();
        BzpHuffmanMain(huffman.cast());
        BzpWriteBlockHead(outData.cast(), bwt.cast());
        BzpWriteValidASCII(outData.cast(), bwt.cast());
        BzpWriteToArray(huffman.nGroups.cast(), BZP_BITS3!(), outData.cast());
        BzpWriteSelect(outData.cast(), huffman.cast());
        BzpWriteLen(outData.cast(), huffman.cast());
        BzpWriteInputEncode(outData.cast(), mtf.cast(), huffman.cast());
    }
    return BZP_OK!();
}


pub fn BzpBuffToStream(mut bzpf: Ptr<BzpFile>, mut outData: Ptr<BzpOutComdata>) -> i32 {
    bzpf.output.pos = 0;
    let mut pos: i32 = 0;
    while (pos < outData.num) {
        bzpf.output.nBuf = 0;
        while (pos < outData.num) && (bzpf.output.nBuf < BZP_BUF_SIZE!()) {
            let tmp0 = bzpf.output.nBuf;
            bzpf.output.buf[tmp0] = outData.out[pos];
            bzpf.output.nBuf += 1;
            pos += 1;
        }
        let mut n2: i32 = c_fwrite!(bzpf.output.buf.cast::<Ptr<Void>>(), c_sizeof!(u8), bzpf.output.nBuf, bzpf.output.filePtr);
        if (n2 != bzpf.output.nBuf) {
            return BZP_ERROR_IO!();
        }
    }
    return BZP_OK!();
}


pub fn BzpAddCharToBlock(mut lasch: u8, mut num: i32, mut bwt: Ptr<BzpBwtInfo>) {
    if (num < BZP_RLC_NUM_LOWER_LIMIT!()) || (num > BZP_RLC_NUM_UPPER_LIMIT!()) {
        return;
    }
    c_for!(let mut i: i32 = 0; i < num; i.suffix_plus_plus(); {
        BZP_UPDATE_CRC!(bwt.blockCRC, lasch);
    });
    let mut val: i32 = BZP_MIN_FUN!(num, BZP_RLC_NUM_4!());
    c_switch!(val, {
        BZP_RLC_NUM_4!() => {
            let tmp0 = bwt.nBlock.suffix_plus_plus();
            bwt.block[tmp0] = lasch;
        }
        BZP_RLC_NUM_3!() => {
            bwt.block[bwt.nBlock.suffix_plus_plus()] = lasch;
        }
        BZP_RLC_NUM_2!() => {
            bwt.block[bwt.nBlock.suffix_plus_plus()] = lasch;
        }
        BZP_RLC_NUM_1!() => {
            bwt.block[bwt.nBlock.suffix_plus_plus()] = lasch;
        }
        _ => {}
    });
    if (num >= BZP_RLC_NUM_4!()) {
        bwt.block[bwt.nBlock.suffix_plus_plus()] = (num - BZP_RLC_NUM_4!()).cast::<u8>();
        bwt.inUse[num - BZP_RLC_NUM_4!()] = true;
    }
    bwt.inUse[lasch] = true;
}


pub fn BzpBuffToBlockRLC(mut bzpf: Ptr<BzpFile>, mut bwt: Ptr<BzpBwtInfo>, mut IsLastdata: bool) {
    while (!BZP_BLOCK_FULL!(bwt).as_bool() && (!BZP_BUFF_READ_EMPTY!(bzpf)).as_bool()) {
        let mut pos: i32 = bzpf.input.pos.cast();
        let mut ch: u8 = (bzpf.input.buf[pos]).cast::<u8>();
        let mut lasch: u8 = (bzpf.lasChar).cast::<u8>();
        if (ch != lasch).as_bool() || (bzpf.num == BZP_RLC_NUM_UPPER_LIMIT!()).as_bool() {
            BzpAddCharToBlock(lasch.cast(), bzpf.num.cast(), bwt.cast());
            bzpf.lasChar = ch.cast();
            bzpf.num = 1;
        } else {
            bzpf.num += 1;
        }
        bzpf.input.pos += 1;
    }
    if IsLastdata.as_bool() && BZP_BUFF_READ_EMPTY!(bzpf).as_bool() {
        BzpAddCharToBlock(bzpf.lasChar.cast(), bzpf.num.cast(), bwt.cast());
        bzpf.lasChar = BZP_ASCII_SIZE!();
        bzpf.num = 0;
    }
}


pub fn BzpResetCompress(mut bwt: Ptr<BzpBwtInfo>, mut outData: Ptr<BzpOutComdata>) {
    outData.num = 0;
    bwt.nBlock = 0;
    bwt.blockCRC = BZP_INIT_BLOCK_CRC!();
    c_memset_s!(bwt.inUse, c_sizeofval!(bwt.inUse), 0, c_sizeofval!(bwt.inUse)).cast::<Void>();
    let mut n: i32 = outData.blockSize * BZP_BASE_BLOCK_SIZE!() * c_sizeof!(i32);
    c_memset_s!(bwt.isStartPos, n, 0, n).cast::<Void>();
    bwt.blockId += 1;
}


pub fn BzpProcessData(mut bzpInfo: Ptr<BzpAlgorithmInfo>, mut IsLastdata: bool) -> i32 {
    let mut bzpf: Ptr<BzpFile> = bzpInfo.compressFile.cast();
    let mut outData: Ptr<BzpOutComdata> = bzpInfo.outData.cast();
    let mut bwt: Ptr<BzpBwtInfo> = bzpInfo.bwt.cast();

    bzpf.state = BZP_INPUT_COMPRESS!();
    let mut ret: i32 = BZP_OK!();
    while (bzpf.state != BZP_RETUEN_COMPRESS!()).as_bool() {
        if (bzpf.state == BZP_OUTPUT_COMPRESS!()).as_bool() {
            ret = BzpBuffToStream(bzpf.cast(), outData.cast()).cast();

            BzpResetCompress(bwt.cast(), outData.cast());
            bzpf.state = BZP_INPUT_COMPRESS!();
            if IsLastdata.as_bool() && BZP_BUFF_READ_EMPTY!(bzpf).as_bool() {
                bzpf.state = BZP_RETUEN_COMPRESS!();
            }
        }
        if (bzpf.state == BZP_INPUT_COMPRESS!()).as_bool() {
            BzpBuffToBlockRLC(bzpf.cast(), bwt.cast(), IsLastdata.cast());

            if IsLastdata.as_bool() && BZP_BUFF_READ_EMPTY!(bzpf).as_bool() {
                ret = BzpCompressOneBlock(bzpInfo.cast(), outData.cast()).cast();

                BzpWriteFileEnd(outData.cast(), bwt.combinedCRC.cast());
                BzpFlushbuf(outData.cast());

                bzpf.state = BZP_OUTPUT_COMPRESS!();
            } else if BZP_BLOCK_FULL!(bwt).as_bool() {
                ret = BzpCompressOneBlock(bzpInfo.cast(), outData.cast()).cast();
                bzpf.state = BZP_OUTPUT_COMPRESS!();
            } else {
                bzpf.state = BZP_RETUEN_COMPRESS!();
            }
        }
        if (ret != BZP_OK!()).as_bool() {
            return ret;
        }
    }
    return ret.cast();
}


pub fn BzpCompressEnd(mut bzpInfo: Ptr<BzpAlgorithmInfo>) {
    if (bzpInfo.compressFile.input.filePtr != NULL!()).as_bool() {
        c_fclose!(bzpInfo.compressFile.input.filePtr);
    }
    if (bzpInfo.compressFile.output.filePtr != NULL!()).as_bool() {
        c_fclose!(bzpInfo.compressFile.output.filePtr);
    }
    BzpAlgorithmInfoFinish(bzpInfo.cast());
}


pub fn BzpCompressStream(mut inName: Ptr<u8>, mut outName: Ptr<u8>, mut blockSize: i32) -> i32 {
    let mut ret: i32 = BZP_OK!();
    let mut IsLastdata: bool = false;
    if (inName == NULL!()) || (outName == NULL!()) || BZP_INVALID_BLOCK_SIZE!(blockSize) {
        return BZP_ERROR_PARAM!();
    }
    let mut bzpInfo: Ptr<BzpAlgorithmInfo> = BzpAlgorithmInfoInit(blockSize);
    if (bzpInfo == NULL!()) {
        return BZP_ERROR_MEMORY_OPER_FAILURE!();
    }
    ret = BzpOpenFile(bzpInfo, inName, outName);
    if (ret != BZP_OK!()) {
        return ret;
    }
    let mut inStream: Ptr<BzpStream> = bzpInfo.compressFile.input;
    while !IsLastdata {
        inStream.nBuf = c_fread!(inStream.buf, c_sizeof!(char), c_sizeofval!(inStream.buf), inStream.filePtr);
        inStream.pos = 0;
        IsLastdata = BzpFileEOF(inStream.filePtr);
        ret = BzpProcessData(bzpInfo, IsLastdata);
        if (ret != BZP_OK!()) {
            break;
        }
    }
    BzpCompressEnd(bzpInfo);
    if (ret != BZP_OK!()) {
        c_remove!(outName);
    }
    return ret;
}


