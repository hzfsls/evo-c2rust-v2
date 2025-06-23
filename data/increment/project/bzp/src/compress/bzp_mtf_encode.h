/**
 * @file bzp_mtf_encode.h
 * @copyright Copyright (c) Huawei Technologies Co., Ltd. 2023-2023. All rights reserved.
 * @brief 提供mtf变换函数及结构体声明
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
#ifndef BZP_MTF_ENCODE_H
#define BZP_MTF_ENCODE_H

#include "bzp_utils.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef struct {
    uint8_t *block;                      // 输入原始字符序列
    int32_t *map;                        // 映射关系，排序后结果到原始字符序列的映射
    int32_t *mtfV;                       // mtf编码值 [BZP_BASE_BLOCK_SIZE]
    bool *inUse;                         // 每个字符使用情况
    int32_t mtfFreq[BZP_MAX_ALPHA_SIZE]; // mtf编码结果中每个字符出现次数
    int32_t nBlock;                      // 输入字符序列长度
    int32_t nMtf;                        // mtf编码结果的个数
    int32_t nUse;                        // 输入序列中出现的字符种类
    int32_t pad;                         // 用于填充对齐
} BzpMtfInfo;
// 初始化
BzpMtfInfo *BzpMtfInit(int32_t blockSize);
// 统计使用字符，构造字典列表
void BzpMapInputChar(BzpMtfInfo *mtf, uint8_t *list, int32_t lenList);
// 将一个正整数进行编码
void BzpNumEncode(BzpMtfInfo *mtf, int32_t num);
// mtf变换主流程
void BzpMtfMain(BzpMtfInfo *mtf);
// 释放资源
void BzpMtfFinish(BzpMtfInfo *mtf);
// 重置
void BzpMtfReSet(BzpMtfInfo *mtf);
#ifdef __cplusplus
}
#endif

#endif
