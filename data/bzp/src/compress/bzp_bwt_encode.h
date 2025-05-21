/**
 * @file bzp_bwt_encode.h
 * @copyright Copyright (c) Huawei Technologies Co., Ltd. 2023-2023. All rights reserved.
 * @brief 提供bwt变换函数及结构体声明
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
#ifndef BZP_BWT_ENCODE_H
#define BZP_BWT_ENCODE_H

#include "bzp_utils.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef struct {
    int32_t *sortBlock; // 排序后的结果在原序列中的位置，sortBlock[i]=j表示排序后序列的第i个元素是原序列中第j个
    int32_t *idx;       // 维护第二关键字的序号，用于排序时进行比较
    int32_t *isStartPos;        // 维护区间首部的位置 [BZP_BASE_BLOCK_SIZE] bool
    uint8_t *block;             // 输入字符序列
    uint32_t blockCRC;          // 每个块的循环校验码
    uint32_t combinedCRC;       // 所有块合并的循环校验码
    int32_t nBlockMax;          // 每个块所申请的空间大小，level*基础块大小-余留空间
    int32_t blockId;            // 块编码，第几个块
    int32_t nBlock;             // 输入字符序列长度
    int32_t oriPtr;             // 起始字符的位置
    bool inUse[BZP_ASCII_SIZE]; // 每个ASCII是否出现过
} BzpBwtInfo;
// 初始化
BzpBwtInfo *BzpBlockSortInit(int32_t blockSize);
// 希尔排序
void BzpShellSort(int32_t *sortBlock, int32_t *idx, int32_t l, int32_t r);
// 交换2个元素的位置
void BzpSwap2Elem(int32_t *sortBlock, int32_t lPos, int32_t rPos);
// 交换3个元素的位置
void BzpSwap3Elem(int32_t *sortBlock, int32_t lPos, int32_t ePos, int32_t rPos);
// 选择快排需要的基准元素
int32_t BzpSelectMidVal(int32_t *sortBlock, int32_t *idx, int32_t l, int32_t r);
// 快排 排序区间[l,r]
void BzpQuickSort(int32_t *sortBlock, int32_t *idx, int32_t l, int32_t r);
// 基于倍增思想的排序
void BzpBinaryLiftingSort(BzpBwtInfo *bwt);
// 调用排序的函数，求解码起始位置
void BzpBlockSortMain(BzpBwtInfo *bwt);
// 释放资源
void BzpBwtFinish(BzpBwtInfo *bwt);
// 更新区间标志位
void BzpUpdateflag(BzpBwtInfo *bwt, int32_t l, int32_t r);

typedef struct {
    int32_t stackL[BZP_MAX_STACK_SIZE]; // 栈 存储排序左端点
    int32_t stackR[BZP_MAX_STACK_SIZE]; // 栈 存储排序右端点
    int32_t cnt;                        // 栈中元素数量
    int32_t tl, tr;                     // 单轮排序区间
} BzpQSortInfo;
// 单轮快排
void BzpQSortSingle(int32_t *sortBlock, int32_t *idx, BzpQSortInfo *stack);

#ifdef __cplusplus
}
#endif

#endif
