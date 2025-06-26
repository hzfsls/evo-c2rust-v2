/**
 * @file cmptlz_base.h
 * @copyright Copyright (c) Huawei Technologies Co., Ltd. 2022-2023. All rights reserved.
 * @brief CMPTLZ 基本数据类型头文件
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

/**
 * @defgroup cmptlz cmptlz
 */

#ifndef CMPTLZ_BASE_H
#define CMPTLZ_BASE_H

#include <stdint.h>
#include <stdbool.h>
#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

/**
 * @ingroup cmptlz
 * @brief cmptlz 模块定义
 */
#define CMPTLZ_MODULE (0x0A00 + 0x0D)

#define CMPTLZ_ERROR_CONVERT(x) (int32_t)((CMPTLZ_MODULE << 16) | (uint32_t)(x))

typedef enum {
    CMPTLZ_ERROR_DATA = 1,          /**< 输入stream数据错误 */
    CMPTLZ_ERROR_MEM,               /**< 内存相关操作错误 */
    CMPTLZ_ERROR_UNSUPPORTED,       /**< 不支持的入参 */
    CMPTLZ_ENC_ERROR_FILESIZE,      /**< 压缩文件大小错误 */
    CMPTLZ_ENC_CTX_INIT_FAIL,       /**< 压缩结构体初始化错误 */
    CMPTLZ_ENC_RC_INIT_FAIL,        /**< 区间编码初始化错误 */
    CMPTLZ_ENC_MF_INIT_FAIL,        /**< 匹配器初始化错误 */
    CMPTLZ_ENC_ERROR_WRITE,         /**< 拷贝到输出buffer错误 */
    CMPTLZ_ENC_ERROR_HEAD,          /**< 写文件头错误 */
    CMPTLZ_ENC_ERROR_PARAM,         /**< 用户传入参数错误 */
    CMPTLZ_ERROR_BUTT
} EnCmptErrNo;

/**
 * @ingroup cmptlz
 * @brief 成功情况的返回值
 */
#define CMPT_OK 0

/**
 * @ingroup cmptlz
 * 0x0A0D0001:输入stream数据错误
 */
#define CMPT_ERROR_DATA CMPTLZ_ERROR_CONVERT(CMPTLZ_ERROR_DATA)

/**
 * @ingroup cmptlz
 * 0x0A0D0002:内存相关操作错误
 */
#define CMPT_ERROR_MEM CMPTLZ_ERROR_CONVERT(CMPTLZ_ERROR_MEM)

/**
 * @ingroup cmptlz
 * 0x0A0D0003:不支持的入参
 */
#define CMPT_ERROR_UNSUPPORTED CMPTLZ_ERROR_CONVERT(CMPTLZ_ERROR_UNSUPPORTED)

/**
 * @ingroup cmptlz
 * 0x0A0D0004:压缩文件大小错误
 */
#define CMPT_ENC_ERROR_FILESIZE CMPTLZ_ERROR_CONVERT(CMPTLZ_ENC_ERROR_FILESIZE)

/**
 * @ingroup cmptlz
 * 0x0A0D0005:压缩结构体初始化错误
 */
#define CMPT_ENC_CTX_INIT_FAIL CMPTLZ_ERROR_CONVERT(CMPTLZ_ENC_CTX_INIT_FAIL)

/**
 * @ingroup cmptlz
 * 0x0A0D0006:区间编码初始化错误
 */
#define CMPT_ENC_RC_INIT_FAIL CMPTLZ_ERROR_CONVERT(CMPTLZ_ENC_RC_INIT_FAIL)

/**
 * @ingroup cmptlz
 * 0x0A0D0007:匹配器初始化错误
 */
#define CMPT_ENC_MF_INIT_FAIL CMPTLZ_ERROR_CONVERT(CMPTLZ_ENC_MF_INIT_FAIL)

/**
 * @ingroup cmptlz
 * 0x0A0D0008:拷贝到输出buffer错误
 */
#define CMPT_ENC_ERROR_WRITE CMPTLZ_ERROR_CONVERT(CMPTLZ_ENC_ERROR_WRITE)

/**
 * @ingroup cmptlz
 * 0x0A0D0009:写文件头错误
 */
#define CMPT_ENC_ERROR_HEAD CMPTLZ_ERROR_CONVERT(CMPTLZ_ENC_ERROR_HEAD)

/**
 * @ingroup cmptlz
 * 0x0A0D0010:用户传入参数错误
 */
#define CMPT_ENC_ERROR_PARAM CMPTLZ_ERROR_CONVERT(CMPTLZ_ENC_ERROR_PARAM)

/**
 * @ingroup cmptlz
 * @brief 头部入参缓冲区
 */
#define CMPTLZ_PROPS_SIZE 5

/**
 * @ingroup cmptlz
 * @brief 头部实际解压大小解析缓冲区
 */
#define CMPTLZ_UNCOMP_SIZE 8

/**
 * @ingroup cmptlz
 * @brief 最小解压长度
 */
