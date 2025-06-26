/**
 * @file cmptlz_enc_dp.c
 * @copyright Copyright (c) Huawei Technologies Co., Ltd. 2024-2024. All rights reserved.
 * @brief CMPTLZ 最优匹配dp实现
 * @author Anonym
 * @date 2024-01-09
 */

#include "cmptlz_enc_inner.h"
#include "cmptlz_enc_price.h"

static ALWAYS_INLINE void CmptlzDpInitShortRep(CmptLzEncCtx *encCtx,
    uint32_t repMatchPrice, const uint32_t posState)
{
    const uint32_t shortRepPrice = repMatchPrice +
        CmptPriceShortRep(encCtx, encCtx->state, posState);
    if (shortRepPrice < encCtx->opts[1].price) {
        encCtx->opts[1].price = shortRepPrice;
        encCtx->opts[1].backPrev = 0;
    }
}

static ALWAYS_INLINE void CmptlzDpInitLongRep(CmptLzEncCtx *encCtx,
    uint32_t *repLens, const uint32_t repMatchPrice, const uint32_t posState)
{
    uint32_t i;
    for (i = 0; i < CMPTLZ_NUM_REPS; i++) {
        uint32_t repLen = repLens[i]; // 取出most recent四种repLen
        if (repLen < CMPTLZ_MATCH_LEN_MIN) {
            continue;
        }
        const uint32_t price = repMatchPrice +
            CmptPriceLongRep(encCtx, i, encCtx->state, posState); // 11 + 其他前缀的代价
        do {
            const uint32_t curAndLenPrice = price +
                CmptPriceLen(&encCtx->repLenEncoder, repLen, posState);
            if (curAndLenPrice < encCtx->opts[repLen].price) {
                encCtx->opts[repLen].price = curAndLenPrice;
                encCtx->opts[repLen].posPrev = 0;
                encCtx->opts[repLen].backPrev = i;  // backPrev == 最小代价的repi
            }
            repLen--;
        } while (repLen >= CMPTLZ_MATCH_LEN_MIN);
    }
}

static ALWAYS_INLINE void CmptlzDpInitMatch(CmptLzEncCtx *encCtx, uint32_t matchesCount,
    uint32_t normalMatchPrice, uint32_t posState, uint32_t len)
{
    uint32_t i = 0;
    while (len > encCtx->matches[i].len) {
        i++; // i也初始化为len
    }
    for (; ; len++) { // match的遍历范围[len，encCtx->matches[i].len] 填好这段表【len,len_main】
        const uint32_t dist = encCtx->matches[i].dist;
        const uint32_t curAndLenPrice = normalMatchPrice +
            CmptPriceDistWithLen(encCtx, dist, len, posState);
        if (curAndLenPrice < encCtx->opts[len].price) {
            encCtx->opts[len].price = curAndLenPrice;
            encCtx->opts[len].posPrev = 0;
            encCtx->opts[len].backPrev = dist + CMPTLZ_NUM_REPS; // 已经减1
        }
        if (len == encCtx->matches[i].len) {
            if (++i == matchesCount) { // 每跑一轮就i++,控制循环次数，直到longest_match就break
                break;
            }
        }
    }
}

/*
 * 做一些初始化工作包括：
 * 1.直接处理超长match_len(>niceLen)的情况
 * 2.直接处理buf_aviliable太少的情况
 * 3.直接处理LIT包情况
 * 4.直接处理lenEnd==1的情况，得到LIT与shortRep最佳解,放在opt[1]
 * 5.得到lenEnd为5个值中的最大值，供下一步mainProcess来做进一步dp
 * 6.还做了opt[0 ~ lenEnd]的初始化,从5个包中选代价最小的
 */
