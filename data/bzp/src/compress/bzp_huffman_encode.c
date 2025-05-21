/**
 * @file bzp_huffman_encode.c
 * @copyright Copyright (c) Huawei Technologies Co., Ltd. 2023-2023. All rights reserved.
 * @brief 提供了bzp压缩中huffman算法的初始化、编码、资源释放等接口
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
#include "bzp_huffman_encode.h"

#ifdef __cplusplus
extern "C" {
#endif
// 单个Huffman树构建
void BzpHuffmanInit(int32_t alphaSize, BzpHuffmanInfo *huffman)
{
    (void)memset_s(huffman->len, sizeof(huffman->len), 0, sizeof(huffman->len));
    huffman->nHeap = 0;
    huffman->nWeight = 0;
    huffman->alphaSize = alphaSize;
}
// 初始化
void BzpHuffmanInitArray(BzpHuffmanInfo *huffman)
{
    int32_t i;
    huffman->nHeap = 0;
    huffman->nWeight = huffman->alphaSize;

    for (i = 0; i < huffman->alphaSize; i++) {
        huffman->parent[i] = -1;
    }
}
// 堆的向上调整-初始化堆的过程中
void BzpHeapAdjustUp(int32_t *heap, int32_t *weight, int32_t pos)
{
    int32_t tmpw = weight[heap[pos]];
    int32_t tmpv = heap[pos];
    while (pos > 1) {
        if (tmpw < weight[heap[pos >> 1]]) {
            heap[pos] = heap[pos >> 1];
            pos >>= 1;
        } else {
            break;
        }
    }
    heap[pos] = tmpv;
}
// 堆的向下调整-向堆中添加删除元素的的过程中
void BzpHeapAdjustDown(int32_t *heap, int32_t *weight, int32_t nHeap)
{
    int32_t pos = 1;
    int32_t chpos = pos << 1; // 左子树节点id
    int32_t tmpid = heap[pos];
    int32_t tmpv = weight[tmpid];
    while (chpos <= nHeap) {
        if ((chpos | 1) <= nHeap && weight[heap[chpos]] > weight[heap[chpos | 1]]) {
            chpos |= 1;
        }
        if (tmpv < weight[heap[chpos]]) {
            break;
        }
        heap[pos] = heap[chpos];
        pos = chpos;
        chpos = pos << 1;
    }
    heap[pos] = tmpid;
}
// 堆初始化
void BzpHeapInit(BzpHuffmanInfo *huffman)
{
    int32_t i = 0;
    for (i = 0; i < huffman->alphaSize; i++) {
        huffman->nHeap++;
        huffman->heap[huffman->nHeap] = i;
        BzpHeapAdjustUp(huffman->heap, huffman->weight, huffman->nHeap);
    }
}
// 节点权重加和
// 权重定义：（频数，节点高度）
int32_t BzpHuffmanWeightAdd(int32_t w1, int32_t w2)
{
    return ((w1 & 0xffffff00) + (w2 & 0xffffff00)) | (BZP_MAX_FUN((w1 & 0x000000ff), (w2 & 0x000000ff)) + 1);
}
// 建树
void BzpBuildHuffmanTree(BzpHuffmanInfo *huffman)
{
    BzpHuffmanInitArray(huffman);
    BzpHeapInit(huffman);
    int32_t idx1, idx2;
    while (huffman->nHeap > 1) {
        idx1 = huffman->heap[1];
        huffman->heap[1] = huffman->heap[huffman->nHeap--];
        BzpHeapAdjustDown(huffman->heap, huffman->weight, huffman->nHeap);
        idx2 = huffman->heap[1];
        huffman->heap[1] = huffman->heap[huffman->nHeap--];
        BzpHeapAdjustDown(huffman->heap, huffman->weight, huffman->nHeap);
        huffman->weight[huffman->nWeight] = BzpHuffmanWeightAdd(huffman->weight[idx1], huffman->weight[idx2]);
        huffman->parent[idx1] = huffman->nWeight;
        huffman->parent[idx2] = huffman->nWeight;
        huffman->parent[huffman->nWeight] = -1;
        huffman->nHeap++;
        huffman->heap[huffman->nHeap] = huffman->nWeight;
        huffman->nWeight++;
        BzpHeapAdjustUp(huffman->heap, huffman->weight, huffman->nHeap);
    }
}

// 建树并求得树的高度
int32_t BzpGetCodeLen(BzpHuffmanInfo *huffman)
{
    int32_t maxlen = 0;

    BzpBuildHuffmanTree(huffman);
    int32_t i;
    maxlen = 0;
    for (i = 0; i < huffman->alphaSize; i++) {
        int32_t x = i;
        int32_t tlen = 0;
        while (huffman->parent[x] >= 0) {
            x = huffman->parent[x];
            tlen++;
        }
        huffman->len[i] = tlen;
        maxlen = BZP_MAX_FUN(maxlen, tlen);
    }

    return maxlen;
}
// 调用建树求高度的函数，用来限制树的高度
void BzpBuildTreeBalanceHeight(BzpHuffmanInfo *huffman)
{
    int32_t maxlen = 0;
    for (int32_t i = 0; i < huffman->alphaSize; i++) {
        if (huffman->weight[i] == 0) {
            huffman->weight[i] = 1 << BZP_HUFFMAN_HEIGHT_WEIGHT_BITS;
        } else {
            huffman->weight[i] <<= BZP_HUFFMAN_HEIGHT_WEIGHT_BITS;
        }
    }

    do {
        maxlen = BzpGetCodeLen(huffman);
        // 树高超限，调整权值
        if (maxlen > BZP_MAX_TREE_HEIGHT_ENCODE) {
            for (int32_t i = 0; i < huffman->alphaSize; i++) {
                int32_t w = (huffman->weight[i] >> BZP_HUFFMAN_HEIGHT_WEIGHT_BITS);
                w = ((w >> 1) + 1);
                huffman->weight[i] = w << BZP_HUFFMAN_HEIGHT_WEIGHT_BITS;
            }
        }
    } while (maxlen > BZP_MAX_TREE_HEIGHT_ENCODE);
}
// 求编码表
void BzpGetHuffmanTable(BzpHuffmanInfo *huffman)
{
    int32_t vec = 0;
    int32_t mi = huffman->len[0], mx = huffman->len[0];
    for (int32_t i = 0; i < huffman->alphaSize; i++) {
        mi = BZP_MIN_FUN(mi, huffman->len[i]);
        mx = BZP_MAX_FUN(mx, huffman->len[i]);
    }
    for (int32_t i = mi; i <= mx; i++) {
        for (int32_t j = 0; j < huffman->alphaSize; j++) {
            if (huffman->len[j] == i) {
                huffman->table[j] = vec;
                vec++;
            }
        }
        vec <<= 1;
    }
}

// 重置
int32_t BzpHuffmanGroupsReset(BzpHuffmanGroups *huffman, int32_t alphaSize)
{
    if (BZP_INVALID_ALPHA_SIZE(alphaSize)) {
        return BZP_ERROR_PARAM;
    }

    huffman->alphaSize = alphaSize;
    huffman->block = NULL;
    huffman->mtfFreq = NULL;
    huffman->nSelect = 0;
    huffman->nGroups = 0;

    for (int32_t i = 0; i < BZP_MAX_GROUPS_NUM; i++) {
        BzpHuffmanInit(alphaSize, &huffman->huffmanGroups[i]);
    }
    return BZP_OK;
}

// 建立一组huffman树，初始化
BzpHuffmanGroups *BzpHuffmanGroupsInit(int32_t blockSize)
{
    if (BZP_INVALID_BLOCK_SIZE(blockSize)) {
        return NULL;
    }
    BzpHuffmanGroups *huffmanGroups = (BzpHuffmanGroups *)malloc(sizeof(BzpHuffmanGroups));
    if (huffmanGroups == NULL) {
        return NULL;
    }
    huffmanGroups->select = NULL;
    huffmanGroups->selectMTF = NULL;
    int32_t spaceSize = blockSize * BZP_BASE_BLOCK_SIZE / BZP_ELEMS_NUM_IN_ONE_GROUP;
    huffmanGroups->select = (int32_t *)malloc(spaceSize * sizeof(int32_t));
    huffmanGroups->selectMTF = (int32_t *)malloc(spaceSize * sizeof(int32_t));
    if (huffmanGroups->select == NULL || huffmanGroups->selectMTF == NULL) {
        BzpBzpHuffmanGroupsFinish(huffmanGroups);
        return NULL;
    }
    huffmanGroups->alphaSize = 0;
    huffmanGroups->block = NULL;
    huffmanGroups->mtfFreq = NULL;
    huffmanGroups->nSelect = 0;
    huffmanGroups->nGroups = 0;

    for (int32_t i = 0; i < BZP_MAX_GROUPS_NUM; i++) {
        BzpHuffmanInit(0, &huffmanGroups->huffmanGroups[i]);
    }

    return huffmanGroups;
}
// 资源释放
void BzpBzpHuffmanGroupsFinish(BzpHuffmanGroups *huffman)
{
    if (huffman != NULL) {
        if (huffman->select != NULL) {
            free(huffman->select);
            huffman->select = NULL;
        }
        if (huffman->selectMTF != NULL) {
            free(huffman->selectMTF);
            huffman->selectMTF = NULL;
        }
        free(huffman);
        huffman = NULL;
    }
}
// 根据块内元素数量确定建树的数量
int32_t BzpGetHuffmanGroups(int32_t nBlock)
{
    int32_t nGroups = 1;
    if (nBlock < BZP_NGROUPS_BLOCK_NUM_LIMIT0) {
        nGroups = BZP_NGROUPS_NUM_0;
    } else if (nBlock < BZP_NGROUPS_BLOCK_NUM_LIMIT1) {
        nGroups = BZP_NGROUPS_NUM_1;
    } else if (nBlock < BZP_NGROUPS_BLOCK_NUM_LIMIT2) {
        nGroups = BZP_NGROUPS_NUM_2;
    } else if (nBlock < BZP_NGROUPS_BLOCK_NUM_LIMIT3) {
        nGroups = BZP_NGROUPS_NUM_3;
    } else {
        nGroups = BZP_NGROUPS_NUM_4;
    }
    return nGroups;
}

void BzpGenerateSelectMTF(BzpHuffmanGroups *huffman)
{
    int32_t nGroups = huffman->nGroups;
    int32_t list[nGroups];
    for (int32_t i = 0; i < nGroups; i++) {
        list[i] = i;
    }
    for (int32_t i = 0; i < huffman->nSelect; i++) {
        int32_t pos = 0;
        for (int32_t j = 0; j < nGroups; j++) {
            if (huffman->select[i] == list[j]) {
                pos = j;
                break;
            }
        }
        for (int32_t j = pos; j > 0; j--) {
            list[j] = list[j - 1];
        }
        list[0] = huffman->select[i];
        huffman->selectMTF[i] = pos;
    }
}
// 初始化len数组
void BzpInitLenArray(BzpHuffmanGroups *huffman)
{
    int32_t nGroups = huffman->nGroups;
    int32_t npart = nGroups;
    int32_t AllFreqNum = huffman->nBlock;
    int32_t st = 0, ed;
    // 枚举字符表，根据出现次数，来按顺序分配给每个树，来初始化这个树的len
    while (npart > 0) {
        int32_t NowFreqNum = 0;
        int32_t FreqNumLimit = AllFreqNum / npart; // 当前树分配的所有字符频数的综合的最大值

        ed = st - 1;
        while (ed < huffman->alphaSize - 1 && NowFreqNum < FreqNumLimit) {
            ed++;
            NowFreqNum += huffman->mtfFreq[ed];
        }

        // 当前分组不是第一个和最后一个组，是奇数个组。来保证Freq小于上限
        if (ed > st && npart != nGroups && npart != 1 && ((nGroups - npart) & 1)) {
            NowFreqNum -= huffman->mtfFreq[ed];
            ed--;
        }
        // 根据字符是否在这个树中，初始化Len
        for (int32_t i = 0; i < huffman->alphaSize; i++) {
            if (i >= st && i <= ed) {
                huffman->huffmanGroups[npart - 1].len[i] = 0;
            } else {
                huffman->huffmanGroups[npart - 1].len[i] = BZP_HUFFMAN_LEN_MAX_COST;
            }
        }
        npart--;
        st = ed + 1;
        AllFreqNum -= NowFreqNum;
    }
}

void BzpCalculateCost(BzpHuffmanGroups *huffman, int32_t st, int32_t ed)
{
    (void)memset_s(huffman->cost, sizeof(huffman->cost), 0, sizeof(huffman->cost));
    int32_t nGroups = huffman->nGroups;
    for (int32_t k = st; k <= ed; k++) {
        for (int32_t t = 0; t < nGroups; t++) {
            huffman->cost[t] += huffman->huffmanGroups[t].len[huffman->block[k]];
        }
    }
}
int32_t BzpSelectTree(BzpHuffmanGroups *huffman)
{
    int32_t id = 0;
    int32_t nGroups = huffman->nGroups;
    for (int32_t k = 0; k < nGroups; k++) {
        if (huffman->cost[k] < huffman->cost[id]) {
            id = k;
        }
    }
    huffman->select[huffman->nSelect++] = id;
    return id;
}
// 建立一组huffman树-主要流程
void BzpHuffmanMain(BzpHuffmanGroups *huffman)
{
    int32_t nGroups = BzpGetHuffmanGroups(huffman->nBlock);
    huffman->nGroups = nGroups;
    // 初始化数树 的len数组
    BzpInitLenArray(huffman);
    int32_t st = 0, ed;
    // 迭代N次，优化建树
    for (int32_t i = 0; i < BZP_MAX_ITER_NUM; i++) {
        for (int32_t j = 0; j < nGroups; j++) {
            (void)memset_s(huffman->huffmanGroups[j].weight, sizeof(huffman->huffmanGroups[j].weight), 0,
                           sizeof(huffman->huffmanGroups[j].weight));
        }

        st = 0;
        huffman->nSelect = 0;
        while (st < huffman->nBlock) { // [st,ed]
            ed = BZP_MIN_FUN(huffman->nBlock, st + (int32_t)BZP_ELEMS_NUM_IN_ONE_GROUP) - 1;
            // 统计花费
            BzpCalculateCost(huffman, st, ed);
            // 分配树
            int32_t id = BzpSelectTree(huffman);
            // 更新频率
            for (int32_t k = st; k <= ed; k++) {
                huffman->huffmanGroups[id].weight[huffman->block[k]]++;
            }
            st = ed + 1;
        }
        // 建N个huffman树 限制树高
        for (int32_t j = 0; j < nGroups; j++) {
            BzpBuildTreeBalanceHeight(&huffman->huffmanGroups[j]);
        }
    }
    // 生成select的MTF表
    BzpGenerateSelectMTF(huffman);
    // 生成编码表
    for (int32_t i = 0; i < nGroups; i++) {
        BzpGetHuffmanTable(&huffman->huffmanGroups[i]);
    }
}

#ifdef __cplusplus
}
#endif
