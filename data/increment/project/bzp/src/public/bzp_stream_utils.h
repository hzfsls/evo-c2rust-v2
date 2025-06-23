/**
 * @file bzp_stream_utils.h
 * @copyright Copyright (c) Huawei Technologies Co., Ltd. 2023-2023. All rights reserved.
 * @brief 提供bzp流处理相关的结构体、函数、宏定义
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
#ifndef BZP_STREAM_UTILS_H
#define BZP_STREAM_UTILS_H

#include <stdint.h>
#include <stdio.h>
#include "securec.h"

#ifdef __cplusplus
extern "C" {
#endif

// file head 文件头信息
#define BZP_HDR_B 0x42 // 'B'
#define BZP_HDR_Z 0x5a // 'Z'
#define BZP_HDR_H 0x68 // 'h'
#define BZP_HDR_0 0x30 // '0'
// block head 块的头部信息
#define BZP_BLOCK_HEAD_0 0x31
#define BZP_BLOCK_HEAD_1 0x41
#define BZP_BLOCK_HEAD_2 0x59
#define BZP_BLOCK_HEAD_3 0x26
#define BZP_BLOCK_HEAD_4 0x53
#define BZP_BLOCK_HEAD_5 0x59

// file end 文件尾部信息
#define BZP_FILE_END_0 0x17
#define BZP_FILE_END_1 0x72
#define BZP_FILE_END_2 0x45
#define BZP_FILE_END_3 0x38
#define BZP_FILE_END_4 0x50
#define BZP_FILE_END_5 0x90

// 缓冲区大小
#define BZP_BUF_SIZE 5000
// 文件终止符
#define BZP_EOF (-1)

// read or write bits 操作的比特位的数量
#define BZP_BIT 1
#define BZP_BITS2 2
#define BZP_BITS3 3
#define BZP_BITS5 5
#define BZP_BITS8 8
#define BZP_BITS15 15
#define BZP_BITS16 16
#define BZP_BITS24 24
#define BZP_BITS32 32

// Run Length Coding 游程编码时用到的判断数值
#define BZP_RLC_NUM_1 1
#define BZP_RLC_NUM_2 2
#define BZP_RLC_NUM_3 3
#define BZP_RLC_NUM_4 4
// 游程编码处理的数值的上下限
#define BZP_RLC_NUM_LOWER_LIMIT 1
#define BZP_RLC_NUM_UPPER_LIMIT 255
// ASCII码分成16组
#define BZP_GROUPS_ASCII 16
// 每组6个ASCII码
#define BZP_CHARS_PER_GROUP_ASCII 16
// CRC 校验码右移位数
#define BZP_CRC_MOVE_RIGHT_VAL 31
// huffman huffman编码len数组存储时的数值，比前一位增加存2否则存3
#define BZP_HUFFMAN_LEN_INCREASE 2
#define BZP_HUFFMAN_LEN_REDUCED 3
// huffman huffman编码可能出现的字符种类比原始字符串多几个
#define BZP_EXTRA_CHARS_NUM 2
// 压缩时，单个压缩块中的数据已经写满
#define BZP_BLOCK_FULL(bwt) (bwt->nBlock >= bwt->nBlockMax)
// 压缩时，从缓冲区读数据，缓冲区被读空
#define BZP_BUFF_READ_EMPTY(bzpf) (bzpf->input->pos >= bzpf->input->nBuf)
// 处理过程中出错

typedef struct {
    FILE *filePtr;             // 文件流
    int32_t nBuf;              // buf中数据的数量
    int32_t pos;               // 当前读取的位置（输入流中）
    uint8_t buf[BZP_BUF_SIZE]; // 缓冲区
} BzpStream;

// 循环校验码用到的表
extern int32_t g_bzpCRC32Table[256];
// 流初始化
BzpStream *BzpStreamInit();
// 处理流结束
void BzpStreamFinish(BzpStream *stream);
// 更新循环校验码
#define BZP_UPDATE_CRC(crcVar, cha)                                                        \
    {                                                                                      \
        (crcVar) = ((crcVar) << 8) ^ g_bzpCRC32Table[((crcVar) >> 24) ^ ((uint8_t)(cha))]; \
    }

#ifdef __cplusplus
}
#endif

#endif
