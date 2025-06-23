/**
 * @file bzp_decompress_stream.c
 * @copyright Copyright (c) Huawei Technologies Co., Ltd. 2023-2023. All rights reserved.
 * @brief 提供bzp解压功能
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
#include "bzp_decompress_stream.h"

#ifdef __cplusplus
extern "C" {
#endif

InDeComdata *BzpInDeComdataInit()
{
    InDeComdata *inData = (InDeComdata *)malloc(sizeof(InDeComdata));
    if (inData == NULL) {
        return NULL;
    }
    inData->input = NULL;
    inData->output = NULL;
    inData->num = 0;
    inData->lasChar = BZP_ASCII_SIZE; // 初始化为无效字符 用于RLC
    inData->nBuf = 0;
    inData->buf = 0;
    inData->num = 0;

    inData->blockCRC = BZP_INIT_BLOCK_CRC;
    return inData;
}
void BzpInDeComdataFinish(InDeComdata *inData)
{
    if (inData != NULL) {
        free(inData);
        inData = NULL;
    }
}
// 从输入流中每次读n位
uint32_t BzpReadBits(int32_t nBit, InDeComdata *inData)
{
    uint32_t res = 0;
    // 从32bit缓冲区读  从5k缓冲区读，从流读
    while (inData->nBuf < nBit) {
        if (inData->input->nBuf == inData->input->pos) {
            inData->input->nBuf =
                fread(inData->input->buf, sizeof(char), sizeof(inData->input->buf), inData->input->filePtr);
            inData->input->pos = 0;
        }
        int32_t data = ((uint32_t)(inData->input->buf[inData->input->pos]));
        // 数据存放在低nBuf位
        inData->buf = (inData->buf << BZP_BITS8) | data;
        inData->input->pos++;
        inData->nBuf += BZP_BITS8;
    }
    res = inData->buf >> (inData->nBuf - nBit);
    res = res & ((1 << nBit) - 1);
    inData->nBuf -= nBit;
    return res;
}
// 向输出流中写一个uchar类型的数据
int32_t BzpWriteChar(uint8_t ch, InDeComdata *inData)
{
    int32_t ret = BZP_OK;
    if (inData->output->nBuf >= BZP_BUF_SIZE) {
        int32_t n2 =
            fwrite((void *)(inData->output->buf), sizeof(uint8_t), inData->output->nBuf, inData->output->filePtr);
        if (n2 != inData->output->nBuf) {
            ret = BZP_ERROR_IO;
        }
        inData->output->nBuf = 0;
    }
    inData->output->buf[inData->output->nBuf++] = ch;
    return ret;
}
// 从流中读，然后每次返回一个huffman解码的字符
int32_t BzpHuffmanDecodeStep(BzpHuffmanDecode *huffman, InDeComdata *inData)
{
    // 每50个字符换个组
    if (huffman->deCodeNum == BZP_ELEMS_NUM_IN_ONE_GROUP) {
        huffman->deCodeNum = 0;
        huffman->selectCnt++;
    }
    int32_t gid = huffman->select[huffman->selectCnt];

    // 从输入流中读取
    int32_t chlen = huffman->minLens[gid];
    int32_t val = BzpReadBits(chlen, inData);

    while (chlen < BZP_HUFFMAN_LEN_UPPER_LIMIT && val > huffman->limit[gid][chlen]) {
        chlen++;
        int32_t nxtbit = BzpReadBits(1, inData);
        val = (val << 1) | nxtbit;
    }
    if (chlen > BZP_HUFFMAN_LEN_UPPER_LIMIT) {
        return -1;
    }

    // 解码
    val = val - huffman->base[gid][chlen];
    val = huffman->perm[gid][val];
    huffman->deCodeNum++;
    return val;
}
// 检查文件首部信息
int32_t BzpCheckFileHead(InDeComdata *inData)
{
    uint8_t ch;
    ch = BzpReadBits(BZP_BITS8, inData);
    if (ch != BZP_BLOCK_HEAD_1) {
        return BZP_ERROR_DATA;
    }
    ch = BzpReadBits(BZP_BITS8, inData);
    if (ch != BZP_BLOCK_HEAD_2) {
        return BZP_ERROR_DATA;
    }
    ch = BzpReadBits(BZP_BITS8, inData);
    if (ch != BZP_BLOCK_HEAD_3) {
        return BZP_ERROR_DATA;
    }
    ch = BzpReadBits(BZP_BITS8, inData);
    if (ch != BZP_BLOCK_HEAD_4) {
        return BZP_ERROR_DATA;
    }
    ch = BzpReadBits(BZP_BITS8, inData);
    if (ch != BZP_BLOCK_HEAD_5) {
        return BZP_ERROR_DATA;
    }
    return BZP_OK;
}

uint32_t BzpReadUInt24(InDeComdata *inData)
{
    uint8_t ch;
    uint32_t val = 0;
    ch = BzpReadBits(BZP_BITS8, inData);
    val = (val << BZP_BITS8) | ((uint32_t)ch);
    ch = BzpReadBits(BZP_BITS8, inData);
    val = (val << BZP_BITS8) | ((uint32_t)ch);
    ch = BzpReadBits(BZP_BITS8, inData);
    val = (val << BZP_BITS8) | ((uint32_t)ch);
    return val;
}

uint32_t BzpReadUInt32(InDeComdata *inData)
{
    uint8_t ch;
    uint32_t val = 0;
    ch = BzpReadBits(BZP_BITS8, inData);
    val = (val << BZP_BITS8) | ((uint32_t)ch);
    ch = BzpReadBits(BZP_BITS8, inData);
    val = (val << BZP_BITS8) | ((uint32_t)ch);
    ch = BzpReadBits(BZP_BITS8, inData);
    val = (val << BZP_BITS8) | ((uint32_t)ch);
    ch = BzpReadBits(BZP_BITS8, inData);
    val = (val << BZP_BITS8) | ((uint32_t)ch);
    return val;
}

int32_t BzpDeHuffmanSelect(InDeComdata *inData, BzpHuffmanDecode *huffman)
{
    uint8_t ch;

    int32_t selectmtf[BZP_HUFFMAN_MAX_SIZE_SELECT];
    for (int32_t i = 0; i < huffman->nSelect; i++) {
        int32_t j = -1;
        do {
            ch = BzpReadBits(BZP_BIT, inData);
            j++;
        } while (ch != 0);
        if (j >= huffman->nGroups) {
            return BZP_ERROR_DATA;
        }
        selectmtf[i] = j;
    }

    int32_t listSelect[BZP_MAX_GROUPS_NUM];
    for (int32_t i = 0; i < BZP_MAX_GROUPS_NUM; i++) {
        listSelect[i] = i;
    }

    for (int32_t i = 0; i < huffman->nSelect; i++) {
        int32_t pos = selectmtf[i];
        int32_t tmpv = listSelect[pos];
        for (int32_t j = pos; j > 0; j--) {
            listSelect[j] = listSelect[j - 1];
        }
        listSelect[0] = tmpv;
        huffman->select[i] = tmpv;
    }
    return BZP_OK;
}

int32_t BzpDeHuffmanLen(InDeComdata *inData, BzpHuffmanDecode *huffman)
{
    uint8_t ch;
    for (int32_t i = 0; i < huffman->nGroups; i++) {
        int32_t val = BzpReadBits(BZP_BITS5, inData);
        for (int32_t j = 0; j < huffman->alphaSize; j++) {
            // 读一位为0，则跳出。读一位不为0，则根据下一位的值处理
            ch = BzpReadBits(BZP_BIT, inData);
            while (ch != 0) {
                ch = BzpReadBits(BZP_BIT, inData);
                val += (ch == 0 ? 1 : -1);
                ch = BzpReadBits(BZP_BIT, inData);
            }
            if (val < 1 || val > BZP_HUFFMAN_LEN_UPPER_LIMIT) {
                return BZP_ERROR_DATA;
            }
            huffman->len[i][j] = val;
        }
    }
    return BZP_OK;
}

int32_t BzpMTFDeCode(InDeComdata *inData, BzpHuffmanDecode *huffman, BzpBwtDecodeInfo *debwt)
{
    debwt->nBlock = 0;
    uint8_t ch;
    int32_t ninUse = huffman->alphaSize - BZP_EXTRA_CHARS_NUM;
    int32_t eob = ninUse + 1;                            // 块终止符
    int32_t val = BzpHuffmanDecodeStep(huffman, inData); // val==-1表示解码错误，正数为解码值
    while (val != eob && val != -1) {
        if (val == 0 || val == 1) {
            int32_t res = 0, basenum = 1;
            while (val == 0 || val == 1) {
                res = res + (val + 1) * basenum;
                basenum <<= 1;
                val = BzpHuffmanDecodeStep(huffman, inData);
            }
            for (int32_t j = 0; j < res; j++) {
                debwt->block[debwt->nBlock++] = inData->list[0];
            }
        } else {
            int32_t pos = val - 1;
            ch = inData->list[pos];
            debwt->block[debwt->nBlock++] = ch;

            for (int32_t j = pos; j > 0; j--) {
                inData->list[j] = inData->list[j - 1];
            }
            inData->list[0] = ch;
            val = BzpHuffmanDecodeStep(huffman, inData);
        }
    }
    if (val == -1) {
        return BZP_ERROR_DATA;
    }
    return BZP_OK;
}
int32_t BzpDeCodeToStream(InDeComdata *inData, BzpBwtDecodeInfo *debwt)
{
    uint8_t ch;
    int32_t ret = BZP_OK;
    for (int32_t i = 0; i < debwt->nBlock; i++) {
        ch = debwt->deCode[i];
        if (inData->num == BZP_RLC_NUM_4) {
            for (int32_t j = 0; j < ((int32_t)ch); j++) {
                BZP_UPDATE_CRC(inData->blockCRC, (uint8_t)inData->lasChar);
                ret |= BzpWriteChar(inData->lasChar, inData);
            }
            inData->lasChar = BZP_ASCII_SIZE;
            inData->num = 0;
        } else if (ch == inData->lasChar) {
            BZP_UPDATE_CRC(inData->blockCRC, ch);
            ret = BzpWriteChar(ch, inData);
            inData->num++;
        } else {
            BZP_UPDATE_CRC(inData->blockCRC, ch);
            ret = BzpWriteChar(ch, inData);
            inData->lasChar = ch;
            inData->num = 1;
        }
        if (ret != BZP_OK)
            break;
    }
    return ret;
}

int32_t BzpGetDictionaryList(InDeComdata *inData)
{
    int32_t ninUse = 0;
    // 使用过的字符
    bool use16[16] = {0};
    bool inUse[BZP_ASCII_SIZE] = {0};
    for (int32_t i = 0; i < BZP_GROUPS_ASCII; i++) {
        use16[i] = BzpReadBits(BZP_BIT, inData);
    }
    for (int32_t i = 0; i < BZP_GROUPS_ASCII; i++) {
        if (use16[i]) {
            for (int32_t j = 0; j < BZP_CHARS_PER_GROUP_ASCII; j++) {
                inUse[i * BZP_GROUPS_ASCII + j] = BzpReadBits(BZP_BIT, inData);
            }
        }
    }

    for (int32_t i = 0; i < BZP_ASCII_SIZE; i++) {
        if (inUse[i]) {
            inData->list[ninUse++] = i;
        }
    }
    return ninUse;
}

// 解压缩单个块的流程
int32_t BzpDeCompressOneBlock(InDeComdata *inData, BzpHuffmanDecode *huffman, BzpBwtDecodeInfo *debwt)
{
    // 从流中解析数据
    int32_t ret = BZP_OK;
    BzpCheckFileHead(inData); // 校验块首和crc校验码
    uint32_t blockCRC = BzpReadUInt32(inData);
    // blockrandomised 保持兼容，无使用的地方
    (void)BzpReadBits(BZP_BIT, inData);

    // bwt解码起始位置
    int32_t oriPtr = BzpReadUInt24(inData);
    if (oriPtr < 0 || oriPtr > BZP_BASE_BLOCK_SIZE * inData->blockSize) {
        return BZP_ERROR_DATA;
    }

    // 使用过的字符
    int32_t ninUse = BzpGetDictionaryList(inData);
    huffman->alphaSize = ninUse + BZP_EXTRA_CHARS_NUM; // 字符种类 + EOB + 1（mtf用0,1来表示重复字符）
    huffman->nGroups = BzpReadBits(BZP_BITS3, inData);
    if (huffman->nGroups < BZP_NGROUPS_NUM_0 || huffman->nGroups > BZP_NGROUPS_NUM_4) {
        return BZP_ERROR_DATA;
    }
    huffman->nSelect = BzpReadBits(BZP_BITS15, inData);

    int32_t nSelectUpperLimit = (inData->blockSize * BZP_BASE_BLOCK_SIZE / BZP_ELEMS_NUM_IN_ONE_GROUP + 1);
    if (huffman->nSelect < 1 || huffman->nSelect > nSelectUpperLimit) {
        return BZP_ERROR_DATA;
    }

    // huffman中每50个数据选择的哪个表
    ret |= BzpDeHuffmanSelect(inData, huffman);
    // huffman len数组的信息
    ret |= BzpDeHuffmanLen(inData, huffman);
    if (ret != BZP_OK) {
        return ret;
    }
    // 根据len构建Huffman解码表
    BzpGenerateDecodeTable(huffman);
    // 解码

    debwt->oriPtr = oriPtr;
    ret = BzpMTFDeCode(inData, huffman, debwt);
    if (ret != BZP_OK || debwt->nBlock >= BZP_BASE_BLOCK_SIZE * inData->blockSize) {
        return BZP_ERROR_DATA;
    }

    BzpBwtDecode(debwt);
    // 下面将解码结果写入输出流，并计算crc
    ret = BzpDeCodeToStream(inData, debwt);
    if (ret != BZP_OK) {
        return ret;
    }
    inData->blockCRC = ~(inData->blockCRC);

    if (blockCRC != inData->blockCRC) {
        ret = BZP_ERROR_DATA;
    }

    return ret;
}

int32_t BZPReadFileEnd(InDeComdata *inData, uint32_t caltotalCRC)
{
    uint8_t ch = BzpReadBits(BZP_BITS8, inData);
    if (ch != BZP_FILE_END_1) {
        return BZP_ERROR_DATA;
    }
    ch = BzpReadBits(BZP_BITS8, inData);
    if (ch != BZP_FILE_END_2) {
        return BZP_ERROR_DATA;
    }
    ch = BzpReadBits(BZP_BITS8, inData);
    if (ch != BZP_FILE_END_3) {
        return BZP_ERROR_DATA;
    }
    ch = BzpReadBits(BZP_BITS8, inData);
    if (ch != BZP_FILE_END_4) {
        return BZP_ERROR_DATA;
    }
    ch = BzpReadBits(BZP_BITS8, inData);
    if (ch != BZP_FILE_END_5) {
        return BZP_ERROR_DATA;
    }

    uint32_t storedcombinedcrc = BzpReadUInt32(inData);
    // 检查storedcombinedcrc和计算的crc是否一致
    if (caltotalCRC != storedcombinedcrc) {
        return BZP_ERROR_DATA;
    }
    return BZP_OK;
}

int32_t BzpReadFileHead(InDeComdata *inData)
{
    // 开头
    uint8_t ch = BzpReadBits(BZP_BITS8, inData);
    if (ch != BZP_HDR_B) {
        return BZP_ERROR_DATA_MAGIC;
    }
    ch = BzpReadBits(BZP_BITS8, inData);
    if (ch != BZP_HDR_Z) {
        return BZP_ERROR_DATA_MAGIC;
    }
    ch = BzpReadBits(BZP_BITS8, inData);
    if (ch != BZP_HDR_H) {
        return BZP_ERROR_DATA_MAGIC;
    }
    ch = BzpReadBits(BZP_BITS8, inData);
    int32_t blockSize = ch - BZP_HDR_0;
    if (BZP_INVALID_BLOCK_SIZE(blockSize)) {
        return BZP_ERROR_DATA_MAGIC;
    }
    // 根据blocksize申请一下空间
    inData->blockSize = blockSize;
    return BZP_OK;
}

// 对数据进行解压缩，这里循环调用处理多个块
int32_t BZPDeCompressData(InDeComdata *inData)
{
    int32_t ret = BZP_OK;
    uint32_t caltotalCRC = 0;
    uint8_t ch;
    ret = BzpReadFileHead(inData);
    if (ret != BZP_OK) {
        return ret;
    }
    BzpHuffmanDecode *huffman = BzpHuffmanDecodeInit(inData->blockSize);
    BzpBwtDecodeInfo *debwt = BzpBwtDecodeInit(inData->blockSize);

    while ((ch = BzpReadBits(BZP_BITS8, inData)) != BZP_FILE_END_0) {
        if (ch != BZP_BLOCK_HEAD_0) {
            ret = BZP_ERROR_DATA;
            break;
        }
        BzpHuffmanDecodeReset(huffman);
        inData->blockCRC = BZP_INIT_BLOCK_CRC;

        // 先一直从流中读数据，处理完一个块， 将这个块处理的结果写入输出流
        ret = BzpDeCompressOneBlock(inData, huffman, debwt);
        if (ret != BZP_OK) {
            break;
        }

        caltotalCRC = (caltotalCRC << 1) | (caltotalCRC >> BZP_CRC_MOVE_RIGHT_VAL);
        caltotalCRC ^= inData->blockCRC;
    }
    if (ret == BZP_OK) {
        ret = BZPReadFileEnd(inData, caltotalCRC);
    }
    BzpHuffmanDecodeFinish(huffman);
    BzpBwtDecodeFinish(debwt);
    return ret;
}

void BzpDeComStreamFinish(InDeComdata *inData, BzpStream *inStream, BzpStream *outStream)
{
    // 关闭输入流，申请空间释放
    if (inStream->filePtr != NULL) {
        fclose(inStream->filePtr);
        inStream->filePtr = NULL;
    }
    if (outStream->filePtr != NULL) {
        fclose(outStream->filePtr);
        outStream->filePtr = NULL;
    }
    BzpStreamFinish(inStream);
    BzpStreamFinish(outStream);
    BzpInDeComdataFinish(inData);
}
// 解码调用接口，输入为文件路径
int32_t BzpDeCompressStream(char *inName, char *outName)
{
    int32_t ret = BZP_OK;
    if (inName == NULL || outName == NULL) {
        return BZP_ERROR_PARAM;
    }
    // 建立输入，输出流
    BzpStream *inStream = BzpStreamInit();
    BzpStream *outStream = BzpStreamInit();
    if (inStream == NULL || outStream == NULL) {
        BzpStreamFinish(inStream);
        BzpStreamFinish(outStream);
        return BZP_ERROR_MEMORY_OPER_FAILURE;
    }
    inStream->filePtr = fopen(inName, "rb");
    outStream->filePtr = fopen(outName, "wb");
    if ((inStream->filePtr == NULL || outStream->filePtr == NULL)) {
        free(inStream);
        inStream = NULL;
        free(outStream);
        outStream = NULL;
        remove(outName);
        return BZP_ERROR_IO;
    }
    InDeComdata *inData = BzpInDeComdataInit();
    if (inData == NULL) {
        BzpDeComStreamFinish(inData, inStream, outStream);
        remove(outName);
        return BZP_ERROR_MEMORY_OPER_FAILURE;
    }
    inData->input = inStream;
    inData->output = outStream;
    // 解码
    ret = BZPDeCompressData(inData);
    // 刷新缓冲区
    if (inData->output->nBuf > 0) {
        int32_t n2 =
            fwrite((void *)(inData->output->buf), sizeof(uint8_t), inData->output->nBuf, inData->output->filePtr);
        if (n2 != inData->output->nBuf) {
            ret = BZP_ERROR_IO;
        }
        inData->output->nBuf = 0;
    }

    BzpDeComStreamFinish(inData, inStream, outStream);
    if (ret != BZP_OK) {
        remove(outName);
    }
    return ret;
}

#ifdef __cplusplus
}
#endif
