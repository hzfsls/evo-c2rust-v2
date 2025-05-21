/**
 * @file cmptlz_log.c
 * @copyright Copyright (c) Huawei Technologies Co., Ltd. 2024-2024. All rights reserved.
 * @brief CMPTLZ 日志文件
 * @author Anonym
 * @date 2024-01-09
 */

#include "securec.h"
#include "cmptlz_log.h"

#ifdef  __cplusplus
extern "C" {
#endif

#define LOG_BUF_SIZE 1024U

static CmptlzLogFunc g_cmptlzLogFunc = NULL;

void CmptlzLogWrite(size_t errorCode, const char *funcName, unsigned short line, const char *fmt, ...)
{
    va_list alist;
    char output[LOG_BUF_SIZE];
    int ret;
    size_t len;
    CmptlzLogFunc func = g_cmptlzLogFunc;

    if (func == NULL) {
        return;
    }

    ret = snprintf_s(output, LOG_BUF_SIZE, LOG_BUF_SIZE - 1,
        "\n[Cmptlz-Log] Func=%s, Line=%u, Error=0x%zx\n",
        funcName, line, errorCode);
    if (ret < 0) {
        return;
    }
    len = (size_t)ret;

    va_start(alist, fmt);
    ret = vsnprintf_s(output + len, LOG_BUF_SIZE - len, LOG_BUF_SIZE - len - 1, fmt, alist);
    va_end(alist);
    if (ret < 0) {
        return;
    }

    func(output, strlen(output) + 1);
}

void CmptlzLogRegister(CmptlzLogFunc func)
{
    g_cmptlzLogFunc = func;
}

#ifdef  __cplusplus
}
#endif

