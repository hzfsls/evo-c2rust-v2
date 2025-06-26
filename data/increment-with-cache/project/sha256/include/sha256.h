/**
 * @file sha256.h
 * @copyright Copyright (c) Huawei Technologies Co., Ltd. 2021-2021. All rights reserved.
 * @brief sha256算法对外头文件。
 * @details \n
 * sha256算法把任意长度的数据经过一定处理之后，输出长度为256位的哈希值。本头文件提供sha256的对外接口。
 * @author c00464580
 * @date 2021-06-07
 * @version v1.0.0
 * *******************************************************************************************
 * @par 修改日志：
 * <table>
 * <tr><th>Date        <th>Version  <th>Author    <th>Description
 * <tr><td>2021-06-07  <td>1.0.0    <td>c00464580 <td>创建初始版本
 * </table>
 *
 * *******************************************************************************************
 */

/**
 * @defgroup sha256
 * @ingroup util
 */

#ifndef SHA256_H
#define SHA256_H

#include <stdint.h>
#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

/**
 * @ingroup sha256
 * @brief sha256算法上下文哈希数组长度
 */
#define VOS_SHA256_CTX_HASH_LEN 8

/**
 * @ingroup sha256
 * @brief sha256算法上下文缓冲区长度
 */
#define VOS_SHA256_CTX_BUF_LEN 16

/**
 * @ingroup sha256
 * @brief sha256算法处理块长度
 */
#define SHA256_BLOCK_SIZE 64

/**
 * @ingroup sha256
 * @brief sha256算法输出摘要长度
 */
#define SHA256_DIGEST_SIZE 32

/**
 * @ingroup sha256
 * @brief sha256算法上下文存储类型
 */
typedef struct {
    uint32_t h[8];                                          /**< 128 bits for SHA256 state */
    uint32_t N[2];                                          /**< input bits counter, max 2^64 bits */
    uint32_t block[SHA256_BLOCK_SIZE / sizeof(uint32_t)];   /**< block cache */
    uint32_t blocklen;
    uint32_t outlen;                                        /**< digest output length */
    uint32_t computed : 1;                                  /**< Is the digest computed */
    uint32_t corrupted : 1;                                 /**< Is the digest corrupted */
} VOS_SHA256_CTX;

/**
 * @ingroup sha256
 * @brief sha256算法初始化函数
 * @par 描述：初始化算法上下文内容
 * @attention \n
 * 调用者自行保证入参指针合法性。\n
 * pstCtx不能为空且地址长度足够。
 * @param  pstCtx         [IN|OUT]       指向上下文的指针。
 * @retval 无
 * @par 依赖：无。
 * @li sha256.h：该接口声明所在的文件。
 * @see vosSha256Hash
 */
void vosSha256Begin(VOS_SHA256_CTX *pstCtx);

/**
 * @ingroup sha256
 * @brief sha256算法哈希值更新函数
 * @par 描述：用输入的数据文本更新上下文中的哈希值
 * @attention \n
 * 调用者自行保证入参合法性。\n
 * pucData指向地址的长度必须与uiLen相匹配。\n
 * pstCtx不能为空且已被vosSha256Begin初始化。
 * @param  pucData        [IN]       指向输入文本的指针。
 * @param  uiLen          [IN]       输入文本的长度。
 * @param  pstCtx         [IN|OUT]   指向上下文的指针。
 * @retval 无
 * @par 依赖：无。
 * @li sha256.h：该接口声明所在的文件。
 * @see vosSha256End
 */
void vosSha256Hash(const uint8_t *pucData, uint32_t uiLen, VOS_SHA256_CTX *pstCtx);

/**
 * @ingroup sha256
 * @brief sha256算法获取摘要函数
 * @par 描述：将更新过的哈希值转化为摘要输出
 * @attention \n
 * 调用者自行保证入参合法性。\n
 * pucOut指向地址的长度必须与uiOutSize相匹配，且不小于算法摘要长度#SHA256_DIGEST_LEN。\n
 * pstCtx不能为空且已经通过vosSha256Hash更新哈希值。\n
 * @param  pucOut          [IN|OUT]   指向输出摘要的地址。
 * @param  uiOutSize       [IN]       输出摘要地址的长度，至少32字节。
 * @param  pstCtx          [IN|OUT]   指向上下文的指针。
 * @retval 无
 * @par 依赖：无。
 * @li sha256.h：该接口声明所在的文件。
 * @see
 */
void vosSha256End(uint8_t *pucOut, uint32_t uiOutSize, VOS_SHA256_CTX *pstCtx);

/**
 * @ingroup sha256
 * @brief sha256算法计算函数
 * @par 描述：对输入的文本执行sha256操作，将信息摘要保存到输出参数中
 * @attention \n
 * 调用者自行保证入参合法性。\n
 * pucInput指向地址的长度必须与uiInputLen相匹配。\n
 * pucOutput指向地址的长度必须与uiOutputLen相匹配，且不小于算法摘要长度#SHA256_DIGEST_LEN。\n
 * @param  pucInput       [IN]       指向输入文本的指针。
 * @param  uiInputLen     [IN]       输入文本的长度。
 * @param  pucOutput      [IN|OUT]   指向生成的摘要的指针。
 * @param  uiOutputLen    [IN]       输出缓冲区的长度，至少32字节。
 * @retval 无
 * @par 依赖：无。
 * @li sha256.h：该接口声明所在的文件。
 * @see
 */
void VOS_Sha256Calc(const uint8_t *pucInput, uint32_t uiInputLen, uint8_t *pucOutput, uint32_t uiOutputLen);

#ifdef __cplusplus
}
#endif

#endif
