/**
 * @file bzp_bwt_encode.c
 * @copyright Copyright (c) Huawei Technologies Co., Ltd. 2023-2023. All rights reserved.
 * @brief 提供了bzp压缩中bwt算法的初始化、变换、资源释放等接口
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

#include "bzp_bwt_encode.h"

#ifdef __cplusplus
extern "C" {
#endif
// 初始化

BzpBwtInfo *BzpBlockSortInit(int32_t blockSize)
{
    if (BZP_INVALID_BLOCK_SIZE(blockSize)) {
        return NULL;
    }
    BzpBwtInfo *bwt = (BzpBwtInfo *)malloc(sizeof(BzpBwtInfo));
    if (bwt == NULL) {
        return NULL;
    }
    // 将整个结构体空间全部置0
    (void)memset_s(bwt, sizeof(BzpBwtInfo), 0, sizeof(BzpBwtInfo));

    int32_t spaceSize = blockSize * BZP_BASE_BLOCK_SIZE;
    bwt->nBlockMax = spaceSize - BZP_BLOCK_RESERVED_SPACE_SIZE;
    bwt->block = (uint8_t *)malloc(spaceSize * sizeof(uint8_t));
    bwt->sortBlock = (int32_t *)malloc(spaceSize * sizeof(int32_t));
    bwt->idx = (int32_t *)malloc(spaceSize * sizeof(int32_t));
    bwt->isStartPos = (int32_t *)malloc(spaceSize * sizeof(int32_t));
    if (bwt->block == NULL || bwt->sortBlock == NULL || bwt->idx == NULL || bwt->isStartPos == NULL) {
        BzpBwtFinish(bwt);
        return NULL;
    }

    (void)memset_s(bwt->isStartPos, spaceSize * sizeof(int32_t), 0, spaceSize * sizeof(int32_t));
    bwt->blockCRC = BZP_INIT_BLOCK_CRC;
    return bwt;
}
// 希尔排序
void BzpShellSort(int32_t *sortBlock, int32_t *idx, int32_t l, int32_t r)
{
    // 定义增量
    int32_t increments[] = {BZP_SHELL_SORT_INCREMENT1, BZP_SHELL_SORT_INCREMENT0};
    int32_t i, j;
    if (l >= r) {
        return;
    }

    for (int32_t id = 0; id < BZP_SHELL_SORT_INCREMENT_NUMS; id++) {
        int32_t H = increments[id];
        if (r - l + 1 <= H) {
            continue;
        }
        for (i = l + H; i <= r; i++) {
            int32_t tmpIdx = sortBlock[i];
            int32_t tmpVal = idx[tmpIdx];
            for (j = i - H; j >= l && idx[sortBlock[j]] > tmpVal; j -= H) {
                sortBlock[j + H] = sortBlock[j];
            }
            sortBlock[j + H] = tmpIdx;
        }
    }
}
// 交换2个元素的位置
void BzpSwap2Elem(int32_t *sortBlock, int32_t lPos, int32_t rPos)
{
    int32_t value = sortBlock[lPos];
    sortBlock[lPos] = sortBlock[rPos];
    sortBlock[rPos] = value;
}
// 交换3个元素的位置
void BzpSwap3Elem(int32_t *sortBlock, int32_t lPos, int32_t ePos, int32_t rPos)
{
    // ePos > v  lPos = v  rPos < v
    int32_t value = sortBlock[lPos];
    sortBlock[lPos] = sortBlock[rPos];
    sortBlock[rPos] = sortBlock[ePos];
    sortBlock[ePos] = value;
}
// 选择快排需要的基准元素
int32_t BzpSelectMidVal(int32_t *sortBlock, int32_t *idx, int32_t l, int32_t r)
{
    int32_t mid = (l + r) >> 1;
    int32_t vl = idx[sortBlock[l]];
    int32_t vmid = idx[sortBlock[mid]];
    int32_t vr = idx[sortBlock[r]];
    if (vl > vr) {
        int32_t tmp = l;
        l = r;
        r = tmp;
        vl = idx[sortBlock[l]];
        vr = idx[sortBlock[r]];
    }
    if (vmid <= vl) {
        return vl;
    } else if (vmid <= vr) {
        return vmid;
    } else {
        return vr;
    }
}

void BzpQSortSingle(int32_t *sortBlock, int32_t *idx, BzpQSortInfo *stack)
{
    int32_t tl = stack->tl, tr = stack->tr;
    int32_t value = BzpSelectMidVal(sortBlock, idx, tl, tr);
    int32_t lPos = tl, rPos = tr, ePos = tl;
    // 结果中[l ,lPos)区间的值是小于value的，[lPos ,ePos)区间的值等于value，(rPos,r]区间的值大于value
    while (ePos <= rPos) {
        if (idx[sortBlock[ePos]] < value) { // [l ,lPos) <
            BzpSwap2Elem(sortBlock, ePos, lPos);
            ePos++;
            lPos++;
        } else if (idx[sortBlock[ePos]] == value) { // [lPos ,ePos) ==
            ePos++;
        } else { // (rPos ,r] >
            while (rPos >= ePos && idx[sortBlock[rPos]] > value) {
                rPos--;
            }
            if (rPos < ePos) {
                break;
            }
            if (idx[sortBlock[rPos]] == value) {
                BzpSwap2Elem(sortBlock, ePos, rPos);
            } else if (lPos == ePos) {
                // 通过上面的步骤确定了ePos上的值是>v，rPos上的值<v。但是没有判断lPos，特殊情况，lPos==ePos
                BzpSwap2Elem(sortBlock, ePos, rPos);
                lPos++;
            } else {
                BzpSwap3Elem(sortBlock, lPos, ePos, rPos); // ePos > v  lPos = v  rPos < v
                lPos++;
            }
            ePos++;
            rPos--;
        }
    }
    // 小区间先出栈，减少栈的最大使用空间
    if (lPos - tl > tr - rPos) {
        stack->stackL[stack->cnt] = tl;
        stack->stackR[stack->cnt] = lPos - 1;
        stack->cnt++;
        stack->stackL[stack->cnt] = rPos + 1;
        stack->stackR[stack->cnt] = tr;
        stack->cnt++;
    } else {
        stack->stackL[stack->cnt] = rPos + 1;
        stack->stackR[stack->cnt] = tr;
        stack->cnt++;
        stack->stackL[stack->cnt] = tl;
        stack->stackR[stack->cnt] = lPos - 1;
        stack->cnt++;
    }
}
// 快排 排序区间[l,r]
void BzpQuickSort(int32_t *sortBlock, int32_t *idx, int32_t l, int32_t r)
{
    BzpQSortInfo stack;
    stack.cnt = 0;
    stack.stackL[stack.cnt] = l;
    stack.stackR[stack.cnt] = r;
    stack.cnt++;
    while (stack.cnt > 0) {
        stack.cnt--;
        int32_t tl = stack.stackL[stack.cnt];
        int32_t tr = stack.stackR[stack.cnt];
        // 对于不合法的区间，直接丢弃
        if (tl >= tr) {
            continue;
        }
        if (tr - tl < BZP_THRESHOLD_SHELL_SORT) {
            BzpShellSort(sortBlock, idx, tl, tr);
            continue;
        }
        stack.tl = tl;
        stack.tr = tr;
        BzpQSortSingle(sortBlock, idx, &stack);
    }
}
// 更新区间标志位
void BzpUpdateflag(BzpBwtInfo *bwt, int32_t l, int32_t r)
{
    int32_t tmpst = -1;
    for (int32_t i = l; i <= r; i++) {
        int32_t tmpnow = bwt->idx[bwt->sortBlock[i]];
        if (tmpst != tmpnow) {
            bwt->isStartPos[i] = 1;
            tmpst = tmpnow;
        }
    }
}
// 基于倍增思想的排序
void BzpBinaryLiftingSort(BzpBwtInfo *bwt)
{
    int32_t ftab[BZP_ASCII_SIZE]; // 存储字符出现次数，用于计数排序
    (void)memset_s(ftab, sizeof(ftab), 0, sizeof(ftab));
    for (int32_t i = 0; i < bwt->nBlock; i++) {
        ftab[bwt->block[i]]++;
    }
    for (int32_t i = 1; i < BZP_ASCII_SIZE; i++) {
        ftab[i] += ftab[i - 1];
    }
    for (int32_t i = 0; i < bwt->nBlock; i++) {
        int32_t ch = bwt->block[i];
        ftab[ch]--;
        bwt->sortBlock[ftab[ch]] = i;
    }
    for (int32_t i = 0; i < BZP_ASCII_SIZE; i++) {
        bwt->isStartPos[ftab[i]] = 1;
    }
    int32_t M = 1, sortflag = true; // 要排序字符串长度 和 本轮是否排序标志
    // 统计一下处理完成的区间，提前跳出,sortflag用于判断这一轮是否经过排序，没有则说明所有区间有序。
    while (M < bwt->nBlock && sortflag == true) {
        int32_t st = 0;
        sortflag = false;
        // 维护第二关键字的大小，
        for (int32_t i = 0; i < bwt->nBlock; i++) {
            if (bwt->isStartPos[i]) {
                st = i;
            }
            int32_t pos = bwt->sortBlock[i] - M;
            if (pos < 0) {
                pos += bwt->nBlock;
            }
            bwt->idx[pos] = st;
        }
        int32_t l = 0, r = 1;
        while (l < bwt->nBlock) {
            while (r < bwt->nBlock && bwt->isStartPos[r] != 1) {
                r++;
            }
            r--; // [l,r]
            if (l < r) {
                sortflag = true;
                BzpQuickSort(bwt->sortBlock, bwt->idx, l, r);
                BzpUpdateflag(bwt, l, r);
            }
            l = r + 1;
            r = l + 1;
        }
        M <<= 1;
    }
}
// 调用排序的函数，之后可以用于替换排序算法
void BzpBlockSortMain(BzpBwtInfo *bwt)
{
    BzpBinaryLiftingSort(bwt);
    // 找到解码的起始位置
    for (int32_t i = 0; i < bwt->nBlock; i++) {
        if (bwt->sortBlock[i] == 0) {
            bwt->oriPtr = i;
            break;
        }
    }
}
// 释放资源
void BzpBwtFinish(BzpBwtInfo *bwt)
{
    if (bwt != NULL) {
        if (bwt->block != NULL) {
            free(bwt->block);
            bwt->block = NULL;
        }
        if (bwt->sortBlock != NULL) {
            free(bwt->sortBlock);
            bwt->sortBlock = NULL;
        }
        if (bwt->idx != NULL) {
            free(bwt->idx);
            bwt->idx = NULL;
        }
        if (bwt->isStartPos != NULL) {
            free(bwt->isStartPos);
            bwt->isStartPos = NULL;
        }

        free(bwt);
        bwt = NULL;
    }
}

#ifdef __cplusplus
}
#endif
