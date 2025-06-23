/**
 * @file bzp_utils.h
 * @copyright Copyright (c) Huawei Technologies Co., Ltd. 2023-2023. All rights reserved.
 * @brief 提供bzp算法用到宏定义
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
#ifndef BZP_UTILS_H
#define BZP_UTILS_H

#include <stdbool.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include "bzp_type.h"
#include "securec.h"

#ifdef __cplusplus
extern "C" {
#endif
// 基础块的大小 实际块的大小需要乘level
#define BZP_BASE_BLOCK_SIZE 100000
// 压缩时输入的压缩等级，用于设定压缩块的大小
#define BZP_BLOCK_SIZE_LEVEL_UPPER_LIMIT 9
#define BZP_BLOCK_SIZE_LEVEL_LOWER_LIMIT 1
// 块大小异常
#define BZP_INVALID_BLOCK_SIZE(blockSize) \
    ((blockSize) < BZP_BLOCK_SIZE_LEVEL_LOWER_LIMIT || (blockSize) > BZP_BLOCK_SIZE_LEVEL_UPPER_LIMIT)
// 无效的alphaSize
#define BZP_INVALID_ALPHA_SIZE(alphaSize) ((alphaSize) > BZP_MAX_ALPHA_SIZE || (alphaSize) < 1)

// 块保留空间，大部分块会让其余留部分空间
#define BZP_BLOCK_RESERVED_SPACE_SIZE 19

// bwt
// 调用希尔排序的阈值
#define BZP_THRESHOLD_SHELL_SORT 10
// 栈的大小
#define BZP_MAX_STACK_SIZE 100
// 出现字符的种类数量
#define BZP_ASCII_SIZE 256
// 希尔排序增量的种类
#define BZP_SHELL_SORT_INCREMENT_NUMS 2
// 希尔排序的2个增量
#define BZP_SHELL_SORT_INCREMENT0 1
#define BZP_SHELL_SORT_INCREMENT1 4

// mtf编码时，对重复出现的0进行特殊的编码
#define BZP_MTF_ENCODE0 0
#define BZP_MTF_ENCODE1 1
#define BZP_MTF_ENCODE_BASE 2

// crc 初始化值
#define BZP_INIT_BLOCK_CRC 0xffffffffL

// huffman
// huffman树中出现的最大字符种类数量
#define BZP_MAX_ALPHA_SIZE 258
// huffman编码最多建的树的数量
#define BZP_MAX_GROUPS_NUM 6
// huffman编码迭代次数上限
#define BZP_MAX_ITER_NUM 4
// huffman编码树高上限
#define BZP_MAX_TREE_HEIGHT_ENCODE 17

// huffman编码中，根据block中元素数量来确定建树的数量
#define BZP_NGROUPS_BLOCK_NUM_LIMIT0 200
#define BZP_NGROUPS_BLOCK_NUM_LIMIT1 600
#define BZP_NGROUPS_BLOCK_NUM_LIMIT2 1200
#define BZP_NGROUPS_BLOCK_NUM_LIMIT3 2400
#define BZP_NGROUPS_NUM_0 2
#define BZP_NGROUPS_NUM_1 3
#define BZP_NGROUPS_NUM_2 4
#define BZP_NGROUPS_NUM_3 5
#define BZP_NGROUPS_NUM_4 6

// huffman编码中，50个元素为一组，来选择同一个huffman树进行编码
#define BZP_ELEMS_NUM_IN_ONE_GROUP 50
// huffman编码中，树权重由频数和树高组成，这里是树高的位数
#define BZP_HUFFMAN_HEIGHT_WEIGHT_BITS 8
// huffman编码中，len数组初始化的最大值
#define BZP_HUFFMAN_LEN_MAX_COST 15

// len的最大值，老版的是20，最新的上限为17
#define BZP_HUFFMAN_LEN_UPPER_LIMIT 20
// huffman编码中，select数组最大值长度
#define BZP_HUFFMAN_MAX_SIZE_SELECT \
    (BZP_BLOCK_SIZE_LEVEL_UPPER_LIMIT * BZP_BASE_BLOCK_SIZE / BZP_ELEMS_NUM_IN_ONE_GROUP)
// 求最大值最小值的函数
#define BZP_MAX_FUN(a, b) (((a) > (b)) ? (a) : (b))
#define BZP_MIN_FUN(a, b) (((a) < (b)) ? (a) : (b))

#ifdef __cplusplus
}
#endif

#endif
