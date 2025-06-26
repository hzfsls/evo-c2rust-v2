/**
 * @file bzp_huffman_decode.h
 * @copyright Copyright (c) Huawei Technologies Co., Ltd. 2023-2023. All rights reserved.
 * @brief 提供huffman解码的函数及结构体声明
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
#ifndef BZP_HUFFMAN_DECODE_H
#define BZP_HUFFMAN_DECODE_H

#include "bzp_utils.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef struct {
    int32_t *select;                                       // 每个组选择了哪棵树
    int32_t len[BZP_MAX_GROUPS_NUM][BZP_MAX_ALPHA_SIZE];   // 每个字符的编码长度
    int32_t perm[BZP_MAX_GROUPS_NUM][BZP_MAX_ALPHA_SIZE];  // 解码表
    int32_t limit[BZP_MAX_GROUPS_NUM][BZP_MAX_ALPHA_SIZE]; // 解码表
    int32_t base[BZP_MAX_GROUPS_NUM][BZP_MAX_ALPHA_SIZE];  // 解码表
    int32_t minLens[BZP_MAX_GROUPS_NUM];                   // 每棵树最小编码长度
    int32_t nGroups;                                       // 总共建了多少huffman树
    int32_t nSelect;                                       // select数组的元素数量
    int32_t alphaSize;                                     // huffman中用到的字符数量
    int32_t deCodeNum;                                     // 解码完成字符的数量，到50要调整组
    int32_t selectCnt;                                     // 当前属于哪个select的组
    int32_t nBlock;                                        // 解码字符用存了多少
} BzpHuffmanDecode;
// 初始化
BzpHuffmanDecode *BzpHuffmanDecodeInit(int32_t blockSize);
// 生成一个解码表
void BzpGetOneTable(BzpHuffmanDecode *huffman, int32_t t);
// 生成N个解码表
void BzpGenerateDecodeTable(BzpHuffmanDecode *huffman);
// 资源释放
void BzpHuffmanDecodeFinish(BzpHuffmanDecode *huffman);
// 重置结构体内容
void BzpHuffmanDecodeReset(BzpHuffmanDecode *huffman);

#ifdef __cplusplus
}
#endif

#endif
