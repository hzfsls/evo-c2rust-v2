/**
 * @file bzp_huffman_encode.h
 * @copyright Copyright (c) Huawei Technologies Co., Ltd. 2023-2023. All rights reserved.
 * @brief 提供huffman编码函数及结构体声明
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
#ifndef BZP_HUFFMAN_ENCODE_H
#define BZP_HUFFMAN_ENCODE_H

#include "bzp_utils.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef struct {
    int32_t heap[BZP_MAX_ALPHA_SIZE + 1]; // 堆，构建Huffman树时用于查询最小的节点 从下标为1开始使用
    int32_t weight[BZP_MAX_ALPHA_SIZE * 2]; // 权重数组
    int32_t parent[BZP_MAX_ALPHA_SIZE * 2]; // 每个节点的父节点
    int32_t len[BZP_MAX_ALPHA_SIZE];        // 每个字符的编码长度
    int32_t table[BZP_MAX_ALPHA_SIZE];      // 每个字符的新编码
    int32_t nHeap;                          // 堆使用的大小
    int32_t nWeight;                        // 权重数组使用的大小
    int32_t alphaSize;                      // 表中的字符数量，在调整树高度时会用到
} BzpHuffmanInfo;
// 单个huffman树的初始化
void BzpHuffmanInit(int32_t alphaSize, BzpHuffmanInfo *huffman);
// 建树前重新初始化数组
void BzpHuffmanInitArray(BzpHuffmanInfo *huffman);
// 堆的向上调整-初始化堆的过程中
void BzpHeapAdjustUp(int32_t *heap, int32_t *weight, int32_t pos);
// 堆的向下调整-向堆中添加删除元素的的过程中
void BzpHeapAdjustDown(int32_t *heap, int32_t *weight, int32_t nHeap);
// 堆初始化
void BzpHeapInit(BzpHuffmanInfo *huffman);
// 节点权重加和
int32_t BzpHuffmanWeightAdd(int32_t w1, int32_t w2);
// 建树
void BzpBuildHuffmanTree(BzpHuffmanInfo *huffman);
// 建树并求得树的高度
int32_t BzpGetCodeLen(BzpHuffmanInfo *huffman);
// 调用建树求高度的函数，用来限制树的高度
void BzpBuildTreeBalanceHeight(BzpHuffmanInfo *huffman);
// 求编码表
void BzpGetHuffmanTable(BzpHuffmanInfo *huffman);
// 根据块内元素数量确定建树的数量
int32_t BzpGetHuffmanGroups(int32_t nBlock);

typedef struct {
    int32_t *block;     // mtf编码结果，mtfv[BZP_BASE_BLOCK_SIZE]
    int32_t *mtfFreq;   // mtf编码的各个字符出现次数  EOB+1  EOB上限为256 [257]
    int32_t *select;    // 每个组选哪个树
    int32_t *selectMTF; // select经过Mtf编码的结果
    BzpHuffmanInfo huffmanGroups[BZP_MAX_GROUPS_NUM]; // 建树用的
    int32_t cost[BZP_MAX_GROUPS_NUM]; // 统计花费，用于判断一组数据在每个树上的花费，来选择最优的树
    int32_t nGroups;                  // 建的Huffman树的数量
    int32_t nBlock;                   // block中数据数量
    int32_t nSelect;                  // select中元素数量
    int32_t alphaSize;                // block中字符种类数
} BzpHuffmanGroups;
// 建立一组huffman树，初始化
BzpHuffmanGroups *BzpHuffmanGroupsInit(int32_t blockSize);
// 建立一组huffman树-主要流程
void BzpHuffmanMain(BzpHuffmanGroups *huffman);
// 资源释放
void BzpBzpHuffmanGroupsFinish(BzpHuffmanGroups *huffman);
// 将select数组用mtf方式编码
void BzpGenerateSelectMTF(BzpHuffmanGroups *huffman);
// 初始化Len数组
void BzpInitLenArray(BzpHuffmanGroups *huffman);
// 计算一段区间在每个树上的花费
void BzpCalculateCost(BzpHuffmanGroups *huffman, int32_t st, int32_t ed);
// 为这段区间选择一个最合适的树进行编码
int32_t BzpSelectTree(BzpHuffmanGroups *huffman);

// 重置
int32_t BzpHuffmanGroupsReset(BzpHuffmanGroups *huffman, int32_t alphaSize);

#ifdef __cplusplus
}
#endif

#endif
