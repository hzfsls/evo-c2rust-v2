/**
 * @file cmptlz_end.h
 * @copyright Copyright (c) Huawei Technologies Co., Ltd. 2024-2024. All rights reserved.
 * @brief CMPTLZ 压缩功能对外头文件
 * @author Anonym
 * @date 2024-01-11
 */
#ifndef CMPTLZ_ENC_H
#define CMPTLZ_ENC_H

#include <stddef.h>
#include "cmptlz_base.h"

#ifdef __cplusplus
extern "C" {
#endif

/**
 * @ingroup cmptlz
 * @brief cmptlz压缩结构体
 */
typedef struct TagCmptLzEncCtx CmptLzEncCtx;

/**
 * @ingroup cmptlz
 * @brief cmptlz压缩入参
 */
typedef struct {
    int level;               /**< 压缩等级，支持0 ~ 9 */
    uint32_t dictSize;       /**< LZ匹配字典，支持1024 Bytes ~ 128M Bytes */
    int litCtx;              /**< 压缩参数，default = 3，支持litCtx + litPos <= 4的使用 */
    int litPos;              /**< 压缩参数，default = 0， 支持litCtx + litPos <= 4的使用 */
    int posBits;             /**< 压缩参数，default = 2， 支持0 <= posBits <= 4 */
    int fastBytes;           /**< 压缩参数，在level <= 6 时default = 32，level >= 7时default = 64，支持5 <= fastBytes <= 273 */
    int numThreads;          /**< 当前仅支持单线程，numThreads == 1 */
    unsigned char *protData; /**< 5字节头，1字节压缩参数，4字节字典，小端序读法 */
    size_t protSize;         /**< 与lzma兼容，仅可以为5 */
    CmptLzMemHook *memHook;  /**< 内存申请释放钩子 */
} CmptlzCompParam;

/**
 * @ingroup cmptlz
 * @brief 压缩接口
 * @par 描述: cmptlz普通压缩接口
 * @attention
 * 特性1：dst、param不能为NULL
 * 特性2：srcSize和dstSize不能超过2G Bytes,dstSize应预留尽可能大。
 * 特性3：param.dictSize不能超过128M Bytes, 三个概率表相关变量仅支持litCtx + litPos <= 4
 * 默认litCtx, litPos, posBits分别为3,0,2
 * 特性4：dictSize 最小支持1024,传入更小则会强制由level设置
 * @li Memory operation:alloc、free、size:
 * -# 使用用户传入的memHook钩子进行内存的申请</br>
 * @li Thread safe:
 * -# 支持并发</br>
 * @li OS/CPU difference:
 * -# 不涉及</br>
 * @li Time consuming:
 * -# 输入数据越大，则耗时越多</br>
 * @li Permission Required:
 * -# 该接口不需要capbility</br>
 * @param dst          [IN] 输出缓冲区
 * @param dstSize      [IN/OUT] 传入输出缓冲区大小，传出压缩后数据大小
 * @param src          [IN] 待压缩的数据
 * @param srcSize      [IN] 待压缩数据的大小
 * @param param        [IN] 压缩等级、内存钩子等配置信息
 * @retval =0   压缩成功
 * @retval !=0   压缩失败,返回错误码
 * @par 依赖: 如下
 * @li cmptlz：该接口所属的开发包。
 * @li cmptlz_enc.h：该接口声明所在的头文件。
 * @since V300R024C00
 */
int CmptlzCompress(void *src, size_t srcSize, void *dst, size_t *dstSize, CmptlzCompParam *param);

#ifdef __cplusplus
}
#endif /* __cplusplus */

#endif