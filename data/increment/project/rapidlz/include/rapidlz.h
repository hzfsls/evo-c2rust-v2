/**
 * @file rapidlz.h
 * @copyright Copyright (c) Huawei Technologies Co., Ltd. 2021-2023. All rights reserved.
 * @brief rapidlz 核心头文件
 * @details rapidlz 核心头文件
 * @author Anonym
 * @date 2022-6-29
 * @version v0.1.0
 * *******************************************************************************************
 * @par 修改日志：
 * <table>
 * <tr><th>Date        <th>Version  <th>Author       <th>Description
 * <tr><td>2022-6-29  <td>0.1.0    <td>        <td>Initial Version
 * </table>
 * *******************************************************************************************
 */

/**
 * @defgroup  rapidlz
 */

#ifndef RAPIDLZ_H
#define RAPIDLZ_H

#include <stddef.h>
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

/******************************************压缩接口对外说明******************************************/
/**
 * @ingroup rapidlz
 * @brief 根据输入流大小确定输出流最大所占空间
 * @par 描述: 提供输出流最大所占空间
 * @attention
 * rapidlz不支持srcSize大于0x7E000000U的情况，大于时返回0
 * @li Memory operation: alloc、free、size:
 * -# 不涉及</br>
 * @li Thread safe:
 * -# 线程安全，支持多线程调用</br>
 * @li OS/CPU difference:
 * -# 不涉及</br>
 * @li Time consuming:
 * -# 不涉及</br>
 * @li Permission Required:
 * -# 该接口不需要capbility</br>
 * @param srcSize  [IN] 待压缩数据流大小，单位为Byte
 * @retval >=0 返回输出流最大所占空间
 * @par 依赖: 如下
 * @li rapidlz：该接口所属的开发包。
 * @li rapidlz.h：该接口声明所在的头文件。
 * @since V300R023C00
 */
size_t RapidlzCompressBound(size_t srcSize);

/**
 * @ingroup rapidlz
 * @brief 压缩接口
 * @par 描述: 提供压缩功能
 * @attention
 * 步长越大，压缩率越低，压缩性能越高
 * 压缩数据输出内存块大小需大于等于#RapidlzCompressBound的返回值。
 * 传入的内存地址及其内存大小必须匹配，否则可能造成越界访问。
 * 规格限制：#RapidlzCompressBound接口获取的输出缓冲区大小超过2G时，不支持压缩。
 * @li Memory operation: alloc、free、size:
 * -# 涉及内存分配释放</br>
 * @li Thread safe:
 * -# 线程安全，支持多线程调用</br>
 * @li OS/CPU difference:
 * -# 不涉及</br>
 * @li Time consuming:
 * -# 输入数据越大，则耗时越多</br>
 * @li Permission Required:
 * -# 该接口不需要capbility</br>
 * @param src               [IN] 待压缩数据流
 * @param dst               [OUT] 输出buffer, 用来存放压缩好的数据流
 * @param srcSize           [IN] 输入数据流大小
 * @param dstSize           [IN] 输出数据流中留存的内存空间,单位为Byte
 * @param acceleration      [IN] 压缩步长，等级为1~10的整数，超范围则压缩失败
 * @retval >0 返回压缩后的数据大小
 * @retval =0 压缩失败
 * @par 依赖: 如下
 * @li rapidlz：该接口所属的开发包。
 * @li rapidlz.h：该接口声明所在的头文件。
 * @since V300R023C00
 */
size_t RapidlzCompress(const void *src, void *dst, size_t srcSize, size_t dstSize, int acceleration);