#define CMPTLZ_REQUIRED_INPUT_MAX 20

/**
 * @ingroup cmptlz
 * @brief cmptlz概率表元素长度
 */
typedef uint16_t CmptLzDecProb;

/**
 * @ingroup cmptlz
 * @brief cmptlz内部使用的内存句柄
 */
#define CMPTLZ_HANDLE_CONVERT(x) (int32_t)((CMPTLZ_MODULE << 16) | ((uint32_t)(x) << 8))

typedef enum {
    CMPTLZ_DICT_MEM = 1,                /**< 字典内存对应句柄 */
    CMPTLZ_PROB_MEM,                    /**< 概率表内存对应句柄 */
    CMPTLZ_ENC_CCTX,                    /**< 压缩大结构体内存对应句柄 */
    CMPTLZ_MF_CCTX,                     /**< 压缩匹配模块内存对应句柄 */
    CMPTLZ_MF_HASH,                     /**< 压缩匹配模块HASH对应句柄 */
    CMPTLZ_MF_SON,                      /**< 压缩匹配模块SON对应句柄 */
    CMPTLZ_RC_CCTX,                     /**< 压缩区间编码模块内存对应句柄 */
    CMPTLZ_RC_BUF,                      /**< 压缩区间编码模块BUFFERBASE对应句柄 */
    CMPTLZ_MEM_TYPE_BUT
} EnCmptLzMemType;

/**
 * @ingroup cmptlz
 * @brief 内存分配释放钩子函数对
 */
typedef struct {
    void *(*CmptLzAlloc)(int32_t enMemType, size_t size);
    void (*CmptLzFree)(int32_t enMemType, void *address);
} CmptLzMemHook;

/**
 * @ingroup cmptlz
 * 0x0A0D0100:字典所需要内存对应句柄
 */
#define CMPTLZ_DICT_HANDLE CMPTLZ_HANDLE_CONVERT(CMPTLZ_DICT_MEM)

/**
 * @ingroup cmptlz
 * 0x0A0D0200:概率表所需要内存对应句柄
 */
#define CMPTLZ_PROB_HANDLE CMPTLZ_HANDLE_CONVERT(CMPTLZ_PROB_MEM)

/**
 * @ingroup cmptlz
 * 0x0A0D0300:压缩大结构体内存对应句柄
 */
#define CMPTLZ_ENC_CCTX_HANDLE CMPTLZ_HANDLE_CONVERT(CMPTLZ_ENC_CCTX)

/**
 * @ingroup cmptlz
 * 0x0A0D0400:压缩匹配模块内存对应句柄
 */
#define CMPTLZ_MF_CCTX_HANDLE CMPTLZ_HANDLE_CONVERT(CMPTLZ_MF_CCTX)

/**
 * @ingroup cmptlz
 * 0x0A0D0500:压缩匹配模块HASH对应句柄
 */
#define CMPTLZ_MF_HASH_HANDLE CMPTLZ_HANDLE_CONVERT(CMPTLZ_MF_HASH)

/**
 * @ingroup cmptlz
 * 0x0A0D0600:压缩匹配模块SON对应句柄
 */
#define CMPTLZ_MF_SON_HANDLE CMPTLZ_HANDLE_CONVERT(CMPTLZ_MF_SON)

/**
 * @ingroup cmptlz
 * 0x0A0D0700:压缩区间编码模块内存对应句柄
 */
#define CMPTLZ_RC_CCTX_HANDLE CMPTLZ_HANDLE_CONVERT(CMPTLZ_RC_CCTX)

/**
 * @ingroup cmptlz
 * 0x0A0D0800:压缩区间编码模块BUFFERBASE对应句柄
 */
#define CMPTLZ_RC_BUF_HANDLE CMPTLZ_HANDLE_CONVERT(CMPTLZ_RC_BUF)

/**
 * @brief 日志记录钩子
 * @attention 由于压缩、解压接口支持并发，所以此接口实现须用户保证支持并发。
 * @param  message  [IN] 日志信息
 * @param  size     [IN] 日志长度
 */
typedef void (*CmptlzLogFunc)(const char *message, size_t size);
 
/**
 * @ingroup cmptlz
 * @brief 日志钩子注册接口
 * @par 描述: 用户可以自定义日志输出，默认不记录日志。
 * @attention
 * @li Memory operation: alloc、free、size:
 * -# 不涉及</br>
 * @li Thread safe:
 * -# 不支持并发，以最后一次注册为准</br>
 * @li OS/CPU difference:
 * -# 不涉及</br>
 * @li Time consuming:
 * -# 不涉及</br>
 * @li Permission Required:
 * -# 该接口不需要capbility</br>
 * @param func     [IN] 日志记录钩子
 * @retval 无
 * @par 依赖: 如下
 * @li cmptlz：该接口所属的开发包。
 * @li cmptlz_base.h：该接口声明所在的头文件。
 * @since V300R024C00
 */
void CmptlzLogRegister(CmptlzLogFunc func);

#ifdef __cplusplus
}
#endif /* __cplusplus */

#endif