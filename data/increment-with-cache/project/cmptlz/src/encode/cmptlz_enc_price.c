/**
 * @file cmptlz_enc_price.c
 * @copyright Copyright (c) Huawei Technologies Co., Ltd. 2024-2024. All rights reserved.
 * @brief CMPTLZ 压缩Price 计算代价文件
 * @author Anonym
 * @date 2024-01-09
 */

#include "cmptlz_enc_price.h"

#define CMPT_LIT_SUBCODER(probs, litCtx, lpMask, pos, prevByte) \
    ((probs)[(((pos) & (lpMask)) << (litCtx)) + ((uint32_t)(prevByte) >> (8U - (litCtx)))])
// 给lit概率数组的一列

CMPTLZ_HIDDEN void CmptPriceGenRootTable(CmptLzEncCtx *encCtx)
{
    uint32_t *rootTable = encCtx->priceRootTable;

    const unsigned expandCycleNum = 4;
    const unsigned bitsTotalModeNum = 11;
    const unsigned valueForNormal = 15;
    const unsigned wTopBoarder = 1 << 16;

    for (unsigned i = 0; i < ((uint32_t)1 << bitsTotalModeNum >> CMPT_PRICE_BITS_MOVING_NUM); i++) {
        unsigned w = (i << CMPT_PRICE_BITS_MOVING_NUM) + (1 << (CMPT_PRICE_BITS_MOVING_NUM - 1));
        unsigned dummyNormalizeCnt = 0;
        for (unsigned j = 0; j < expandCycleNum; j++) {
            w = w * w;
            dummyNormalizeCnt <<= 1;
            while (w >= wTopBoarder) {
                w >>= 1;
                dummyNormalizeCnt++;
            }
        }
        rootTable[i] = (uint32_t)((bitsTotalModeNum << expandCycleNum) - valueForNormal - dummyNormalizeCnt);
    }
}

CMPTLZ_HIDDEN void CmptPriceGenDistTable(CmptLzEncCtx *encCtx)
{
    uint32_t distState = 0;
    // 一般地encCtx->distTableSize == 63，代表最大区间号为63
    /* 先填好dist_slot_price表 */
    do {
        uint32_t *const tmpPriceDistSlot = encCtx->priceDistSlotTable[distState]; // 用tmp指向priceDistSlotTable
        // i means distSlot, 先算distSlot固定代价
        for (uint32_t i = 0; i < encCtx->distTableSize; i++) {
            tmpPriceDistSlot[i] = CmptPriceSymbol(encCtx, encCtx->probDistSlot[distState], CMPTLZ_DIST_SLOT_BITS, i);
        }
        // distSlot == 14 ~ 63时，说明dist >= 128，这时先加上额外代价 (这里先不管最后四比特，genalign会管的)
        // 解释下这里为了方便，先填在dist_slot代价表里面了，反正这个后面也用不到，用到的关键是dist代价表，dist_slot代价表只是个中间表
        for (uint32_t i = 14; i < encCtx->distTableSize; i++) { // distSlot == 14
            tmpPriceDistSlot[i] += CmptPriceOneBitDirect((i >> 1) - 1 - CMPTLZ_ALIGN_BITS);
        }

        for (uint32_t i = 0; i < 4; i++) { // 小于4的dist代价等于distSlot代价
            encCtx->priceDistTable[distState][i] = tmpPriceDistSlot[i]; // 填好了priceDistTable[STATE][0 ~ 3]
        }

        distState++;
    } while (distState < CMPTLZ_DIST_STATE_TOTAL);

    /* 然后填dist_price表 */
    // i means dist
    for (uint32_t i = 4; i < 128; i++) { // 填好dist从4到128
        uint32_t distSlot = PosSloter(i);
        uint32_t footerBits = (distSlot >> 1) - 1;
        uint32_t base = (2 | (distSlot & 1)) << footerBits; // 2与区间号最低位取或
        uint32_t price = CmptPriceSymbolReverse(encCtx,
            encCtx->probDistSpecial + base - distSlot - 1, footerBits, i - base);

        for (distState = 0; distState < 4; distState++) { // 填好了priceDistTable[STATE][4 ~ 128]
            encCtx->priceDistTable[distState][i] = price +
                                           encCtx->priceDistSlotTable[distState][distSlot];
        }
    }

    encCtx->matchPriceCount = 0; // 每次重置表都要初始化一下这个flag
}