/******************************************解压缩接口对外说明******************************************/
/**
 * @ingroup rapidlz
 * @brief 解压接口
 * @par 描述: 提供解压功能
 * @attention
 * 传入的内存地址及其内存大小必须匹配，否则可能造成越界访问。
 * 规格限制：输出缓冲区大小超过2G时，不支持解压。
 * 数据内容限制：输入数据必须为LZ4压缩接口或者Rapidlz压缩接口压缩后的数据，不可以随机传入数据或者破坏压缩后内容，
 * 否则可能出现解压越界访问
 * @li Memory operation: alloc、free、size:
 * -# 涉及内存分配释放</br>
 * @li Thread safe:
 * -# 线程安全，支持多线程调用</br>
 * @li OS/CPU difference:
 * -# 不涉及</br>
 * @li Time consuming:
 * -# 输入数据越大，则耗时越多</br>
 * @li Permission Required:
 * -# 该接口不需要capbility</br>
 * @param src               [IN] 待解压数据流
 * @param dst               [OUT] 输出buffer, 用来存放解压好的数据流
 * @param srcSize           [IN] 输入数据流大小
 * @param dstSize           [IN] 输出数据流中留存的内存空间,单位为Byte
 * @retval >0 返回压缩后的数据大小
 * @retval =0 解压失败
 * @par 依赖: 如下
 * @li rapidlz：该接口所属的开发包。
 * @li rapidlz.h：该接口声明所在的头文件。
 * @since V300R023C00
 */
size_t RapidlzDecompress(const void *src, void *dst, size_t srcSize, size_t dstSize);

/******************************************日志接管注册接口对外说明******************************************/

/**
 * @ingroup rapidlz
 * @brief 日志记录钩子原型
 * @attention 由于压缩、解压接口支持并发，所以此接口实现须用户保证支持并发。
 * @param  message  [IN] 日志信息
 * @param  size     [IN] 日志信息长度
 */
typedef void (*RapidlzLogFunc)(const char *message, size_t size);

/**
 * @ingroup rapidlz
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
 * @li rapidlz：该接口所属的开发包。
 * @li rapidlz.h：该接口声明所在的头文件。
 * @since V300R023C00
 */
void RapidlzLogRegister(RapidlzLogFunc func);

/******************************************字典压缩解压接口对外说明******************************************/
#define RAPIDLZ_STREAM_HASH_SIZE (1 << 12)
/**
 * @ingroup rapidlz
 * @brief 流式压缩结构体 & 字典压缩结构体
 */
typedef struct TagRapidlzStreamCtx RapidlzStreamCtx;
struct TagRapidlzStreamCtx {
    uint32_t hashTable[RAPIDLZ_STREAM_HASH_SIZE];   /* 流式压缩中哈希表大小固定，且使用4字节哈希值 */
    const uint8_t *dict;                            /* 本轮流式压缩的字典 */
    uint32_t dictSize;                              /* 本轮流式字典大小 */
    uint32_t currentOffset;                         /* 用作记录，可以找到最初src位置base == src - offset */
    int acceleration;                               /* 压缩步长，输入为1 ~ 10的整数，超出范围会规范化 */
    RapidlzStreamCtx *strmCtxSpecific;              /* 可被载入的dict结构体，类似一个常驻字典，当存在该字典时，优先使用该结构体。所以流式压缩过程不用每次都载入该字典 */
};

/**
 * @ingroup rapidlz
 * @brief 字典载入接口
 * @par 描述: 用户可以自定义字典内容并载入到流式压缩结构体
 * @attention
 * @li Memory operation: alloc、free、size:
 * -# 不涉及</br>
 * @li Thread safe:
 * -# 不涉及</br>
 * @li OS/CPU difference:
 * -# 不涉及</br>
 * @li Time consuming:
 * -# 不涉及</br>
 * @li Permission Required:
 * -# 该接口不需要capbility</br>
 * @param strmCtx            [IN|OUT] 字典压缩结构体，传入一块申请好的结构体内存，传出一个配置好的字典结构体
 * @param dictionary         [IN] 用户需要载入的字典
 * @param dictSize           [IN] 用户需要载入的字典大小
 * @retval >0 载入后的字典长度
 * @retval =0 载入字典失败
 * @par 依赖: 如下
 * @li rapidlz：该接口所属的开发包。
 * @li rapidlz.h：该接口声明所在的头文件。
 * @since V300R024C10
 */
