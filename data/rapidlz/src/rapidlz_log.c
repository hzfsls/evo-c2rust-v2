/*
 * @file rapidlz_log.c
 * Copyright (c) Huawei Technologies Co., Ltd. 2023-2023. All rights reserved.
 * Description: 日志记录模块机制
 * Author: Anonym
 * Create: 2023-2-14
 */

#include <libgen.h>
#include <stdarg.h>
#include "securec.h"
#include "rapidlz_log.h"

#ifdef  __cplusplus
extern "C" {
#endif

#define LOG_BUF_SIZE 1024

static RapidlzLogFunc g_rapidlzLogFunc = NULL;

void RapidlzLogWrite(size_t error_code, const char *file_name, unsigned short line, const char *fmt, ...)
{
    va_list alist;
    char output[LOG_BUF_SIZE];
    int retVal, len;
    RapidlzLogFunc func = g_rapidlzLogFunc;
    char *filename;

    if (func == NULL) {
        return;
    }

    filename = strdup(file_name);
    if (filename == NULL) {
        return;
    }

    retVal = snprintf_s(output, LOG_BUF_SIZE, LOG_BUF_SIZE - 1,
        "\n[Rapidlz-Log] File=%s, Line=%u, Error=%zu\n",
        basename(filename), line, error_code);
    if (retVal < 0) {
        free(filename);
        return;
    }
    len = retVal;

    free(filename);

    va_start(alist, fmt);
    retVal = vsnprintf_s(output + len, LOG_BUF_SIZE - len, LOG_BUF_SIZE - len - 1, fmt, alist);
    va_end(alist);
    if (retVal < 0) {
        return;
    }

    func(output, strlen(output) + 1);
}

void RapidlzLogRegister(RapidlzLogFunc func)
{
    g_rapidlzLogFunc = func;
}

#ifdef  __cplusplus
}
#endif