CMPTLZ_HIDDEN void CmptPriceGenAlignTable(CmptLzEncCtx *encCtx)
{
    for (uint32_t i = 0; i < (1 << CMPTLZ_ALIGN_BITS); i++) {
        encCtx->priceAlignTable[i] = CmptPriceSymbolReverse(encCtx, encCtx->probAlign, CMPTLZ_ALIGN_BITS, i);
    }
}

CMPTLZ_HIDDEN uint32_t CmptPriceLiteral(CmptLzEncCtx *encCtx, bool matchMode, uint32_t matchByte, uint32_t symbol)
{
    uint32_t pos = encCtx->litMarcov.pos;
    uint32_t prevByte = encCtx->litMarcov.prevByte;
    uint32_t litCtx = encCtx->litMarcov.lcBits;
    uint32_t lpMask = encCtx->litMarcov.posMask;
    CmptlzProb *subCoder = CMPT_LIT_SUBCODER(encCtx->litMarcov.literal, litCtx, lpMask, pos, prevByte);

    uint32_t price = 0;
    if (!matchMode) {
        price = CmptPriceSymbol(encCtx, subCoder, 8, symbol); // lit有8位
    } else { // lit after match
        uint32_t offset = 0x100;
        symbol += 1 << 8; // 8比特 相当于 |= 1 0000 0000
        do {
            matchByte <<= 1;
            const uint32_t matchBit = matchByte & offset;
            const uint32_t subCoderIndex = offset + matchBit + (symbol >> 8); // 右移8位找下标
            const uint32_t bit = (symbol >> 7) & 1; // 右移7位找最高位
            price += CmptPriceOneBit(encCtx, subCoder[subCoderIndex], bit);
            symbol <<= 1;
            offset &= ~(matchByte ^ symbol);
        } while (symbol < (1 << 16)); // 左移16位为上界
    }
    return price;
}

static void CmptPriceSet(CmptLzEncCtx *encCtx, const CmptlzProb *probs, uint32_t startPrice, uint32_t *prices)
{
    uint32_t i;
    for (i = 0; i < 8; i += 2) { // 设置8个，每次++2
        uint32_t price = startPrice;
        uint32_t prob;
        price += CmptPriceOneBit(encCtx, probs[1           ], (i >> 2)); // 右移2
        price += CmptPriceOneBit(encCtx, probs[2 + (i >> 2)], (i >> 1) & 1); // 右移2
        prob = probs[4 + (i >> 1)]; // 4开始偏移
        prices[i    ] = price + CmptPriceBit0(encCtx, prob);
        prices[i + 1] = price + CmptPriceBit1(encCtx, prob);
    }
}

void CmptPriceGenLenTable(CmptLzEncCtx *encCtx, CmptLenEncoder *lenEncoder)
{
    const uint32_t numPosStates = 1 << encCtx->posBits;

    uint32_t b;
    /* gen low */
    uint32_t prob = lenEncoder->low[0];
    uint32_t a, c;
    uint32_t posState;

    b = CmptPriceBit1(encCtx, prob);
    a = CmptPriceBit0(encCtx, prob);
    c = b + CmptPriceBit0(encCtx, lenEncoder->low[1 << CMPT_LEN_LOW_BITS]);

    for (posState = 0; posState < numPosStates; posState++) {
        uint32_t *prices = lenEncoder->prices[posState];
        const CmptlzProb *probs = lenEncoder->low + (posState << (1 + CMPT_LEN_LOW_BITS));
        CmptPriceSet(encCtx, probs, a, prices);
        CmptPriceSet(encCtx, probs + (1 << CMPT_LEN_LOW_BITS), c, prices + (1 << CMPT_LEN_LOW_BITS));
    }
    /* gen high */
    uint32_t i = lenEncoder->tableSize;
    if (i > (1 << CMPT_LEN_LOW_BITS) * CMPT_DOUBLE) {
        const CmptlzProb *probs = lenEncoder->high;
        uint32_t *prices = lenEncoder->prices[0] + (1 << CMPT_LEN_LOW_BITS) * CMPT_DOUBLE;
        i -= (1 << CMPT_LEN_LOW_BITS) * CMPT_DOUBLE - 1;
        i >>= 1;
        b += CmptPriceBit1(encCtx, lenEncoder->low[(1 << CMPT_LEN_LOW_BITS)]);
        do {
            uint32_t sym = --i + (1 << (CMPT_LEN_HIGH_BITS - 1));
            uint32_t price = b;
            do {
                uint32_t bit = sym & 1;
                sym >>= 1;
                price += CmptPriceOneBit(encCtx, probs[sym], bit);
            } while (sym >= 2); // 2为下界

            prob = probs[(size_t)i + (1 << (CMPT_LEN_HIGH_BITS - 1))];
            prices[(size_t)i * CMPT_DOUBLE    ] = price + CmptPriceBit0(encCtx, prob);
            prices[(size_t)i * CMPT_DOUBLE + 1] = price + CmptPriceBit1(encCtx, prob);
        } while (i);
        size_t num = (lenEncoder->tableSize -
            (1 << CMPT_LEN_LOW_BITS) * CMPT_DOUBLE) * sizeof(lenEncoder->prices[0][0]);
        
        for (posState = 1; posState < numPosStates; posState++) {
            memcpy_s(lenEncoder->prices[posState] + (1 << CMPT_LEN_LOW_BITS) * CMPT_DOUBLE, CMPT_MF_LONGEST_MATCH - 1,
                lenEncoder->prices[0] + (1 << CMPT_LEN_LOW_BITS) * CMPT_DOUBLE, num);
        }
    }
}