int RapidlzLoadDict(RapidlzStreamCtx* strmCtx, const char *dictionary, int dictSize);

/**
 * @ingroup rapidlz
 * @brief 流式压缩接口 & 字典压缩接口
 * @par 描述: 用户可以通过多次调用该接口实现流式压缩或单次调用该接口实现字典压缩
 * @attention
 * 传入的内存地址及其内存大小必须匹配，否则可能造成越界访问。
 * 规格限制：字典压缩接口单次待压缩文件长度最大为0x7E000000
 * 0字节压缩：不支持srcSize != 0但是src == NULL的数据传入，但支持传入srcSize == 0的数据压缩（无论src是否为NULL）
 * 字典限制：压缩解压缩必须使用同一个字典，字典有效大小最大为64 * 1024Bytes，超过该值，只取尾部64KB作为字典。
 * @li Memory operation: alloc、free、size:
 * -# 不涉及</br>
 * @li Thread safe:
 * -# 不涉及</br>
 * @li OS/CPU difference:
 * -# 不涉及</br>
 * @li Time consuming:
 * -# 不涉及</br>
 * @li Permission Required:
 * -# 该接口不需要capbility</br>
 * @param strmCtx            [IN|OUT] 字典压缩结构体
 * @param src                [IN] 待压缩数据地址
 * @param dst                [IN] 压缩后数据存放地址
 * @param srcSize            [IN] 待压缩数据长度
 * @param dstSize            [IN] 压缩后数据缓冲区的长度
 * @param acceleration       [IN] 压缩步长，输入为1 ~ 10的整数，超出范围会规范化
 * @retval >0 压缩后数据长度
 * @retval <=0 压缩失败
 * @par 依赖: 如下
 * @li rapidlz：该接口所属的开发包。
 * @li rapidlz.h：该接口声明所在的头文件。
 * @since V300R024C10
 */
int RapidlzCompressStream(RapidlzStreamCtx* strmCtx, const char* src, char* dst, int srcSize, int dstSize);

/**
 * @ingroup rapidlz
 * @brief 字典解压接口
 * @par 描述: 用户可以通过调用该接口实现字典解压
 * @attention
 * 传入的内存地址及其内存大小必须匹配，否则可能造成越界访问。
 * 规格限制：输出缓冲区大小超过2G时，不支持解压。
 * 数据内容限制：输入数据必须为LZ4压缩接口或者Rapidlz压缩接口压缩后的数据，不可以随机传入数据或者破坏压缩后内容，
 * 否则可能出现解压越界访问
 * @li Memory operation: alloc、free、size:
 * -# 不涉及</br>
 * @li Thread safe:
 * -# 不涉及</br>
 * @li OS/CPU difference:
 * -# 不涉及</br>
 * @li Time consuming:
 * -# 不涉及</br>
 * @li Permission Required:
 * -# 该接口不需要capbility</br>
 * @param src                [IN] 待解压数据地址
 * @param dst                [IN] 解压后数据存放地址
 * @param compressedSize     [IN] 压缩后数据长度
 * @param dstSize            [IN] 解压后数据缓冲区长度
 * @param dictStart          [IN] 字典指针
 * @param dictSize           [IN] 字典长度
 * @retval >0 解压后数据长度
 * @retval <=0 解压失败
 * @par 依赖: 如下
 * @li rapidlz：该接口所属的开发包。
 * @li rapidlz.h：该接口声明所在的头文件。
 * @since V300R024C10
 */
int RapidlzDecompressSafeUsingDict(const char *src, char *dst, int compressedSize, int dstSize,
    const char *dictStart, int dictSize);
#ifdef __cplusplus
}
#endif

#endif