static uint32_t CmptlzDpInit(CmptLzEncCtx *encCtx, CmptMfCtx *mf, uint32_t position)
{
    const uint32_t niceLen = mf->niceLen;
    uint32_t lenMain;
    uint32_t matchesCount = 0;

    if (mf->readAhead == 0) {
        lenMain = CmptlzMatchFinder(mf, &matchesCount, encCtx->matches);
    } else {
        lenMain = encCtx->longestMatchLen;
        matchesCount = encCtx->matchesCount;
    }

    const uint8_t *const buf = CmptMfGetPtr(mf) - 1;
    const uint32_t bufAvail = CMPTLZ_FIND_MIN(CmptMfAvail(mf) + 1, CMPT_MF_LONGEST_MATCH);
    /* bufAvail不够,只有shortrep或lit */
    if (bufAvail < CMPTLZ_MATCH_LEN_MIN) {
        encCtx->backRes = CMPTLZ_UINT32_MAX;
        encCtx->lenRes = 1;
        return CMPTLZ_UINT32_MAX;
    }

    uint32_t repLens[CMPTLZ_NUM_REPS];
    uint32_t repMaxIndex = 0;

    /* try倒回去地址能有多长匹配 */
    uint32_t i;
    for (i = 0; i < CMPTLZ_NUM_REPS; i++) {
        const uint8_t *const bufBack = buf - encCtx->reps[i] - 1;

        if (NOT_EQUAL_2_BYTES(buf, bufBack)) {
            repLens[i] = 0;
            continue;
        } // speed
        repLens[i] = CmptMemCmpLenSafe(buf, bufBack, CMPTLZ_MATCH_LEN_MIN, bufAvail);
        if (repLens[i] > repLens[repMaxIndex]) {
            repMaxIndex = i;
        }
    }

    /* 下面两种处理一下大于niceLen的情况，也快速return */
    if (repLens[repMaxIndex] >= niceLen) {
        encCtx->backRes = repMaxIndex;
        encCtx->lenRes = repLens[repMaxIndex];
        CmptlzMatchSkiper(mf, repLens[repMaxIndex] - 1);
        return CMPTLZ_UINT32_MAX; // 一定是先try rep 后try match
    }

    /* 处理一下大于niceLen的情况，也快速return */
    if (lenMain >= niceLen) {
        encCtx->backRes = encCtx->matches[matchesCount - 1].dist + CMPTLZ_NUM_REPS; // 这里也设计为backres -= 1
        encCtx->lenRes = lenMain;
        CmptlzMatchSkiper(mf, lenMain - 1);
        return CMPTLZ_UINT32_MAX; // init里面仅表示快速返回
    }

    // try 顺序先:纯LIT,可shortrep可lit,二选一
    const uint8_t currentByte = *buf;
    const uint8_t matchByte = *(buf - encCtx->reps[0] - 1);
    const uint32_t lenEnd = CMPTLZ_FIND_MAX(lenMain, repLens[repMaxIndex]);
    if ((lenEnd < CMPTLZ_MATCH_LEN_MIN) && (currentByte != matchByte)) {
        encCtx->backRes = CMPTLZ_UINT32_MAX;
        encCtx->lenRes = 1;
        return CMPTLZ_UINT32_MAX;
    }

    // opts[0]里面保存当前包状态和最近四次匹配距离
    encCtx->opts[0].state = encCtx->state;

    const uint32_t posState = position & encCtx->posMask;

    // 计算压成literal的代价 == 编0 + 编literal代价

    encCtx->litMarcov.pos = position;
    encCtx->litMarcov.prevByte = *(buf - 1);
    bool isLiteralState = (encCtx->state < 7);
    bool isMatchMode = !isLiteralState;

    encCtx->opts[1].price = CmptPriceBit0(encCtx, encCtx->isMatch[encCtx->state][posState]) +
        CmptPriceLiteral(encCtx, isMatchMode, matchByte, currentByte);
    encCtx->opts[1].backPrev = CMPTLZ_UINT32_MAX;
    // 下面计算压成rep的代价，分为shortRep和longRep0123
    const uint32_t matchPrice = CmptPriceBit1(encCtx, encCtx->isMatch[encCtx->state][posState]);
    const uint32_t repMatchPrice = matchPrice +
        CmptPriceBit1(encCtx, encCtx->isRep[encCtx->state]);

    // 计算压成shortRep的代价,跟LIT去比一下，留下代价最小的保存在opts[1]里面
    if (matchByte == currentByte) {
        CmptlzDpInitShortRep(encCtx, repMatchPrice, posState);
    }
    // 先在这，处理一下仅有一字节的情况，那就说明既可以压lit也可以压shortRep。
    if (lenEnd < CMPTLZ_MATCH_LEN_MIN) {
        encCtx->backRes = encCtx->opts[1].backPrev;
        encCtx->lenRes = 1;
        return CMPTLZ_UINT32_MAX;
    }

    encCtx->opts[1].posPrev = 0;
    for (i = 0; i < CMPTLZ_NUM_REPS; i++) {
        encCtx->opts[0].backs[i] = encCtx->reps[i];
    }

    /* init_infinity */
    uint32_t len = lenEnd;
    do {
        encCtx->opts[len].price = CMPT_INFINITY_PRICE;
        len--;
    } while (len >= CMPTLZ_MATCH_LEN_MIN);

    // opts[x]上存放当前长度为x的longRep[i]的代价，x从2累加到repLen[i]，4组longRep都可能执行这个操作,所以最终存放的是4组中代价最小的。

    CmptlzDpInitLongRep(encCtx, repLens, repMatchPrice, posState);

    /* 下面计算normalmatch代价 */
    const uint32_t normalMatchPrice = matchPrice
            + CmptPriceBit0(encCtx, encCtx->isRep[encCtx->state]); // 前缀 1 + 0 代价
    len = (repLens[0] > CMPTLZ_MATCH_LEN_MIN) ? repLens[0] + 1 : CMPTLZ_MATCH_LEN_MIN;

    if (len <= lenMain) {
        CmptlzDpInitMatch(encCtx, matchesCount, normalMatchPrice, posState, len);
    }
    return lenEnd;
}

