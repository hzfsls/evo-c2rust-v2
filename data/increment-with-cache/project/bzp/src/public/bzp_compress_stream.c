/**
 * @file bzp_compress_stream.c
 * @copyright Copyright (c) Huawei Technologies Co., Ltd. 2023-2023. All rights reserved.
 * @brief 提供bzp压缩功能
 * @author w50034131
 * @date 2023-08-24
 * @version v0.1.0
 * *******************************************************************************************
 * @par 修改日志：
 * <table>
 * <tr><th>Date        <th>Version  <th>Author    <th>Description
 * <tr><td>2023-08-24  <td>0.1.0    <td>w50034131 <td>Initial Version
 * </table>
 *
 * *******************************************************************************************
 */
#include "bzp_compress_stream.h"

#ifdef __cplusplus
extern "C" {
#endif

BzpAlgorithmInfo *BzpAlgorithmInfoInit(int32_t blockSize)
{
    BzpAlgorithmInfo *bzpInfo = (BzpAlgorithmInfo *)malloc(sizeof(BzpAlgorithmInfo));
    if (bzpInfo == NULL) {
        return NULL;
    }
    bzpInfo->bwt = BzpBlockSortInit(blockSize);
    bzpInfo->mtf = BzpMtfInit(blockSize);
    bzpInfo->huffman = BzpHuffmanGroupsInit(blockSize);
    bzpInfo->outData = BzpOutComDataInit(blockSize);
    bzpInfo->compressFile = BzpFileInit();

    if (bzpInfo->bwt == NULL || bzpInfo->outData == NULL || bzpInfo->compressFile == NULL || bzpInfo->mtf == NULL
        || bzpInfo->huffman == NULL) {
        BzpAlgorithmInfoFinish(bzpInfo);
        return NULL;
    }
    return bzpInfo;
}
int32_t BzpOpenFile(BzpAlgorithmInfo *bzpInfo, char *inName, char *outName)
{
    if (bzpInfo == NULL) {
        return BZP_ERROR_PARAM;
    }
    bzpInfo->compressFile->input->filePtr = fopen(inName, "rb");
    bzpInfo->compressFile->output->filePtr = fopen(outName, "wb");
    if ((bzpInfo->compressFile->input->filePtr == NULL || bzpInfo->compressFile->output->filePtr == NULL)) {
        BzpAlgorithmInfoFinish(bzpInfo);
        remove(outName);
        return BZP_ERROR_IO;
    }
    return BZP_OK;
}
void BzpAlgorithmInfoFinish(BzpAlgorithmInfo *bzpInfo)
{
    if (bzpInfo != NULL) {
        BzpBwtFinish(bzpInfo->bwt);
        BzpMtfFinish(bzpInfo->mtf);
        BzpBzpHuffmanGroupsFinish(bzpInfo->huffman);
        BzpFileFinish(bzpInfo->compressFile);
        BzpOutComDataFinish(bzpInfo->outData);
        free(bzpInfo);
    }
}
BzpFile *BzpFileInit()
{
    BzpFile *compressFile = (BzpFile *)malloc(sizeof(BzpFile));
    BzpStream *inStream = BzpStreamInit();
    BzpStream *outStream = BzpStreamInit();
    if (compressFile == NULL || inStream == NULL || outStream == NULL) {
        BzpStreamFinish(inStream);
        BzpStreamFinish(outStream);
        BzpFileFinish(compressFile);
        return NULL;
    }
    compressFile->input = inStream;
    compressFile->output = outStream;
    compressFile->input->pos = 0;
    compressFile->output->pos = 0;
    compressFile->num = 0;
    compressFile->lasChar = BZP_ASCII_SIZE;
    compressFile->state = BZP_INPUT_COMPRESS;
    return compressFile;
}
void BzpFileFinish(BzpFile *bzpF)
{
    if (bzpF != NULL) {
        BzpStreamFinish(bzpF->input);
        BzpStreamFinish(bzpF->output);
        free(bzpF);
        bzpF = NULL;
    }
}
BzpOutComdata *BzpOutComDataInit(int32_t blockSize)
{
    BzpOutComdata *outData = (BzpOutComdata *)malloc(sizeof(BzpOutComdata));
    if (outData == NULL) {
        return NULL;
    }
    outData->out = NULL;
    // 下面可以尝试申请更少的空间
    outData->out = (uint8_t *)malloc(blockSize * BZP_BASE_BLOCK_SIZE * sizeof(uint32_t));
    if (outData->out == NULL) {
        free(outData);
        return NULL;
    }
    outData->nBuf = 0;
    outData->buf = 0;
    outData->num = 0;
    outData->blockSize = blockSize;
    return outData;
}
void BzpOutComDataFinish(BzpOutComdata *data)
{
    if (data != NULL) {
        if (data->out != NULL) {
            free(data->out);
            data->out = NULL;
        }
        free(data);
        data = NULL;
    }
}
// 将n位的数据写入到缓冲区或者缓冲的数组
void BzpWriteToArray(int32_t val, int32_t n, BzpOutComdata *data)
{
    // 数据存放在高nBuf位
    while (data->nBuf >= BZP_BITS8) {
        data->out[data->num++] = (uint8_t)(data->buf >> BZP_BITS24);
        data->nBuf -= BZP_BITS8;
        data->buf <<= BZP_BITS8;
    }
    data->buf |= (val << (BZP_BITS32 - n - data->nBuf));
    data->nBuf += n;
}
// 写32位数据
void BzpWriteInt32(int32_t val, BzpOutComdata *data)
{
    // 将val中低n位写入outdata, 保证高位为0
    BzpWriteToArray((val >> BZP_BITS24) & 0xffL, BZP_BITS8, data);
    BzpWriteToArray((val >> BZP_BITS16) & 0xffL, BZP_BITS8, data);
    BzpWriteToArray((val >> BZP_BITS8) & 0xffL, BZP_BITS8, data);
    BzpWriteToArray(val & 0xffL, BZP_BITS8, data);
}
// 判断流中数据是否读完
bool BzpFileEOF(FILE *f)
{
    int32_t c = fgetc(f);
    if (c == BZP_EOF)
        return true;
    (void)ungetc(c, f);
    return false;
}
void BzpWriteFileHead(BzpOutComdata *outData, int32_t blockId)
{
    if (blockId == 0) {
        BzpWriteToArray(BZP_HDR_B, BZP_BITS8, outData);
        BzpWriteToArray(BZP_HDR_Z, BZP_BITS8, outData);
        BzpWriteToArray(BZP_HDR_H, BZP_BITS8, outData);
        BzpWriteToArray((BZP_HDR_0 + outData->blockSize), BZP_BITS8, outData);
    }
}
void BzpCalculateCRC(BzpBwtInfo *bwt)
{
    bwt->blockCRC = ~(bwt->blockCRC);
    bwt->combinedCRC = (bwt->combinedCRC << 1) | (bwt->combinedCRC >> BZP_CRC_MOVE_RIGHT_VAL);
    bwt->combinedCRC ^= bwt->blockCRC;
}
// 每个块的首部 标志位+校验码+0+bwt解码起始位置
void BzpWriteBlockHead(BzpOutComdata *outData, BzpBwtInfo *bwt)
{
    BzpWriteToArray(BZP_BLOCK_HEAD_0, BZP_BITS8, outData);
    BzpWriteToArray(BZP_BLOCK_HEAD_1, BZP_BITS8, outData);
    BzpWriteToArray(BZP_BLOCK_HEAD_2, BZP_BITS8, outData);
    BzpWriteToArray(BZP_BLOCK_HEAD_3, BZP_BITS8, outData);
    BzpWriteToArray(BZP_BLOCK_HEAD_4, BZP_BITS8, outData);
    BzpWriteToArray(BZP_BLOCK_HEAD_5, BZP_BITS8, outData);
    BzpWriteInt32(bwt->blockCRC, outData);             // 每个块的循环校验码
    BzpWriteToArray(0, BZP_BIT, outData);              // blockRandomised
    BzpWriteToArray(bwt->oriPtr, BZP_BITS24, outData); // bwt解码时起始位置
}
void BzpWriteValidASCII(BzpOutComdata *outData, BzpBwtInfo *bwt)
{
    int32_t validGid[BZP_ASCII_SIZE], cnt = 0;
    bool use16[BZP_ASCII_SIZE];
    (void)memset_s(use16, sizeof(use16), 0, sizeof(use16));

    for (int32_t i = 0; i < BZP_ASCII_SIZE; i++) {
        int32_t gid = i / BZP_CHARS_PER_GROUP_ASCII;
        use16[gid] |= bwt->inUse[i];
    }
    for (int32_t i = 0; i < BZP_GROUPS_ASCII; i++) {
        BzpWriteToArray((int32_t)(use16[i]), BZP_BIT, outData);
        if (use16[i]) {
            validGid[cnt++] = i;
        }
    }
    for (int32_t i = 0; i < cnt; i++) {
        for (int32_t j = 0; j < BZP_CHARS_PER_GROUP_ASCII; j++) {
            int32_t valid = validGid[i] * BZP_CHARS_PER_GROUP_ASCII + j;
            BzpWriteToArray((int32_t)(bwt->inUse[valid]), BZP_BIT, outData);
        }
    }
}

void BzpWriteSelect(BzpOutComdata *outData, BzpHuffmanGroups *huffman)
{
    // nselector
    BzpWriteToArray(huffman->nSelect, BZP_BITS15, outData);

    // selector[i]
    for (int32_t i = 0; i < huffman->nSelect; i++) {
        for (int32_t j = 0; j < huffman->selectMTF[i]; j++) {
            BzpWriteToArray(1, BZP_BIT, outData);
        }
        BzpWriteToArray(0, BZP_BIT, outData);
    }
}
void BzpWriteLen(BzpOutComdata *outData, BzpHuffmanGroups *huffman)
{
    for (int32_t i = 0; i < huffman->nGroups; i++) {
        int32_t val = huffman->huffmanGroups[i].len[0];
        BzpWriteToArray(val, BZP_BITS5, outData);
        // 存储相对于前一个数的差值，这里存储(tar-val)个2或者3，tar>val存2否则存3
        for (int32_t j = 0; j < huffman->alphaSize; j++) {
            int32_t tar = huffman->huffmanGroups[i].len[j];
            int32_t deta = 0, saveVal = 0;
            if (val < tar) {
                saveVal = BZP_HUFFMAN_LEN_INCREASE;
                deta = 1;
            } else if (val > tar) {
                saveVal = BZP_HUFFMAN_LEN_REDUCED;
                deta = -1;
            }
            while (val != tar) {
                BzpWriteToArray(saveVal, BZP_BITS2, outData);
                val += deta;
            }
            BzpWriteToArray(0, BZP_BIT, outData);
        }
    }
}
void BzpWriteInputEncode(BzpOutComdata *outData, BzpMtfInfo *mtf, BzpHuffmanGroups *huffman)
{
    for (int32_t i = 0; i < mtf->nMtf; i++) {
        int32_t val = mtf->mtfV[i];
        int32_t gid = huffman->select[i / BZP_ELEMS_NUM_IN_ONE_GROUP];
        int32_t code = huffman->huffmanGroups[gid].table[val];
        int32_t len = huffman->huffmanGroups[gid].len[val];
        BzpWriteToArray(code, len, outData);
    }
}
void BzpWriteFileEnd(BzpOutComdata *outData, int32_t combinedCRC)
{
    BzpWriteToArray(BZP_FILE_END_0, BZP_BITS8, outData);
    BzpWriteToArray(BZP_FILE_END_1, BZP_BITS8, outData);
    BzpWriteToArray(BZP_FILE_END_2, BZP_BITS8, outData);
    BzpWriteToArray(BZP_FILE_END_3, BZP_BITS8, outData);
    BzpWriteToArray(BZP_FILE_END_4, BZP_BITS8, outData);
    BzpWriteToArray(BZP_FILE_END_5, BZP_BITS8, outData);
    BzpWriteInt32(combinedCRC, outData);
}
// 刷新输出32缓冲区 从buf写入到数组
void BzpFlushbuf(BzpOutComdata *outData)
{
    while (outData->nBuf > 0) {
        outData->out[outData->num++] = (uint8_t)(outData->buf >> BZP_BITS24);
        outData->nBuf -= BZP_BITS8;
        outData->buf <<= BZP_BITS8;
    }
}
// 对单个块的数据进行压缩
int32_t BzpCompressOneBlock(BzpAlgorithmInfo *bzpInfo, BzpOutComdata *outData)
{
    BzpBwtInfo *bwt = bzpInfo->bwt;
    BzpMtfInfo *mtf = bzpInfo->mtf;
    BzpHuffmanGroups *huffman = bzpInfo->huffman;
    int ret = BZP_OK;
    if (bwt->nBlock == 0) { // 块内无数据退出压缩
        return BZP_OK;
    }
    // 判断第一个块，然后头信息。
    BzpWriteFileHead(outData, bwt->blockId);
    if (bwt->nBlock > 0) {
        // 计算combinedCRC
        BzpCalculateCRC(bwt);
        // bwt
        BzpBlockSortMain(bwt);
        // mtf重置
        BzpMtfReSet(mtf);
        // bwt to mtf
        mtf->block = bwt->block;
        mtf->map = bwt->sortBlock;
        mtf->inUse = bwt->inUse;
        mtf->nBlock = bwt->nBlock;
        // mtf
        BzpMtfMain(mtf);
        // 出现的单词数量+EOB+用1额外编码（mtf中的）huffman重置
        ret = BzpHuffmanGroupsReset(huffman, mtf->nUse + BZP_EXTRA_CHARS_NUM);
        if (ret != BZP_OK) {
            return ret;
        }
        // mtf to huffman
        huffman->block = mtf->mtfV;
        huffman->mtfFreq = mtf->mtfFreq;
        huffman->nBlock = mtf->nMtf;
        // huffman main
        BzpHuffmanMain(huffman);
        // 每个块的首部 标志位+校验码+0+bwt解码起始位置
        BzpWriteBlockHead(outData, bwt);
        // 输入序列使用了哪些字符。
        BzpWriteValidASCII(outData, bwt);
        // nGroups huffman tree number
        BzpWriteToArray(huffman->nGroups, BZP_BITS3, outData);
        // write select
        BzpWriteSelect(outData, huffman);
        // nGroups trees len
        BzpWriteLen(outData, huffman);
        // huffman code map input
        BzpWriteInputEncode(outData, mtf, huffman);
    }
    return BZP_OK;
}
// 将缓冲数组中的数据写入到输出流中
int32_t BzpBuffToStream(BzpFile *bzpf, BzpOutComdata *outData)
{
    bzpf->output->pos = 0;
    // 这里有个5k的buff
    // 将压缩后的数据，每次写5k到这个数组里面，然后满了就去写数据流
    int32_t pos = 0;
    // 将outdata中的数据写入buf了。
    while (pos < outData->num) {
        bzpf->output->nBuf = 0;
        // 将data的数据写5k到buff
        while (pos < outData->num && bzpf->output->nBuf < BZP_BUF_SIZE) {
            bzpf->output->buf[bzpf->output->nBuf++] = outData->out[pos];
            pos++;
        }
        int32_t n2 = fwrite((void *)(bzpf->output->buf), sizeof(uint8_t), bzpf->output->nBuf, bzpf->output->filePtr);
        if (n2 != bzpf->output->nBuf) {
            return BZP_ERROR_IO;
        }
    }
    return BZP_OK;
}
void BzpAddCharToBlock(uint8_t lasch, int32_t num, BzpBwtInfo *bwt)
{
    if (num < BZP_RLC_NUM_LOWER_LIMIT || num > BZP_RLC_NUM_UPPER_LIMIT) {
        return;
    }
    for (int32_t i = 0; i < num; i++) { // 计算crc
        BZP_UPDATE_CRC(bwt->blockCRC, lasch);
    }

    // 分情况写入block 这里做了个游程编码
    int32_t val = BZP_MIN_FUN(num, (int32_t)BZP_RLC_NUM_4);
    switch (val) {
        case BZP_RLC_NUM_4:
            bwt->block[bwt->nBlock++] = lasch;
            /* fallthru */
        case BZP_RLC_NUM_3:
            bwt->block[bwt->nBlock++] = lasch;
            /* fallthru */
        case BZP_RLC_NUM_2:
            bwt->block[bwt->nBlock++] = lasch;
            /* fallthru */
        case BZP_RLC_NUM_1:
            bwt->block[bwt->nBlock++] = lasch;
            /* fallthru */
        default:
            break;
    }
    if (num >= BZP_RLC_NUM_4) {
        bwt->block[bwt->nBlock++] = ((char)(num - BZP_RLC_NUM_4));
        bwt->inUse[num - BZP_RLC_NUM_4] = true;
    }

    // 维护inUse。
    bwt->inUse[lasch] = true;
}
// 从缓冲区中将数据写入到块中(进行游程编码)，这里用于输入数据分块压缩
void BzpBuffToBlockRLC(BzpFile *bzpf, BzpBwtInfo *bwt, bool IsLastdata)
{
    // 这里block上限要小一些类似于-19，因为这里RLC一次性可能写入多个字符，用满有可能导致数组越界。
    while (!BZP_BLOCK_FULL(bwt) && !BZP_BUFF_READ_EMPTY(bzpf)) {
        int32_t pos = bzpf->input->pos;
        uint8_t ch = (uint8_t)bzpf->input->buf[pos];
        uint8_t lasch = (uint8_t)bzpf->lasChar;
        if (ch != lasch || bzpf->num == BZP_RLC_NUM_UPPER_LIMIT) {
            BzpAddCharToBlock(lasch, bzpf->num, bwt);
            bzpf->lasChar = ch;
            bzpf->num = 1;
        } else {
            bzpf->num++;
        }

        bzpf->input->pos++;
    }

    if (IsLastdata && BZP_BUFF_READ_EMPTY(bzpf)) {
        // 将维护的最后的字符lastchar写入块中
        BzpAddCharToBlock(bzpf->lasChar, bzpf->num, bwt);
        bzpf->lasChar = BZP_ASCII_SIZE;
        bzpf->num = 0;
    }
}
// 单个块处理完后，重置一些信息
void BzpResetCompress(BzpBwtInfo *bwt, BzpOutComdata *outData)
{
    // 输出变量
    outData->num = 0;
    // 输入变量
    bwt->nBlock = 0;
    bwt->blockCRC = BZP_INIT_BLOCK_CRC;
    (void)memset_s(bwt->inUse, sizeof(bwt->inUse), 0, sizeof(bwt->inUse));
    int32_t n = outData->blockSize * BZP_BASE_BLOCK_SIZE * sizeof(int32_t);
    (void)memset_s(bwt->isStartPos, n, 0, n);
    bwt->blockId++; // 块ID
}
// 对压缩数据的处理，分成输入状态和输出状态
int32_t BzpProcessData(BzpAlgorithmInfo *bzpInfo, bool IsLastdata)
{
    BzpFile *bzpf = bzpInfo->compressFile;
    BzpOutComdata *outData = bzpInfo->outData;
    BzpBwtInfo *bwt = bzpInfo->bwt;

    bzpf->state = BZP_INPUT_COMPRESS;
    int32_t ret = BZP_OK;
    while (bzpf->state != BZP_RETUEN_COMPRESS) {
        if (bzpf->state == BZP_OUTPUT_COMPRESS) {
            // 压缩结果写入输出流 数组存储的压缩结果写入缓冲区再写入输出流
            ret = BzpBuffToStream(bzpf, outData);
            // 初始化块信息
            BzpResetCompress(bwt, outData);
            bzpf->state = BZP_INPUT_COMPRESS;
            if (IsLastdata && BZP_BUFF_READ_EMPTY(bzpf)) {
                bzpf->state = BZP_RETUEN_COMPRESS;
            }
        }
        if (bzpf->state == BZP_INPUT_COMPRESS) {
            // 从buf将数据写入block
            BzpBuffToBlockRLC(bzpf, bwt, IsLastdata);
            // 判断block和buff的状态， 最后一个块，Block满
            if (IsLastdata && BZP_BUFF_READ_EMPTY(bzpf)) { // buf读空说明 要处理最后一个块。
                ret = BzpCompressOneBlock(bzpInfo, outData);
                // 判断最后一个块，然后输出尾部信息。
                BzpWriteFileEnd(outData, bwt->combinedCRC);
                BzpFlushbuf(outData);
                // 写入最后一个块的内容,状态调整，准备写入输出流
                bzpf->state = BZP_OUTPUT_COMPRESS;
            } else if (BZP_BLOCK_FULL(bwt)) {
                ret = BzpCompressOneBlock(bzpInfo, outData);
                bzpf->state = BZP_OUTPUT_COMPRESS;
            } else {
                bzpf->state = BZP_RETUEN_COMPRESS;
            }
            // 前两个压缩，buf空跳出去读数据
        }
        if (ret != BZP_OK) {
            return ret;
        }
    }
    return ret;
}
void BzpCompressEnd(BzpAlgorithmInfo *bzpInfo)
{
    // 压缩结束，释放资源,关闭输入流，申请空间释放
    if (bzpInfo->compressFile->input->filePtr != NULL) {
        fclose(bzpInfo->compressFile->input->filePtr);
    }
    if (bzpInfo->compressFile->output->filePtr != NULL) {
        fclose(bzpInfo->compressFile->output->filePtr);
    }
    BzpAlgorithmInfoFinish(bzpInfo);
}
// 压缩接口，输入文件路径和使用的块大小
int32_t BzpCompressStream(char *inName, char *outName, int32_t blockSize)
{
    // 检查输入
    int32_t ret = BZP_OK;
    bool IsLastdata = false;

    if (inName == NULL || outName == NULL || BZP_INVALID_BLOCK_SIZE(blockSize)) {
        return BZP_ERROR_PARAM;
    }
    // 初始化
    BzpAlgorithmInfo *bzpInfo = BzpAlgorithmInfoInit(blockSize);
    if (bzpInfo == NULL) {
        return BZP_ERROR_MEMORY_OPER_FAILURE;
    }
    ret = BzpOpenFile(bzpInfo, inName, outName);
    if (ret != BZP_OK) {
        return ret;
    }
    BzpStream *inStream = bzpInfo->compressFile->input;
    // 压缩数据  读满一次block压缩一个block
    while (!IsLastdata) {
        inStream->nBuf = fread(inStream->buf, sizeof(char), sizeof(inStream->buf), inStream->filePtr);
        inStream->pos = 0;
        IsLastdata = BzpFileEOF(inStream->filePtr);
        ret = BzpProcessData(bzpInfo, IsLastdata);
        if (ret != BZP_OK) {
            break;
        }
    }
    BzpCompressEnd(bzpInfo);
    if (ret != BZP_OK) {
        remove(outName);
    }
    return ret;
}

#ifdef __cplusplus
}
#endif
