/**
 * @file cmptlz_dec_init.c
 * @copyright Copyright (c) Huawei Technologies Co., Ltd. 2022-2023. All rights reserved.
 * @brief CMPTLZ 解压初始化文件
 * @author Anonym
 * @date 2023-07-06
 * @version v0.1.0
 ********************************************************************************************
 * @par History
 * <table>
 * <tr><th>Date        <th>Version   <th>Author      <th>Description
 * <tr><td>2023-07-06  <td>0.1.0     <td>   <td>Init version
 * </table>
 ********************************************************************************************
 */

#include "cmptlz_dec_inner.h"

#define CMPTLZ_LIT_CTX_MAX 9
#define CMPTLZ_POS_STATE_MAX 5
#define CMPTLZ_LIT_POS_MAX 5

#define CMPTLZ_BIG_DICT_LG_SIZE 30
#define CMPTLZ_MID_DICT_LG_SIZE 22
#define CMPTLZ_SMALL_DICT_LG_SIZE 20

static ALWAYS_INLINE int CmptLzPropsDecode(const unsigned char *protData, unsigned protSize, CmptLzDecProt *decProt)
{
    uint32_t dictSize;

    if (protSize < CMPTLZ_PROPS_SIZE) {
        return CMPT_ERROR_UNSUPPORTED;
    } else {
        // protData的1/2/3/4字节分别按照左移0位8位/16位/24位的规则与运算得到字典大小
        dictSize = protData[1] | ((uint32_t)protData[2] << 8) |
            ((uint32_t)protData[3] << 16) | ((uint32_t)protData[4] << 24);  // 3/4字节分别按照左移16位/24位
    }

    if (dictSize < CMPTLZ_DICT_MIN_LEN) {
        dictSize = CMPTLZ_DICT_MIN_LEN;
    }
    decProt->dicSize = dictSize;

    unsigned char firstData = protData[0];
    if (firstData >= (CMPTLZ_LIT_CTX_MAX * CMPTLZ_POS_STATE_MAX * CMPTLZ_LIT_POS_MAX)) {
        return CMPT_ERROR_UNSUPPORTED;
    }

    decProt->litCtx = (unsigned char)(firstData % CMPTLZ_LIT_CTX_MAX);
    firstData /= CMPTLZ_LIT_CTX_MAX;
    decProt->posBits = (unsigned char)(firstData / CMPTLZ_POS_STATE_MAX);
    decProt->litPos = (unsigned char)(firstData % CMPTLZ_LIT_POS_MAX);

    return CMPT_OK;
}

void CmptLzDecInit(CmptLzDecCtx *decCtx)
{
    decCtx->dictPos = 0;
    decCtx->tempBufSize = 0;
    decCtx->processedPos = 0;
    decCtx->checkDicSize = 0;
    decCtx->remainLen = CMPTLZ_MATCH_MAX_LEN + 2; // remainLen在未解压的初始状态作为标志位，最大匹配距离+2标志此时解压上下文未初始化
}

static ALWAYS_INLINE void *CmptLzDecMemAlloc(CmptLzMemHook *memHook, int32_t memHandle, size_t allocSize)
{
    return memHook->CmptLzAlloc(memHandle, allocSize);
}

static ALWAYS_INLINE void CmptLzDecMemFree(CmptLzMemHook *memHook, int32_t memHandle, void *freeAddress)
{
    memHook->CmptLzFree(memHandle, freeAddress);
}

static ALWAYS_INLINE void CmptLzDecFreeProbs(CmptLzDecCtx *decCtx, CmptLzMemHook *memHook)
{
    if (decCtx->probs != NULL) {
        CmptLzDecMemFree(memHook, CMPTLZ_PROB_HANDLE, decCtx->probs);
        decCtx->probs = NULL;
    }
}

static ALWAYS_INLINE void CmptLzFreeDict(CmptLzDecCtx *decCtx, CmptLzMemHook *memHook)
{
    if (decCtx->dict != NULL) {
        CmptLzDecMemFree(memHook, CMPTLZ_DICT_HANDLE, decCtx->dict);
        decCtx->dict = NULL;
    }
}

static ALWAYS_INLINE int CmptLzDecAllocateProbs(CmptLzDecCtx *decCtx, CmptLzDecProt *decProt, CmptLzMemHook *memHook)
{
    uint32_t numProbs = CmptLzGetNumProbs(decProt);

    if (decCtx->probs == NULL) {
        decCtx->probs = (CmptLzDecProb *)CmptLzDecMemAlloc(
            memHook, CMPTLZ_PROB_HANDLE, numProbs * sizeof(CmptLzDecProb));
    } else {
        if (numProbs != decCtx->numProbs) {
            CmptLzDecFreeProbs(decCtx, memHook);
            decCtx->probs =
                (CmptLzDecProb *)CmptLzDecMemAlloc(memHook, CMPTLZ_PROB_HANDLE, numProbs * sizeof(CmptLzDecProb));
        }
    }

    if (decCtx->probs == NULL) {
        return CMPT_ERROR_MEM;
    }

    decCtx->probsPlus1664 = decCtx->probs + 1664; // 1664 分割概率表
    decCtx->numProbs = numProbs;

    return CMPT_OK;
}