/*
 * 做一些预处理工作包括：
 * 1.拿到当前位置cur的state
 * 2.因为本轮出现新的长len包，所以每个cur的rep[]都会进行更新
 */
static ALWAYS_INLINE void CmptlzDpPre(CmptLzEncCtx *encCtx, uint32_t *mainReps, const uint32_t cur)
{
    uint32_t posPointer = encCtx->opts[cur].posPrev;
    CmptlzState state = encCtx->opts[posPointer].state;

    /* 下面继续往下，从pos_mem更新到cur */
    if (posPointer == cur - 1) { // 判断一下pos_mem到cur（也即本包）是shortrep还是LIT
        if (encCtx->opts[cur].backPrev == 0) {
            CMPT_STATE_UPDATE_WHEN_SHORTREP(state);
        }
        else {
            CMPT_STATE_UPDATE_WHEN_LIT(state);
        }
    } else { // 判断一下pos_mem到cur（也即本包）长度大于1的情况, 也就是说pos_mem到cur（本包）出现了新的rep包或者match包
        uint32_t backPointer;
        backPointer = encCtx->opts[cur].backPrev;

        if (backPointer < CMPTLZ_NUM_REPS) {
            CMPT_STATE_UPDATE_WHEN_LONGREP(state);
        } else {
            CMPT_STATE_UPDATE_WHEN_MATCH(state);
        }
        /* 因为在init过程中已经发现了一个长len，本轮出现了新的长len包，先更新DP中的mainreps[0 ~ 3] */
        /* 大结构体的rep[4]只会被RC来改变 */
        uint32_t i;
        if (backPointer < CMPTLZ_NUM_REPS) { // rep包直接去back[]取
            mainReps[0] = encCtx->opts[posPointer].backs[backPointer];
            
            for (i = 1; i <= backPointer; i++) {
                mainReps[i] = encCtx->opts[posPointer].backs[i - 1];
            }
            for (; i < CMPTLZ_NUM_REPS; i++) {
                mainReps[i] = encCtx->opts[posPointer].backs[i];
            }
        } else {
            mainReps[0] = backPointer - CMPTLZ_NUM_REPS;
            for (i = 1; i < CMPTLZ_NUM_REPS; i++) {
                mainReps[i] = encCtx->opts[posPointer].backs[i - 1];
            }
        }
    }
    /* 从p->opts[cur].posPrev 位置的state，更新到p->opts[cur].state */
    encCtx->opts[cur].state = state;
    /* 把cur的东西都更新一下 */
    uint32_t i;
    for (i = 0; i < CMPTLZ_NUM_REPS; i++) {
        encCtx->opts[cur].backs[i] = mainReps[i];
    }
}

