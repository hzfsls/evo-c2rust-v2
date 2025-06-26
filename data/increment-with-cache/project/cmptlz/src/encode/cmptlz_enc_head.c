/**
 * @file cmptlz_enc_head.c
 * @copyright Copyright (c) Huawei Technologies Co., Ltd. 2024-2024. All rights reserved.
 * @brief CMPTLZ 压缩数据头填写文件
 * @author Anonym
 * @date 2024-01-09
 */

#include "cmptlz_log.h"
#include "cmptlz_enc_inner.h"
#include "cmptlz_utils.h"

#define CMPTLZ_LIT_CTX_MAX 9
#define CMPTLZ_POS_STATE_MAX 5

CMPTLZ_HIDDEN int CmptHeadWrite(CmptLzEncCtx *encCtx, uint8_t *protData, size_t *propsSize)
{
    if (protData == NULL) {
        CMPTLZ_LOG(CMPT_ERROR_DATA, "protData is NULL");
        return CMPT_ENC_ERROR_HEAD;
    }

    if (*propsSize < CMPTLZ_PROPS_SIZE) {
        CMPTLZ_LOG(CMPT_ERROR_DATA, "propsSize need 5 bytes, get %zu", *propsSize);
        return CMPT_ENC_ERROR_HEAD;
    }
    
    CmptlzWriteLE32Bit(protData + 1, encCtx->dicSize);
    protData[0] = (encCtx->posBits * CMPTLZ_POS_STATE_MAX + encCtx->litPos) * CMPTLZ_LIT_CTX_MAX + encCtx->litCtx;
    *propsSize = CMPTLZ_PROPS_SIZE;
    return 0;
}