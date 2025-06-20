/**
 * @file bzp_decompress_stream.h
 * @copyright Copyright (c) Huawei Technologies Co., Ltd. 2023-2023. All rights reserved.
 * @brief 提供bzp解码相关的结构体、函数的声明
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
#ifndef BZP_DECOM_STREAM_H
#define BZP_DECOM_STREAM_H

#include "bzp_bwt_decode.h"
#include "bzp_huffman_decode.h"
#include "bzp_stream_utils.h"
#include "bzp_type.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef struct {
    BzpStream *input;
    BzpStream *output;
    int32_t lasChar;   // 上个字符是什么
    int32_t num;       // 出现多少次
    uint32_t buf;      // 要求判断32位？ 32位缓冲区
    int32_t nBuf;      // buf中存了多少位，从高位开始
    int32_t blockSize; // 输入块的大小
    uint32_t blockCRC;
    int32_t list[BZP_ASCII_SIZE];
} InDeComdata;

// 初始化
InDeComdata *BzpInDeComdataInit();
// 资源释放
void BzpInDeComdataFinish(InDeComdata *inData);
// 从输入流中每次读n位
uint32_t BzpReadBits(int32_t nBit, InDeComdata *inData);
// 向输出流中写一个uchar类型的数据
int32_t BzpWriteChar(uint8_t ch, InDeComdata *inData);
// 从流中读，然后每次返回一个huffman解码的字符
int32_t BzpHuffmanDecodeStep(BzpHuffmanDecode *huffman, InDeComdata *inData);
// 解压缩单个块的流程
int32_t BzpDeCompressOneBlock(InDeComdata *inData, BzpHuffmanDecode *huffman, BzpBwtDecodeInfo *debwt);
// 读文件结尾
int32_t BZPReadFileEnd(InDeComdata *inData, uint32_t caltotalCRC);
// 对数据进行解压缩，这里循环调用处理多个块
int32_t BZPDeCompressData(InDeComdata *inData);
void BzpDeComStreamFinish(InDeComdata *inData, BzpStream *inStream, BzpStream *outStream);
// 解码调用接口，输入为文件路径
int32_t BzpDeCompressStream(char *inName, char *outName);
// 检查文件首部信息
int32_t BzpCheckFileHead(InDeComdata *inData);
// 从输入流中读32位信息
uint32_t BzpReadUInt32(InDeComdata *inData);
// 从输入流中读24位信息
uint32_t BzpReadUInt24(InDeComdata *inData);
// 解码select信息
int32_t BzpDeHuffmanSelect(InDeComdata *inData, BzpHuffmanDecode *huffman);
// 解码Len信息
int32_t BzpDeHuffmanLen(InDeComdata *inData, BzpHuffmanDecode *huffman);
// MTF解码
int32_t BzpMTFDeCode(InDeComdata *inData, BzpHuffmanDecode *huffman, BzpBwtDecodeInfo *debwt);
// 进行RLC解码并将结果写入数据流
int32_t BzpDeCodeToStream(InDeComdata *inData, BzpBwtDecodeInfo *debwt);
// 从输入流中获取使用字符集列表
int32_t BzpGetDictionaryList(InDeComdata *inData);

#ifdef __cplusplus
}
#endif

#endif
