#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
enum BZP_ERROR_BASE_NO {
    BZP_ERROR_MEMORY_OPER_FAILURE = 1,
    BZP_ERROR_PARAM,
    BZP_ERROR_IO,
    BZP_ERROR_DATA,
    BZP_ERROR_DATA_MAGIC,
}
