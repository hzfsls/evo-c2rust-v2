/**
 * @file cmptlz_dec.h
 * @copyright Copyright (c) Huawei Technologies Co., Ltd. 2022-2023. All rights reserved.
 * @brief CMPTLZ 解压功能对外头文件
 * @author Anonym
 * @date 2023-07-04
 * @version v0.1.0
 ********************************************************************************************
 * @par History
 * <table>
 * <tr><th>Date        <th>Version   <th>Author      <th>Description
 * <tr><td>2023-07-04  <td>0.1.0     <td>   <td>Init version
 * </table>
 ********************************************************************************************
 */

#ifndef CMPTLZ_DEC_H
#define CMPTLZ_DEC_H

#include <stddef.h>
#include "cmptlz_base.h"

#ifdef __cplusplus
extern "C" {
#endif

/**
 * @ingroup cmptlz
 * @brief cmptlz解压结束模式
 */
typedef enum {
    CMPTLZ_FINISH_ANY,   /**< 解压可在待解压数据流任意处结束 */
    CMPTLZ_FINISH_END    /**< 解压必须在待解压数据流终止符处结束 */
} EnCmptLzFinMode;

/**
 * @ingroup cmptlz
 * @brief cmptlz解压结束状态
 */
typedef enum {
    CMPTLZ_STATUS_NOT_SPECIFIED,                /**< 解压过程出现错误 */
    CMPTLZ_STATUS_FINISHED_WITH_MARK,           /**< 解压在终止符处正常结束 */
    CMPTLZ_STATUS_NOT_FINISHED,                 /**< 待解压数据未解压完 */
    CMPTLZ_STATUS_NEEDS_MORE_INPUT,             /**< 待解压数据不足 */
    CMPTLZ_STATUS_MAYBE_FINISHED_WITHOUT_MARK,  /**< 解压在任意处正常结束 */
    CMPTLZ_STATUS_BUT
} EnCmptLzStatus;

/**
 * @ingroup cmptlz
 * @brief cmptlz解压设定入参
 */
typedef struct {
    unsigned char litCtx;         /**< 概率表内存对应句柄 */
    unsigned char litPos;         /**< 概率表内存对应句柄 */
    unsigned char posBits;        /**< 概率表内存对应句柄 */
    unsigned char alignPad;       /**< 对齐填充字段 */
    uint32_t dicSize;             /**< 解压字典大小 */
} CmptLzDecProt;

/**
 * @ingroup cmptlz
 * @brief cmptlz解压上下文
 */
typedef struct {
    CmptLzDecProt prop;              /**< 字典内存对应句柄 */
    CmptLzDecProb *probs;            /**< 字典内存对应句柄 */
    CmptLzDecProb *probsPlus1664;    /**< 字典内存对应句柄 */
    unsigned char *dict;             /**< 字典缓存区域首地址 */
    size_t dictBufSize;              /**< 字典缓存区域 */
    size_t dictPos;                  /**< 当前字典缓存区域的解压位置 */
    const unsigned char *buf;        /**< 待解压字符串 */
    uint32_t range;                  /**< 区间编码范围值 */
    uint32_t code;                   /**< 区间编码值 */
    uint32_t processedPos;           /**< 当前总解压位置 */
    uint32_t checkDicSize;           /**< 当前字典长度 */
    uint32_t reps[4];                /**< 当上1/2/3/4个LZ匹配对所对应的距离 */
    uint32_t state;                  /**< 当前状态机所处状态 */
    uint32_t remainLen;              /**< 上一轮解压中由于字典剩余长度不够导致无法写入的解压长度 */
    uint32_t numProbs;               /**< 当前probs概率表长度 */
    unsigned tempBufSize;            /**< 临时缓冲区内字符串的长度 */
    unsigned char tempBuf[CMPTLZ_REQUIRED_INPUT_MAX];  /**< 临时缓冲区放置上一轮解压中由于不完整无法解压的部分字符串 */
} CmptLzDecCtx;

/**
 * @ingroup cmptlz
 * @brief cmptlz待解压内容输入结构体
 */

typedef struct {
    const unsigned char *pSrcIn;  /**< 待解压字符串 */
    size_t strInLen;              /**< 待解压字符串长度 */
    size_t strInCostLen;          /**< 一次解压流程实际解压成功后，实际被解压的待解压字符串长度 */
} CmptLzDecIn;

/**
 * @ingroup cmptlz
 * @brief cmptlz解压获取内容输入结构体
 */
typedef struct {
    unsigned char *pDestOut;        /**< 解压后字符串存放地址 */
    size_t destOutLen;              /**< 解压后字符串存放地址长度 */
    size_t destOutFillLen;          /**< 一次解压流程实际解压成功后，实际解压得到的字符串长度 */
} CmptLzDecOut;

typedef struct {
    const unsigned char *protData;  /**< 数据头位置 */
    unsigned protSize;              /**< 数据头大小 */
    CmptLzMemHook *memHook;         /**< 内存申请释放钩子 */
} CmptlzDecParam;

/**
 * @ingroup cmptlz
 * @brief 初始化decCtx解压结构体两个参数
 * @par 描述: 初始化decCtx解压结构体
 * @attention
 * 无
 * @li Memory operation:
 * -# 不涉及</br>
 * @li Thread safe:
 * -# 不涉及</br>
 * @li OS/CPU difference:
 * -# 不涉及</br>
 * @li Time consuming:
 * -# 不涉及</br>
 * @li Permission Required:
 * -# 该接口不需要capbility</br>
 * @param decCtx           [OUT] 解压上下文信息结构体
 * @retval 无
 * @par 依赖: 如下
 * @li cmptlz：该接口所属的开发包。
 * @li cmptlz_dec.h：该接口声明所在的头文件。
 * @since V300R023C10
 */
void CmptLzDecConstruct(CmptLzDecCtx *decCtx);

/**
 * @ingroup cmptlz
 * @brief 初始化decCtx解压结构体
 * @par 描述: 初始化decCtx解压结构体
 * @attention
 * 无
 * @li Memory operation:
 * -# 涉及内存分配释放
 * @li Thread safe:
 * -# 不涉及</br>
 * @li OS/CPU difference:
 * -# 不涉及</br>
 * @li Time consuming:
 * -# 不涉及</br>
 * @li Permission Required:
 * -# 该接口不需要capbility</br>
 * @param decCtx           [OUT] 解压上下文信息结构体
 * @retval 无
 * @par 依赖: 如下
 * @li cmptlz：该接口所属的开发包。
 * @li cmptlz_dec.h：该接口声明所在的头文件。
 * @since V300R023C10
 */
void CmptLzDecInit(CmptLzDecCtx *decCtx);

/**
 * @ingroup cmptlz
 * @brief 申请压缩上下文所需内存
 * @par 描述: 申请压缩上下文所需内存，包括字典与概率表所需要的内存。
 * @attention
 * 无
 * @li Memory operation:
 * -# 使用用户传入的memHook钩子进行内存的申请</br>
 * @li Thread safe:
 * -# 不支持并发 </br>
 * @li OS/CPU difference:
 * -# 不涉及</br>
 * @li Time consuming:
 * -# 不涉及</br>
 * @li Permission Required:
 * -# 该接口不需要capbility</br>
 * @param decCtx           [IN/OUT] 解压上下文信息结构体，接口调用后字典与概率表指针会被初始化为申请值
 * @param protData         [IN] 设置入参的指针地址
 * @param protSize         [IN] 入参指针的地址长度
 * @param memHook          [IN] 用户传入的内存申请释放钩子
 * @retval =0   设置成功
 * @retval >0   设置失败,返回错误码
 * @par 依赖: 如下
 * @li cmptlz：该接口所属的开发包。
 * @li cmptlz_dec.h：该接口声明所在的头文件。
 * @since V300R023C10
 */
int CmptLzDecAllocate(CmptLzDecCtx *decCtx, const unsigned char *protData, unsigned protSize, CmptLzMemHook *memHook);

/**
 * @ingroup cmptlz
 * @brief 销毁压缩上下文所需内存
 * @par 描述: 销毁压缩上下文所需内存，包括字典与概率表所需要的内存。
 * @attention
 * 无
 * @li Memory operation:
 * -# 使用用户传入的memHook钩子进行内存的释放</br>
 * @li Thread safe:
 * -# 不支持并发 </br>
 * @li OS/CPU difference:
 * -# 不涉及</br>
 * @li Time consuming:
 * -# 不涉及</br>
 * @li Permission Required:
 * -# 该接口不需要capbility</br>
 * @param decCtx           [IN/OUT] 解压上下文信息结构体，接口调用后字典与概率表指针会被置空，对应内存会被释放
 * @param memHook          [IN] 用户传入的内存申请释放钩子
 * @retval =0   设置成功
 * @retval >0   设置失败,返回错误码
 * @par 依赖: 如下
 * @li cmptlz：该接口所属的开发包。
 * @li cmptlz_dec.h：该接口声明所在的头文件。
 * @since V300R023C10
 */
int CmptLzDecFree(CmptLzDecCtx *decCtx, CmptLzMemHook *memHook);

/**
 * @ingroup cmptlz
 * @brief 解压缩接口
 * @par 描述: 根据设定的解压模式，一直解压到传入outBuf满或者待解压src完全耗尽，
 * 同时返回此次解压字符串长度，消耗的压缩字符串的长度以及解压结束状态。
 * @attention
 * 特性1：支持多次解压，ouBuf可以小于解压出来的实际大小，通过多次解压得到完整内容；
 * 特性2：支持outbuf不需要明确指定，OutBuf可以大于实际解压后的大小，同时也可以小于字典大小
 * 特性3：finMode设定为Any，则一直解压直到Outbuffer满或者src完全耗尽，只要解压过程不出错，则一定返回解压OK
 * 特性4：finMode设定为EndMark,则解压后内容尾部一定需要存在End标识符，每次字典解压结束都会进行End标识符校验，如果解压结束后未找到标识符则报错
 * @li Memory operation:alloc、free、size:
 * -# 不涉及内存分配释放</br>
 * @li Thread safe:
 * -# 不支持并发</br>
 * @li OS/CPU difference:
 * -# 不涉及</br>
 * @li Time consuming:
 * -# 输入数据越大，则耗时越多</br>
 * @li Permission Required:
 * -# 该接口不需要capbility</br>
 * @param decCtx           [IN] 解压上下文信息结构体。
 * @param pDecIn           [IN/OUT] 待解压内容输入结构体，解压结束后strInCostLen成员会记录实际消耗的待解压字符串长度。
 * @param pDecOut          [IN/OUT] 解压获取内容输入结构体，解压结束后destOutFillLen成员会记录实际解压得到的字符串长度。
 * @param finMode          [IN] 结束模式，决定解压结束的方式，详细参考EnCmptLzFinMode的枚举定义。
 * @param finStatus        [OUT] 解压结束状态，详细参考EnCmptLzStatus的枚举定义。
 * @retval =0   解压成功
 * @retval >0   设置失败,返回错误码
 * @par 依赖: 如下
 * @li cmptlz：该接口所属的开发包。
 * @li cmptlz_dec.h：该接口声明所在的头文件。
 * @since V300R023C10
 */
int CmptLzDecDecodeToBuf(CmptLzDecCtx *decCtx, CmptLzDecIn *pDecIn, CmptLzDecOut *pDecOut,
    EnCmptLzFinMode finMode, EnCmptLzStatus *finStatus);

/**
 * @ingroup cmptlz
 * @brief 一键式解压缩接口
 * @par 描述: 不需要初始化解压上下文，根据设定的解压模式，一直解压到传入outBuf满或者待解压src完全耗尽，
 * 同时返回此次解压字符串长度，消耗的压缩字符串的长度以及解压结束状态。
 * @attention
 * 特性1：不支持多次解压，pDecOut的ouBuf成员大小推荐大于解压出来的实际大小，否则只能解压部分字符内容；
 * 特性2：finMode设定为Any，则一直解压直到Outbuffer满或者src完全耗尽，只要解压过程不出错，则一定返回解压OK
 * 特性3：finMode设定为EndMark,则解压后内容尾部一定需要存在End标识符，每次字典解压结束都会进行End标识符校验，如果解压结束后未找到标识符则报错
 * @li Memory operation:alloc、free、size:
 * -# 不涉及内存分配释放</br>
 * @li Thread safe:
 * -# 不支持并发</br>
 * @li OS/CPU difference:
 * -# 不涉及</br>
 * @li Time consuming:
 * -# 输入数据越大，则耗时越多</br>
 * @li Permission Required:
 * -# 该接口不需要capbility</br>
 * @param pDecIn           [IN/OUT] 待解压内容输入结构体，解压结束后strInCostLen成员会记录实际消耗的待解压字符串长度。
 * @param pDecOut          [IN/OUT] 解压获取内容输入结构体，解压结束后destOutFillLen成员会记录实际解压得到的字符串长度。
 * @param protData         [IN] 设置入参的指针地址
 * @param finMode          [IN] 结束模式，决定解压结束的方式，详细参考EnCmptLzFinMode的枚举定义。
 * @param finStatus        [OUT] 解压结束状态，详细参考EnCmptLzStatus的枚举定义。
 * @param memHook          [IN] 用户传入的内存申请释放钩子
 * @retval =0   解压成功
 * @retval >0   设置失败,返回错误码
 * @par 依赖: 如下
 * @li cmptlz：该接口所属的开发包。
 * @li cmptlz_dec.h：该接口声明所在的头文件。
 * @since V300R023C10
 */
int CmptLzDecode(CmptLzDecIn *pDecIn, CmptLzDecOut *pDecOut, const unsigned char *protData,
    EnCmptLzFinMode finMode, EnCmptLzStatus *finStatus, CmptLzMemHook *memHook);

/**
 * @ingroup cmptlz
 * @brief 简单解压接口
 * @par 描述: cmptlz简单解压接口，无需感知内部状态。
 * @attention
 * 特性1：src、dst不能为NULL
 * 特性2：srcSize和dstSize不能超过2G
 * 特性3：param不可以传NULL, 必须设置
 * @li Memory operation:alloc、free、size:
 * -# 使用用户传入的memHook钩子进行内存的申请</br>
 * @li Thread safe:
 * -# 不涉及</br>
 * @li OS/CPU difference:
 * -# 不涉及</br>
 * @li Time consuming:
 * -# 输入数据越大，则耗时越多</br>
 * @li Permission Required:
 * -# 该接口不需要capbility</br>
 * @param src          [IN] 待压缩的数据
 * @param srcSize      [IN] 待压缩数据的大小
 * @param dst          [IN] 输出缓冲区
 * @param dstSize      [IN/OUT] 传入输出缓冲区大小，传出压缩后数据大小
 * @param param        [IN] 解压参数，用户需要设置
 * @retval =0   压缩成功
 * @retval !=0   压缩成功,返回错误码
 * @par 依赖: 如下
 * @li cmptlz：该接口所属的开发包。
 * @li cmptlz_dec.h：该接口声明所在的头文件。
 * @since V300R024C00
 */
int CmptlzDecompress(void *src, size_t srcSize, void *dst, size_t *dstSize, CmptlzDecParam *param);

#ifdef __cplusplus
}
#endif /* __cplusplus */

#endif
