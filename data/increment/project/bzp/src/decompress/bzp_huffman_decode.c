/**
 * @file bzp_huffman_decode.c
 * @copyright Copyright (c) Huawei Technologies Co., Ltd. 2023-2023. All rights reserved.
 * @brief 提供了bzp解压中huffman的初始化、解码、资源释放等接口
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
#include "bzp_huffman_decode.h"

#ifdef __cplusplus
extern "C" {
#endif
// 初始化
BzpHuffmanDecode *BzpHuffmanDecodeInit(int32_t blockSize)
{
    if (BZP_INVALID_BLOCK_SIZE(blockSize)) {
        return NULL;
    }
    BzpHuffmanDecode *huffman = (BzpHuffmanDecode *)malloc(sizeof(BzpHuffmanDecode));
    if (huffman == NULL) {
        return NULL;
    }
    int32_t spaceSize = BZP_BASE_BLOCK_SIZE * blockSize / BZP_ELEMS_NUM_IN_ONE_GROUP;
    huffman->select = (int32_t *)malloc(spaceSize * sizeof(int32_t));
    if (huffman->select == NULL) {
        BzpHuffmanDecodeFinish(huffman);
    }

    (void)memset_s(huffman->base, sizeof(huffman->base), 0, sizeof(huffman->base));
    (void)memset_s(huffman->perm, sizeof(huffman->perm), 0, sizeof(huffman->perm));
    (void)memset_s(huffman->limit, sizeof(huffman->limit), 0, sizeof(huffman->limit));

    huffman->selectCnt = 0;
    huffman->deCodeNum = 0;
    return huffman;
}
// 重置结构体内容
void BzpHuffmanDecodeReset(BzpHuffmanDecode *huffman)
{
    (void)memset_s(huffman->base, sizeof(huffman->base), 0, sizeof(huffman->base));
    (void)memset_s(huffman->perm, sizeof(huffman->perm), 0, sizeof(huffman->perm));
    (void)memset_s(huffman->limit, sizeof(huffman->limit), 0, sizeof(huffman->limit));

    huffman->selectCnt = 0;
    huffman->deCodeNum = 0;
}

// 构建解码表
void BzpGetOneTable(BzpHuffmanDecode *huffman, int32_t t)
{
    int32_t vec = 0, cnt = 0;
    int32_t mi = huffman->len[t][0], mx = huffman->len[t][0];
    for (int32_t i = 0; i < huffman->alphaSize; i++) {
        mi = BZP_MIN_FUN(mi, huffman->len[t][i]);
        mx = BZP_MAX_FUN(mx, huffman->len[t][i]);
    }
    huffman->minLens[t] = mi;
    for (int32_t i = mi; i <= mx; i++) {
        for (int32_t j = 0; j < huffman->alphaSize; j++) {
            if (huffman->len[t][j] == i) {
                huffman->perm[t][cnt++] = j;
            }
        }
    }
    for (int32_t i = 0; i < huffman->alphaSize; i++) {
        huffman->base[t][huffman->len[t][i] + 1]++;
    }

    // base[t][i]为小于i的编码的数量 mx+1上限为树的高度，原版设置为23
    for (int32_t i = 1; i <= mx + 1; i++) {
        huffman->base[t][i] += huffman->base[t][i - 1];
    }

    for (int32_t i = mi; i <= mx; i++) {
        // 长度为i的编码出现的次数，这里求的vec是长度为i的编码的上限
        vec += (huffman->base[t][i + 1] - huffman->base[t][i]);
        // 这里是存了长度为i的最后一个编码
        huffman->limit[t][i] = vec - 1;
        vec <<= 1;
    }
    for (int32_t i = mi + 1; i <= mx; i++) {
        huffman->base[t][i] = ((huffman->limit[t][i - 1] + 1) << 1) - huffman->base[t][i];
    }
}

void BzpGenerateDecodeTable(BzpHuffmanDecode *huffman)
{
    for (int32_t t = 0; t < huffman->nGroups; t++) {
        BzpGetOneTable(huffman, t);
    }
}
// 资源释放
void BzpHuffmanDecodeFinish(BzpHuffmanDecode *huffman)
{
    if (huffman != NULL) {
        if (huffman->select != NULL) {
            free(huffman->select);
            huffman->select = NULL;
        }

        free(huffman);
        huffman = NULL;
    }
}

#ifdef __cplusplus
}
#endif
