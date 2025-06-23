/**
 * @file bzp_type.h
 * @copyright Copyright (c) Huawei Technologies Co., Ltd. 2023-2023. All rights reserved.
 * @brief bzp对外类型定义头文件
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
#ifndef BZP_TYPE_H
#define BZP_TYPE_H

/** @name 正常返回*/
#define BZP_OK 0 // 成功

/** @name 基本错误 */
enum BZP_ERROR_BASE_NO {
    BZP_ERROR_MEMORY_OPER_FAILURE = 1, // 内存失败错误码
    BZP_ERROR_PARAM,                   // 参数错误
    BZP_ERROR_IO,                      // IO错误(文件)
    BZP_ERROR_DATA, // 数据错误( 用于解压，检验解压中间的块的头部和整体的尾部)包含crc
    BZP_ERROR_DATA_MAGIC, // 数据错误( 用于解压，检验解压头部文件格式)
};

/** @name 流式错误码 */
enum BZP_ERROR_STREAM_NO {
    BZP_ERROR_STREAM_COMPRESS_FAILUIRE = 10 // 流式压缩失败错误码
};

// typedef enum { BZP_ERROR_BASE_NO, BZP_ERROR_STREAM_NO } BZP_ERROR_NO;

#endif