/* length_update_prices */
CMPTLZ_HIDDEN uint32_t CmptPriceLen(CmptLenEncoder *lenEncoder, uint32_t len, uint32_t posState)
{
    // len的代价在enc中更新
    return lenEncoder->prices[posState][len - CMPTLZ_MATCH_LEN_MIN];
}

CMPTLZ_HIDDEN uint32_t CmptPriceShortRep(CmptLzEncCtx *encCtx, CmptlzState state, uint32_t posState) // 0 + 0
{
    return CmptPriceBit0(encCtx, encCtx->isRepG0[state]) +
           CmptPriceBit0(encCtx, encCtx->isRep0Long[state][posState]);
}

CMPTLZ_HIDDEN uint32_t CmptPriceLongRep(CmptLzEncCtx *encCtx, uint32_t longRepIndex,
    CmptlzState state, uint32_t posState) // 除去前两位 1 1 的其他代价 IsRepG0 & IsRepG1 & IsRepG2 & IsRep0Long
{
    uint32_t price = 0;
    switch (longRepIndex) {
        case 0: // 11 + 01 longrep0
            price = CmptPriceBit0(encCtx, encCtx->isRepG0[state]) +
                    CmptPriceBit1(encCtx, encCtx->isRep0Long[state][posState]);
            break;
        case 1: // 11 + 10 longrep1
            price = CmptPriceBit1(encCtx, encCtx->isRepG0[state]) +
                    CmptPriceBit0(encCtx, encCtx->isRepG1[state]);
            break;
        case 2: // 11 + 110  longrep2
            price = CmptPriceBit1(encCtx, encCtx->isRepG0[state]) +
                    CmptPriceBit1(encCtx, encCtx->isRepG1[state]) +
                    CmptPriceBit0(encCtx, encCtx->isRepG2[state]);
            break;
        case 3: // 11 + 111 longrep3
            price = CmptPriceBit1(encCtx, encCtx->isRepG0[state]) +
                    CmptPriceBit1(encCtx, encCtx->isRepG1[state]) +
                    CmptPriceBit1(encCtx, encCtx->isRepG2[state]);
            break;
        default:
            break;
    }
    return price;
}

CMPTLZ_HIDDEN uint32_t CmptPriceDistWithLen(CmptLzEncCtx *encCtx, uint32_t dist, uint32_t len, uint32_t posState)
{
    const uint32_t distState = CMPT_GET_DIST_STATE(len);
    uint32_t price;
    if (dist < 128) { // dist小于128已经加过distSlot
        price = encCtx->priceDistTable[distState][dist]; // 注：小于128的distpricetable是准的，已经加过distSlot,不含4比特align。
    } else {
        // 距离大于128的，priceDistTable就没填过了，只关注首：distSlot和尾：align，中间都是一样的
        // For match distances greater than 127, only the highest two bits and the
        // lowest four bits (alignment) is encoded using probabilities.
        uint32_t distSlot = PosSloter(dist);
        price = encCtx->priceDistSlotTable[distState][distSlot] +
                encCtx->priceAlignTable[dist & ((1 << CMPTLZ_ALIGN_BITS) - 1)]; // 掩码
    }

    price += CmptPriceLen(&encCtx->matchLenEncoder, len, posState);

    return price;
}