int CmptLzDecAllocate(CmptLzDecCtx *decCtx, const unsigned char *protData, unsigned protSize, CmptLzMemHook *memHook)
{
    int res;
    uint32_t dictMask;
    size_t dictBufSize;
    CmptLzDecProt decProt;

    if ((decCtx == NULL) || (protData == NULL) || (memHook == NULL)) {
        return CMPT_ERROR_UNSUPPORTED;
    }

    res = CmptLzPropsDecode(protData, protSize, &decProt);
    if (res != CMPT_OK) {
        return res;
    }
    res = CmptLzDecAllocateProbs(decCtx, &decProt, memHook);
    if (res != CMPT_OK) {
        return res;
    }

    uint32_t dictSize = decProt.dicSize;
    if (dictSize >= ((uint32_t)1 << CMPTLZ_BIG_DICT_LG_SIZE)) {
        dictMask = ((uint32_t)1 << CMPTLZ_MID_DICT_LG_SIZE) - 1;
    } else if (dictSize >= ((uint32_t)1 << CMPTLZ_MID_DICT_LG_SIZE)) {
        dictMask = ((uint32_t)1 << CMPTLZ_SMALL_DICT_LG_SIZE) - 1;
    } else {
        dictMask = CMPTLZ_DICT_MIN_LEN - 1;
    }

    dictBufSize = ((size_t)dictSize + dictMask) & ~dictMask;
    if (dictBufSize < dictSize) {
        dictBufSize = dictSize;
    }

    if (decCtx->dict == NULL) {
        decCtx->dict = (unsigned char *)CmptLzDecMemAlloc(memHook, CMPTLZ_DICT_HANDLE, dictBufSize);
    } else {
        if (dictBufSize != decCtx->dictBufSize) {
            CmptLzFreeDict(decCtx, memHook);
            decCtx->dict = (unsigned char *)CmptLzDecMemAlloc(memHook, CMPTLZ_DICT_HANDLE, dictBufSize);
        }
    }

    if (decCtx->dict == NULL) {
        CmptLzDecFreeProbs(decCtx, memHook);
        return CMPT_ERROR_MEM;
    }

    decCtx->dictBufSize = dictBufSize;
    decCtx->prop = decProt;

    return CMPT_OK;
}

int CmptLzDecFree(CmptLzDecCtx *decCtx, CmptLzMemHook *memHook)
{
    if ((decCtx == NULL) || (memHook == NULL)) {
        return CMPT_ERROR_UNSUPPORTED;
    }

    CmptLzDecFreeProbs(decCtx, memHook);
    CmptLzFreeDict(decCtx, memHook);

    return CMPT_OK;
}

void CmptLzDecConstruct(CmptLzDecCtx *decCtx)
{
    decCtx->dict = NULL;
    decCtx->probs = NULL;
}

int CmptLzDecode(CmptLzDecIn *pDecIn, CmptLzDecOut *pDecOut, const unsigned char *protData,
    EnCmptLzFinMode finMode, EnCmptLzStatus *finStatus, CmptLzMemHook *memHook)
{
    int res;
    size_t inSize = pDecIn->strInLen;
    CmptLzDecProt decProt;
    CmptLzDecCtx decCtx;
    decCtx.numProbs = 0; // 防止告警，实际上在第一次进入CmptLzDecAllocateProbs时会对该值做初始化。

    if (inSize < CMPTLZ_PROPS_SIZE) { // 至少有5字节作为压缩头部
        return CMPT_ERROR_UNSUPPORTED;
    }

    CmptLzDecConstruct(&decCtx);
    res = CmptLzPropsDecode(protData, CMPTLZ_PROPS_SIZE, &decProt);
    if (res != CMPT_OK) {
        return res;
    }
    res = CmptLzDecAllocateProbs(&decCtx, &decProt, memHook);
    if (res != CMPT_OK) {
        return res;
    }

    decCtx.prop = decProt;
    decCtx.dict = pDecOut->pDestOut;
    decCtx.dictBufSize = pDecOut->destOutLen;
    CmptLzDecInit(&decCtx);

    *finStatus = CMPTLZ_STATUS_NOT_SPECIFIED;
    res = CmptLzDecDecodeToDic(&decCtx, pDecOut->destOutLen, pDecIn->pSrcIn, &inSize, finMode, finStatus);
    pDecIn->strInCostLen = inSize;
    pDecOut->destOutFillLen = decCtx.dictPos;
    CmptLzDecFreeProbs(&decCtx, memHook);

    return res;
}