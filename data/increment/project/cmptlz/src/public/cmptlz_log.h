/**
 * @file cmptlz_log.h
 * @copyright Copyright (c) Huawei Technologies Co., Ltd. 2024-2024. All rights reserved.
 * @brief CMPTLZ 日志头文件
 * @author Anonym
 * @date 2024-01-09
 */

#ifndef CMPTLZ_LOG_H
#define CMPTLZ_LOG_H

#include <stdlib.h>
#include <string.h>
#include <stdio.h>
#include "cmptlz_base.h"

#ifdef __cplusplus
extern "C" {
#endif

/**
 * @ingroup log
 * @brief 日志记录错误
 * @param  error_code   [IN]  错误码。
 * @param  fmt   [IN]  日志格式。
 * @param  args   [IN]  参数。
 */
#define CMPTLZ_LOG(error_code, fmt, args...) \
    do { \
        CmptlzLogWrite((size_t)(error_code), __FUNCTION__, __LINE__, fmt, ##args); \
    } while (0)

/**
 * @ingroup log
 * @brief 输出日志。
 * @par 描述：输出日志记录。
 * @attention
 * @param  level   [IN]  重要等级
 * @param  errorCode   [IN]  错误码
 * @param  funcName   [IN]  日志记录的文件名
 * @param  line   [IN]  日志记录的文件名的行数
 * @param  fmt   [IN]  日志格式
 * @retval 无
 * @par 依赖：\n
 * @li list.h：该接口声明所在的文件。
 * @see
 */
void CmptlzLogWrite(size_t errorCode, const char *funcName, unsigned short line,
    const char *fmt, ...) __attribute__((format(printf, (4), (5))));

#ifdef __cplusplus
}
#endif /* __cplusplus */

#endif /* CMPTLZ_LOG_H */