/**
 * @file bzp_compress_stream.h
 * @copyright Copyright (c) Huawei Technologies Co., Ltd. 2023-2023. All rights reserved.
 * @brief 提供bzp编码相关的宏定义、结构体、函数的声明
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
#ifndef BZP_COM_STREAM_H
#define BZP_COM_STREAM_H

#include "bzp_bwt_encode.h"
#include "bzp_huffman_encode.h"
#include "bzp_mtf_encode.h"
#include "bzp_stream_utils.h"
#include "bzp_type.h"

#ifdef __cplusplus
extern "C" {
#endif

#define BZP_INPUT_COMPRESS 0 // 压缩过程中_输入状态
#define BZP_OUTPUT_COMPRESS 1 // 压缩过程中_输出状态
#define BZP_RETUEN_COMPRESS 2 // 压缩过程中_退出状态

typedef struct {
    BzpStream *input;  // 输入流
    BzpStream *output; // 输出流
    int32_t state;     // 当前处于状态（输入或者输出）
    int32_t lasChar;   // 上个字符是什么
    int32_t num;       // 出现多少次
    int32_t pad;       // 用于填充对齐
} BzpFile;

typedef struct {
    uint8_t *out;      // 存储所有压缩数据[BZP_BASE_BLOCK_SIZE*3]
    int32_t num;       // out中存的数据量
    uint32_t buf;      // 32位缓冲区
    int32_t nBuf;      // buf中存了多少位，从高位开始
    int32_t blockSize; // 输入块的大小
} BzpOutComdata;

typedef struct {
    BzpBwtInfo *bwt;
    BzpHuffmanGroups *huffman;
    BzpMtfInfo *mtf;
    BzpFile *compressFile;
    BzpOutComdata *outData;
} BzpAlgorithmInfo;

BzpAlgorithmInfo *BzpAlgorithmInfoInit(int32_t blockSize);
int32_t BzpOpenFile(BzpAlgorithmInfo *bzpInfo, char *inName, char *outName);
void BzpAlgorithmInfoFinish(BzpAlgorithmInfo *bzpInfo);
// 初始化BzpOutComdata结构体
BzpOutComdata *BzpOutComDataInit(int32_t blockSize);
// 释放BzpOutComdata结构体资源
void BzpOutComDataFinish(BzpOutComdata *data);
// 将n位的数据写入到缓冲区或者缓冲的数组
void BzpWriteToArray(int32_t val, int32_t n, BzpOutComdata *data);
// 写32位数据
void BzpWriteInt32(int32_t val, BzpOutComdata *data);
// 对单个块的数据进行压缩
int32_t BzpCompressOneBlock(BzpAlgorithmInfo *bzpInfo, BzpOutComdata *outData);
// 将缓冲数组中的数据写入到输出流中
int32_t BzpBuffToStream(BzpFile *bzpf, BzpOutComdata *outData);
void BzpAddCharToBlock(uint8_t lasch, int32_t num, BzpBwtInfo *bwt);
// 从缓冲区中将数据写入到块中(进行游程编码)，这里用于输入数据分块压缩
void BzpBuffToBlockRLC(BzpFile *bzpf, BzpBwtInfo *bwt, bool IsLastdata);
// 单个块处理完后，重置一些信息
void BzpResetCompress(BzpBwtInfo *bwt, BzpOutComdata *outData);
// 对压缩数据的处理，分成输入状态和输出状态
int32_t BzpProcessData(BzpAlgorithmInfo *bzpInfo, bool IsLastdata);
// 压缩接口，输入文件路径和使用的块大小
int32_t BzpCompressStream(char *inName, char *outName, int32_t blockSize);
// 压缩结束，释放资源,关闭输入流
void BzpCompressEnd(BzpAlgorithmInfo *bzpInfo);
// BzpFile初始化
BzpFile *BzpFileInit();
// BzpFile释放资源
void BzpFileFinish(BzpFile *bzpF);
// 写文件头
void BzpWriteFileHead(BzpOutComdata *outData, int32_t blockId);
// 计算CRC
void BzpCalculateCRC(BzpBwtInfo *bwt);
// 每个块的首部 标志位+校验码+0+bwt解码起始位置
void BzpWriteBlockHead(BzpOutComdata *outData, BzpBwtInfo *bwt);
// 输出文件尾部信息。
void BzpWriteFileEnd(BzpOutComdata *outData, int32_t combinedCRC);
// 刷新输出32缓冲区 从buf写入到数组
void BzpFlushbuf(BzpOutComdata *outData);
// 将原始数据出现的字符写入缓冲区
void BzpWriteValidASCII(BzpOutComdata *outData, BzpBwtInfo *bwt);
// 写Select数组的信息
void BzpWriteSelect(BzpOutComdata *outData, BzpHuffmanGroups *huffman);
// 写Len数组的信息
void BzpWriteLen(BzpOutComdata *outData, BzpHuffmanGroups *huffman);
// 将输入进行huffman编码后的值写入缓冲区
void BzpWriteInputEncode(BzpOutComdata *outData, BzpMtfInfo *mtf, BzpHuffmanGroups *huffman);
// 判断流中数据是否读完
bool BzpFileEOF(FILE *f);

#ifdef __cplusplus
}
#endif

#endif