static ALWAYS_INLINE void CmptlzDpTryCurAndLit(CmptLzEncCtx *encCtx, const uint32_t curPrice,
    CmptlzState curState, const uint32_t posState, const uint32_t cur,
    const uint8_t latestMatchByte, const uint8_t curByte)
{
    bool isLiteralState = (curState < 7);
    bool isMatchMode = !isLiteralState;
    const uint32_t curAndLitPrice = curPrice + CmptPriceBit0(encCtx, encCtx->isMatch[curState][posState]) +
        CmptPriceLiteral(encCtx, isMatchMode, latestMatchByte, curByte);
    if (curAndLitPrice < encCtx->opts[cur + 1].price) {
        encCtx->opts[cur + 1].price = curAndLitPrice;
        encCtx->opts[cur + 1].posPrev = cur;
        encCtx->opts[cur + 1].backPrev = CMPTLZ_UINT32_MAX;
    }
}

static ALWAYS_INLINE void CmptlzDpTryCurAndShort(CmptLzEncCtx *encCtx, const uint32_t repMatchPrice,
    const uint32_t cur, CmptlzState curState, const uint32_t posState)
{
    const uint32_t shortRepPrice = repMatchPrice +
                                CmptPriceShortRep(encCtx, curState, posState);
    if (shortRepPrice < encCtx->opts[cur + 1].price) {
        encCtx->opts[cur + 1].price = shortRepPrice;
        encCtx->opts[cur + 1].posPrev = cur;
        encCtx->opts[cur + 1].backPrev = 0;
    }
}
static ALWAYS_INLINE void CmptlzDpTryCurAndLong(CmptLzEncCtx *encCtx, const uint32_t prefixPrice,
    const uint32_t cur, uint32_t mainRepIndex, uint32_t lenEqual, const uint32_t posState)
{
    do { // longRep 即 prefix + len
        const uint32_t curLongRepPrice = prefixPrice + CmptPriceLen(&encCtx->repLenEncoder, lenEqual, posState);

        if (curLongRepPrice < encCtx->opts[cur + lenEqual].price) {
            encCtx->opts[cur + lenEqual].price = curLongRepPrice;
            encCtx->opts[cur + lenEqual].posPrev = cur;
            encCtx->opts[cur + lenEqual].backPrev = mainRepIndex;
        }
    } while (--lenEqual >= CMPTLZ_MATCH_LEN_MIN);
}
static ALWAYS_INLINE void CmptlzDpTryCurAndMatch(CmptLzEncCtx *encCtx, uint32_t startLen,
    uint32_t matchCount, const uint32_t normalmatch_prefixPrice, const uint32_t cur, const uint32_t posState)
{
    uint32_t i = 0;
    while (startLen > encCtx->matches[i].len) {
        i++;
    }
    uint32_t lenTest;
    for (lenTest = startLen; ; lenTest++) { // Try NormalMatch
        const uint32_t curBack = encCtx->matches[i].dist;
        uint32_t cur_normalmatchPrice = normalmatch_prefixPrice +
                                        CmptPriceDistWithLen(encCtx, curBack, lenTest, posState);
        if (cur_normalmatchPrice < encCtx->opts[cur + lenTest].price) {
            encCtx->opts[cur + lenTest].price = cur_normalmatchPrice;
            encCtx->opts[cur + lenTest].posPrev = cur;
            encCtx->opts[cur + lenTest].backPrev = curBack + CMPTLZ_NUM_REPS;
        }
        if (lenTest == encCtx->matches[i].len) {
            if (++i == matchCount) {
                break;
            }
        }
    }
}

/*
 * 从cur到lenEnd做主要尝试过程：
 * 1.cur + lit
 * 2.cur + shortRep，两者取最小，更新至cur + 1
 * 3.cur + longRep0123，从[2, lenEqual]尝试新的longRep,更新至 cur + len_x
 * 4.cur + normalmatch 从[startLen, lenTest]尝试新的normalmatch,更新至 cur + len_x
 * 5.同时在cur往前走的过程中，如果发现了新的长匹配，比原来的lenEnd还长，那就更新新的lenEnd。
 */
