/**
 * @file bzp_bwt_decode.c
 * @copyright Copyright (c) Huawei Technologies Co., Ltd. 2023-2023. All rights reserved.
 * @brief 提供了bzp解压中bwt逆变换的初始化、解码、资源释放等接口
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
#include "bzp_bwt_decode.h"

#ifdef __cplusplus
extern "C" {
#endif
// 初始化
BzpBwtDecodeInfo *BzpBwtDecodeInit(int32_t blockSize)
{
    if (BZP_INVALID_BLOCK_SIZE(blockSize)) {
        return NULL;
    }
    BzpBwtDecodeInfo *bwt = (BzpBwtDecodeInfo *)malloc(sizeof(BzpBwtDecodeInfo));
    if (bwt == NULL) {
        return NULL;
    }
    int32_t spaceSize = BZP_BASE_BLOCK_SIZE * blockSize;
    bwt->block = (uint8_t *)malloc(spaceSize * sizeof(uint8_t));
    bwt->deCode = (uint8_t *)malloc(spaceSize * sizeof(uint8_t));
    bwt->sorted = (int32_t *)malloc(spaceSize * sizeof(int32_t));
    if (bwt->block == NULL || bwt->sorted == NULL || bwt->deCode == NULL) {
        BzpBwtDecodeFinish(bwt);
        return NULL;
    }
    bwt->nBlock = 0;
    bwt->oriPtr = 0;
    return bwt;
}

// bwt解码
void BzpBwtDecode(BzpBwtDecodeInfo *bwt)
{
    // 维护出排序后的数组，用于找到原始字符串中相同的字符位置。
    int32_t ftab[257];
    (void)memset_s(ftab, sizeof(ftab), 0, sizeof(ftab));
    for (int32_t i = 0; i < bwt->nBlock; i++) {
        ftab[bwt->block[i] + 1]++;
    }
    for (int32_t i = 1; i <= BZP_ASCII_SIZE; i++) {
        ftab[i] += ftab[i - 1];
    }
    for (int32_t i = 0; i < bwt->nBlock; i++) {
        uint8_t ch = bwt->block[i];
        bwt->sorted[ftab[ch]] = i;
        ftab[ch]++;
    }
    int32_t cnt = 0;
    int32_t pos = bwt->oriPtr;
    while (cnt < bwt->nBlock) {
        pos = bwt->sorted[pos];
        uint8_t ch = bwt->block[pos];
        bwt->deCode[cnt] = ch;
        cnt++;
    }
}
// 释放资源
void BzpBwtDecodeFinish(BzpBwtDecodeInfo *bwt)
{
    if (bwt != NULL) {
        if (bwt->block != NULL) {
            free(bwt->block);
            bwt->block = NULL;
        }
        if (bwt->deCode != NULL) {
            free(bwt->deCode);
            bwt->deCode = NULL;
        }
        if (bwt->sorted != NULL) {
            free(bwt->sorted);
            bwt->sorted = NULL;
        }
        free(bwt);
        bwt = NULL;
    }
}

#ifdef __cplusplus
}
#endif
