/**
 * @file rapidlz_log.h
 * @copyright Copyright (c) Huawei Technologies Co., Ltd. 2023-2023. All rights reserved.
 * @brief rapidlz日志机制
 * @details 定义了显示日志的一些基本方法
 * @author Anonym
 * @date 2023-1-28
 * @version v1.0
 ********************************************************************************************
 * @par 修改日志：
 * <table>
 * <tr><th>Date        <th>Version  <th>Author    <th>Description
 * <tr><td>2023-1-28    <td>1.0      <td>Anonym    <td>创建初始版本
 * </table>
 *
 ********************************************************************************************
 */
#ifndef RAPIDLZ_LOG_H
#define RAPIDLZ_LOG_H

#include <stdio.h>
#include "rapidlz.h"

#ifdef __cplusplus
extern "C" {
#endif

// #ifndef RAPIDLZFILENAME
// #ifdef RAPIDLZ_FILE_NAME
// #define RAPIDLZFILENAME ""
// #else
#define RAPIDLZFILENAME (strrchr(__FILE__, '/') ? strrchr(__FILE__, '/') + 1 : __FILE__)
// #endif
// #endif

/**
 * @ingroup rapidlz_log
 * @brief 日志记录错误
 * @param  error_code   [IN]  错误码。
 * @param  fmt   [IN]  日志格式。
 * @param  args   [IN]  参数。
 */
#define RAPIDLZ_LOG(error_code, fmt, args...) \
    do { \
        RapidlzLogWrite((size_t)(error_code), RAPIDLZFILENAME, __LINE__, fmt, ##args); \
    } while (0)

/**
 * @ingroup rapidlz_log
 * @brief 输出日志。
 * @par 描述：输出日志记录。
 * @attention
 * @param  level        [IN]  重要等级
 * @param  error_code   [IN]  错误码
 * @param  file_name    [IN]  日志记录的文件名
 * @param  line         [IN]  日志记录的文件名的行数
 * @param  fmt          [IN]  日志格式
 * @retval 无
 * @par 依赖：\n
 * @li rapidlz_log.h：该接口声明所在的文件。
 * @see
 */
void RapidlzLogWrite(size_t error_code, const char *file_name, unsigned short line,
    const char *fmt, ...) __attribute__((format(printf, (4), (5))));

#ifdef __cplusplus
}
#endif
#endif /* RAPIDLZ_LOG_H */

