use crate::translation_utils::*;

pub type BZP_ERROR_BASE_NO = i32;
macro_rules! BZP_ERROR_MEMORY_OPER_FAILURE { () => { 1 } }
pub(crate) use BZP_ERROR_MEMORY_OPER_FAILURE;
macro_rules! BZP_ERROR_PARAM { () => { 2 } }
pub(crate) use BZP_ERROR_PARAM;
macro_rules! BZP_ERROR_IO { () => { 3 } }
pub(crate) use BZP_ERROR_IO;
macro_rules! BZP_ERROR_DATA { () => { 4 } }
pub(crate) use BZP_ERROR_DATA;
macro_rules! BZP_ERROR_DATA_MAGIC { () => { 5 } }
pub(crate) use BZP_ERROR_DATA_MAGIC;


pub type BZP_ERROR_STREAM_NO = i32;
macro_rules! BZP_ERROR_STREAM_COMPRESS_FAILUIRE { () => { 10 } }
pub(crate) use BZP_ERROR_STREAM_COMPRESS_FAILUIRE;


macro_rules! BZP_TYPE_H { () => {  } }
pub(crate) use BZP_TYPE_H;


macro_rules! BZP_OK { () => { 0 } }
pub(crate) use BZP_OK;


