/**
 * @file v_md5.h
 * @copyright Copyright (c) Huawei Technologies Co., Ltd. 2021-2021. All rights reserved.
 * @brief md5 对外头文件
 * @details MD5 摘要算法对外头文件
 * @author c00580207
 * @date 2021-08-03
 * @version v0.1.0
 * *******************************************************************************************
 * @par 修改日志：
 * <table>
 * <tr><th>Date        <th>Version  <th>Author       <th>Description
 * <tr><td>2021-08-03  <td>0.1.0    <td>c00580207    <td>创建初始版本
 * </table>
 * *******************************************************************************************
 */

/**
 * @defgroup md5 MD5摘要算法
 * @ingroup md5
 */

#ifndef V_MD5_H
#define V_MD5_H

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif /* __cpluscplus */

/**
 * @ingroup md5
 * @brief 本数据结构描述了MD5的上下文。该上下文用来维护MD5的状态、MD5计算的位数以及当前操作的输入缓冲区。
 */
typedef struct TagMd5Ctx {
    uint32_t aulState[4];     /**< Different states of MD5 buffer */
    uint32_t aulCount[2];     /**< number of bits, modulo 2^64 (lsb first) */
    uint8_t aucBuffer[64];    /**< input buffer */
    uint32_t uiPos;           /**< position of objective */
} MD5_CTX;

/**
 * @ingroup md5
 * @brief 本接口初始化一个MD5上下文。
 * @par 描述:本接口用来初始化一个MD5上下文，使用内部定义的数值来填充MD5上下文的成员变量。
 * @attention \n
 * 用户需要验证输入参数的有效性。\n
 * 不符合公司安全要求，禁止MD5用于生成数字签名/密匙保存这两种场景。
 * @param context   [IN]    指向MD5上下文的指针。
 * @retval 无。
*/
void VOS_MD5Init(MD5_CTX *context);

/**
 * @ingroup md5
 * @brief 本接口用来对输入文本进行编码以及更新信息摘要。
 * @par 描述:本接口将长度为inputLen的输入文本input进行编码后更新保存在context中的MD5上下文。
 * @attention \n
 * 调用本接口前需要调用 #VOS_MD5Init 对context进行初始化。\n
 * 最终的信息摘要可以通过调用接口 #VOS_MD5Final 获取。用户需要验证输入参数的有效性。\n
 * 不符合公司安全要求，禁止MD5用于生成数字签名/密匙保存这两种场景。
 * @param context   [IN/OUT]    指向MD5上下文的指针。
 * @param input     [IN]        指向输入文本的指针。
 * @param inputLen  [IN]        表示输入文本的长度, 如果分多次调用这个接口，输入长度总和应小于2^64位。
 * @retval 无。
*/
void VOS_MD5Update(MD5_CTX *context, uint8_t *input, uint32_t inputLen);

/**
 * @ingroup md5
 * @brief 本接口计算指定MD5上下文的信息摘要。
 * @par 描述:本接口为MD5上下文context中保存的输入文本计算其信息摘要并保存到digest中。
 * @attention \n
 * 调用本接口前，用户需要调用 #VOS_MD5Init 对context进行初始化。\n
 * 用户可以调用接口 #VOS_MD5Update 对输入文本进行编码以及更新信息摘要。用户需要验证输入参数的有效性。\n
 * 不符合公司安全要求，禁止MD5用于生成数字签名/密匙保存这两种场景。
 * @param digest[16]    [OUT]   指向生成的信息摘要的指针。
 * @param context       [IN]    指向MD5上下文的指针。
 * @retval 无。
*/
void VOS_MD5Final(uint8_t digest[16], MD5_CTX *context);

/**
 * @ingroup md5
 * @brief 本接口计算指定MD5上下文的信息摘要。
 * @par 描述:本接口为MD5上下文context中保存的输入文本计算其信息摘要并保存到digest中。
 * @attention \n
 * 调用本接口前，用户需要调用 #VOS_MD5Init 对context进行初始化。\n
 * 用户可以调用接口 #VOS_MD5Update 对输入文本进行编码以及更新信息摘要。用户需要验证输入参数的有效性。\n
 * 不符合公司安全要求，禁止MD5用于生成数字签名/密匙保存这两种场景。
 * @param digest[]  [OUT]   指向生成的信息摘要的指针。
 * @param bufLen    [IN]    生成信息摘要的缓冲区长度，应不小于16字节
 * @param context   [IN]    指向MD5上下文的指针。
 * @retval 无。
*/
void VOS_MD5FinalEx(uint8_t digest[], uint32_t bufLen, MD5_CTX *context);

/**
 * @ingroup md5
 * @brief 本接口对输入的文本执行MD5操作，并将信息摘要保存到输出参数中。
 * @par 描述:
 * 本接口首先创建一个MD5上下文的数据结构，将输入参数input中保存的文本保存到MD5上下文中，
 * 然后再调用接口 #VOS_MD5Final 为其计算信息摘要，得出的信息摘要保存在参数output中。
 * @attention \n
 * 用户需要验证输入参数的有效性。\n
 * 本接口封装了函数：#VOS_MD5Init、 #VOS_MD5Update 和 #VOS_MD5Final。\n
 * 不符合公司安全要求，禁止MD5用于生成数字签名/密匙保存这两种场景。
 * @param output    [OUT]   指向生成的信息摘要的指针，大小不得小于16字节。
 * @param input     [IN]    指向输入文本的指针。
 * @param inputLen  [IN]    表示输入文本的长度。
 * @retval 无。
*/
void VOS_MD5Calc(uint8_t *output, uint8_t *input, uint32_t inputLen);

/**
 * @ingroup md5
 * @brief 本接口对输入的文本执行MD5操作，并将信息摘要保存到输出参数中。
 * @par 描述:
 * 本接口首先创建一个MD5上下文的数据结构，将输入参数input中保存的文本保存到MD5上下文中，
 * 然后再调用接口 #VOS_MD5Final 为其计算信息摘要，得出的信息摘要保存在参数output中。
 * @attention \n
 * 用户需要验证输入参数的有效性。\n
 * 本接口封装了函数：#VOS_MD5Init、 #VOS_MD5Update 和 #VOS_MD5Final。\n
 * 不符合公司安全要求，禁止MD5用于生成数字签名/密匙保存这两种场景。
 * @param output    [OUT]   指向生成的信息摘要的指针。
 * @param outputLen [IN]    生成信息摘要的缓冲区长度，应不小于16字节。
 * @param input     [IN]    指向输入文本的指针。
 * @param inputLen  [IN]    表示输入文本的长度。
 * @retval 无。
*/
void VOS_MD5CalcEx(uint8_t *output, uint32_t outputLen, const uint8_t *input, uint32_t inputLen);

#ifdef __cplusplus
}
#endif /* __cpluscplus */

#endif /* V_MD5_H */