static ALWAYS_INLINE uint32_t CmptlzDpProcess(CmptLzEncCtx *encCtx, CmptMfCtx *mf, uint32_t *mainReps,
    uint32_t lenEnd, uint32_t position, const uint32_t cur)
{
    /* 前面是为了的到cur的rep和state */
    /* 准备工作结束！正式开始DP */
    CmptlzState curState = encCtx->opts[cur].state;
    const uint32_t bufAvailFull = CMPTLZ_FIND_MIN(CmptMfAvail(mf) + 1, CMPT_DP_OPTMAX - 1 - cur);
    const uint8_t *buf = CmptMfGetPtr(mf) - 1;
    const uint32_t niceLen = mf->niceLen;
    const uint32_t curPrice = encCtx->opts[cur].price;
    const uint8_t curByte = *buf;
    const uint8_t latestMatchByte = *(buf - mainReps[0] - 1);
    const uint32_t posState = position & encCtx->posMask;
    /* 当前位置编cur + literal的代价称为cur_and_lit */
    encCtx->litMarcov.pos = position;
    encCtx->litMarcov.prevByte = *(buf - 1);

    CmptlzDpTryCurAndLit(encCtx, curPrice, curState, posState, cur, latestMatchByte, curByte);

    /* 尝试cur + shortRep */
    const uint32_t matchPrice = curPrice
            + CmptPriceBit1(encCtx, encCtx->isMatch[curState][posState]);
    const uint32_t repMatchPrice = matchPrice
            + CmptPriceBit1(encCtx, encCtx->isRep[curState]);

    if (curByte == latestMatchByte && !(encCtx->opts[cur + 1].posPrev < cur && encCtx->opts[cur + 1].backPrev == 0)) {
        // 因为backPrev代表shortrep或者longrep0，如果cur + 1位置的上包结束位置/本包开始位置不为cur，说明就不是shortrep
        CmptlzDpTryCurAndShort(encCtx, repMatchPrice, cur, curState, posState);
    }

    if (bufAvailFull < CMPTLZ_MATCH_LEN_MIN) {
        return lenEnd;
    }
    const uint32_t bufAvail = CMPTLZ_FIND_MIN(bufAvailFull, niceLen);
    uint32_t startLen = CMPTLZ_MATCH_LEN_MIN;

    /* 尝试longrep0123 */
    uint32_t mainRepIndex;
    for (mainRepIndex = 0; mainRepIndex < CMPTLZ_NUM_REPS; mainRepIndex++) {
        const uint8_t *const bufRepBack = buf - mainReps[mainRepIndex] - 1;

        if (NOT_EQUAL_2_BYTES(buf, bufRepBack)) { // len < 2 直接跑路
            continue;
        }

        uint32_t lenEqual;
        lenEqual = CmptMemCmpLenSafe(buf, bufRepBack, CMPTLZ_MATCH_LEN_MIN, bufAvail);
        while (lenEnd < cur + lenEqual) {
            // 这里还在尝试新的最长匹配，lenEnd第一次可能更新的地方
            lenEnd++;
            encCtx->opts[lenEnd].price = CMPT_INFINITY_PRICE;
        }

        const uint32_t lenEqualMem = lenEqual;

        const uint32_t prefixPrice = repMatchPrice +
            CmptPriceLongRep(encCtx, mainRepIndex, curState, posState);

        CmptlzDpTryCurAndLong(encCtx, prefixPrice, cur, mainRepIndex, lenEqual, posState);

        lenEqual = lenEqualMem;

        if (mainRepIndex == 0) {
            startLen = lenEqual + 1;
        }
    }

    /* 尝试normalmatch */

    uint32_t newLongestLen = encCtx->longestMatchLen; // 每一个cur对应的新的最长匹配
    uint32_t matchCount = encCtx->matchesCount;

    if (newLongestLen > bufAvail) {
        newLongestLen = bufAvail;
        matchCount = 0;
        while (newLongestLen > encCtx->matches[matchCount].len) {
            ++matchCount;
        }
        encCtx->matches[matchCount++].len = newLongestLen; // 给p->matches[matchCount_end_index + 1].len 赋值为最新最长长度
    }

    if (newLongestLen >= startLen) {
        const uint32_t normalmatch_prefixPrice = matchPrice +
                                                 CmptPriceBit0(encCtx, encCtx->isRep[curState]);

        while (lenEnd < cur + newLongestLen) {
            lenEnd++;
            encCtx->opts[lenEnd].price = CMPT_INFINITY_PRICE;
        }
        CmptlzDpTryCurAndMatch(encCtx, startLen, matchCount, normalmatch_prefixPrice, cur, posState);
    }
    return lenEnd;
}

