/**
 * @file bzp_mtf_encode.c
 * @copyright Copyright (c) Huawei Technologies Co., Ltd. 2023-2023. All rights reserved.
 * @brief 提供了bzp压缩中mtf算法的初始化、变换、资源释放等接口
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
#include "bzp_mtf_encode.h"

#ifdef __cplusplus
extern "C" {
#endif
// 初始化
BzpMtfInfo *BzpMtfInit(int32_t blockSize)
{
    if (BZP_INVALID_BLOCK_SIZE(blockSize)) {
        return NULL;
    }
    BzpMtfInfo *mtf = (BzpMtfInfo *)malloc(sizeof(BzpMtfInfo));
    if (mtf == NULL) {
        return NULL;
    }
    mtf->mtfV = NULL;
    mtf->mtfV = (int32_t *)malloc(blockSize * BZP_BASE_BLOCK_SIZE * sizeof(int32_t));
    if (mtf->mtfV == NULL) {
        free(mtf);
        mtf = NULL;
        return NULL;
    }
    // mtfFreq在main里会初始化
    mtf->nUse = 0;
    mtf->nMtf = 0;
    mtf->block = NULL;
    mtf->map = NULL;
    mtf->inUse = NULL;
    return mtf;
}
// 重置
void BzpMtfReSet(BzpMtfInfo *mtf)
{
    // mtfFreq在main里会初始化
    mtf->nUse = 0;
    mtf->nMtf = 0;
    mtf->block = NULL;
    mtf->map = NULL;
    mtf->inUse = NULL;
}

// 统计使用字符，构造字典列表
void BzpMapInputChar(BzpMtfInfo *mtf, uint8_t *list, int32_t lenList)
{
    if (BZP_ASCII_SIZE > lenList) {
        return;
    }
    for (int32_t i = 0; i < BZP_ASCII_SIZE; i++) {
        if (mtf->inUse[i]) {
            list[mtf->nUse] = (uint8_t)i;
            mtf->nUse++;
        }
    }
}
// 将一个正整数进行编码
void BzpNumEncode(BzpMtfInfo *mtf, int32_t num)
{
    num <<= 1; // 特殊处理第一次
    // 每次编码完成后，需要将低位删除，第一次则不需要
    do {
        num >>= 1;
        num--;
        if (num & 1) {
            mtf->mtfV[mtf->nMtf++] = BZP_MTF_ENCODE1;
            mtf->mtfFreq[BZP_MTF_ENCODE1]++;
        } else {
            mtf->mtfV[mtf->nMtf++] = BZP_MTF_ENCODE0;
            mtf->mtfFreq[BZP_MTF_ENCODE0]++;
        }
    } while (num >= BZP_MTF_ENCODE_BASE);
}
// mtf变换主流程
void BzpMtfMain(BzpMtfInfo *mtf)
{
    uint8_t list[BZP_MAX_ALPHA_SIZE]; // 共识字典
    int32_t EOB;
    int32_t num = 0; // 某个字符出现的次数
    BzpMapInputChar(mtf, list, BZP_MAX_ALPHA_SIZE);
    EOB = mtf->nUse + 1;
    for (int32_t i = 0; i <= EOB; i++) {
        mtf->mtfFreq[i] = 0;
    }
    for (int32_t i = 0; i < mtf->nBlock; i++) {
        int32_t pos = mtf->map[i] - 1;
        if (pos < 0) {
            pos += mtf->nBlock;
        }
        uint8_t ch = mtf->block[pos];
        if (ch == list[0]) {
            num++;
        } else {
            if (num > 0) {
                BzpNumEncode(mtf, num);
                num = 0;
            }
            int32_t pos_ = 1;
            while (ch != list[pos_] && pos_ < mtf->nUse) {
                pos_++;
            }

            for (int32_t j = pos_; j > 0; j--) {
                list[j] = list[j - 1];
            }
            list[0] = ch;

            mtf->mtfV[mtf->nMtf] = pos_ + 1;
            mtf->mtfFreq[pos_ + 1]++;
            mtf->nMtf++;
        }
    }
    if (num > 0) {
        BzpNumEncode(mtf, num);
    }

    // 最后加入终止符号
    mtf->mtfV[mtf->nMtf] = EOB;
    mtf->mtfFreq[EOB]++;
    mtf->nMtf++;
}
// 释放资源
void BzpMtfFinish(BzpMtfInfo *mtf)
{
    if (mtf != NULL) {
        if (mtf->mtfV != NULL) {
            free(mtf->mtfV);
            mtf->mtfV = NULL;
        }
        free(mtf);
        mtf = NULL;
    }
}

#ifdef __cplusplus
}
#endif
