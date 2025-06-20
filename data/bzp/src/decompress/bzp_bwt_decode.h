/**
 * @file bzp_bwt_decode.h
 * @copyright Copyright (c) Huawei Technologies Co., Ltd. 2023-2023. All rights reserved.
 * @brief 提供bwt逆变换的函数及结构体声明
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
#ifndef BZP_BWT_DECODE_H
#define BZP_BWT_DECODE_H

#include "bzp_utils.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef struct {
    int32_t *sorted; // 把mtf解码结果排序
    uint8_t *block;  // mtf解码的结果
    uint8_t *deCode; // 存放bwt解码结果
    int32_t nBlock;  // 一个块中的字符数量
    int32_t oriPtr;  // bwt解码的起始位置
} BzpBwtDecodeInfo;
// 初始化
BzpBwtDecodeInfo *BzpBwtDecodeInit(int32_t blockSize);
// bwt解码
void BzpBwtDecode(BzpBwtDecodeInfo *bwt);
// 释放资源
void BzpBwtDecodeFinish(BzpBwtDecodeInfo *bwt);

#ifdef __cplusplus
}
#endif

#endif