/*
 * 做收尾工作包括：
 * 1.从lenEnd往前回溯到0，找到lenEnd对应的最小代价包，位置跳回，改变链表的指向。
 * 2.最终opts[0]存了第一个包信息：lenRes与backRes。
 * 3.最终optEndIndex存末包结束位置, optsCurIndex存0包的结束位置。
 * 4.传给大结构体。
 */
static ALWAYS_INLINE void CmptlzDpReverse(CmptLzEncCtx *encCtx, uint32_t cur)
{
    encCtx->optEndIndex = cur;
    uint32_t posTmp = encCtx->opts[cur].posPrev;
    uint32_t backTmp = encCtx->opts[cur].backPrev;
    uint32_t posPrev, backCurPacket;
    do {
        posPrev = posTmp;
        backCurPacket = backTmp;

        backTmp = encCtx->opts[posPrev].backPrev;
        posTmp = encCtx->opts[posPrev].posPrev;

        encCtx->opts[posPrev].backPrev = backCurPacket; // 现在往后指向本包
        encCtx->opts[posPrev].posPrev = cur;
        cur = posPrev; // cur向前更新
    } while (cur != 0);

    encCtx->lenRes  = encCtx->opts[0].posPrev;
    encCtx->backRes = encCtx->opts[0].backPrev;
    encCtx->optsCurIndex = encCtx->opts[0].posPrev;
}

/*
 * Cmptlz最优匹配函数:
 * 入参：未压缩字符串流的位置position，和快速匹配模块mf结构体，和大结构体p
 * 出参：大结构体p的 lenRes：表示下一个要进行区间编码的包应该有多长
 *                  backRes:表示下一个要进行区间编码的包是什么类型
 *                  optsCurIndex:快速逃逸button
 */
void CmptlzDp(CmptLzEncCtx *encCtx, CmptMfCtx *mf, uint32_t position)
{
    uint32_t curIndex = encCtx->optsCurIndex;
    uint32_t endIndex = encCtx->optEndIndex;
    /* 如果一轮就出一个包，那就进不来这个分支 */
    /* 如果一轮出了N个包，就会进入这个分支N - 1次 */
    if (endIndex != curIndex) { // 先处理前一轮剩余包
        encCtx->lenRes = encCtx->opts[curIndex].posPrev - curIndex;
        encCtx->backRes = encCtx->opts[curIndex].backPrev; // 返回数据包类型
        encCtx->optsCurIndex = encCtx->opts[curIndex].posPrev;
        return;
    }
    uint32_t lenEnd = CmptlzDpInit(encCtx, mf, position);
    // CmptlzDpInit得到【0，lenEnd】代价上界
    if (lenEnd == CMPTLZ_UINT32_MAX) {
        return;
    }
    uint32_t mainReps[CMPTLZ_NUM_REPS];
    memcpy_s(mainReps, sizeof(mainReps), encCtx->reps, sizeof(encCtx->reps));

    uint32_t cur;
    for (cur = 1; cur < lenEnd; cur++) {
        encCtx->longestMatchLen = CmptlzMatchFinder(mf, &encCtx->matchesCount, encCtx->matches);
        if (encCtx->longestMatchLen >= mf->niceLen) {
            break;
        }
        CmptlzDpPre(encCtx, mainReps, cur);
        lenEnd = CmptlzDpProcess(encCtx, mf, mainReps, lenEnd, position + cur, cur);
    }
    CmptlzDpReverse(encCtx, cur);
    return;
}