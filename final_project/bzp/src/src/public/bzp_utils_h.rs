use crate::translation_utils::*;
pub use crate::include::bzp_type_h::*;

macro_rules! BZP_UTILS_H { () => { } }
pub(crate) use BZP_UTILS_H;


macro_rules! BZP_BASE_BLOCK_SIZE { () => { 100000 } }
pub(crate) use BZP_BASE_BLOCK_SIZE;


macro_rules! BZP_BLOCK_SIZE_LEVEL_UPPER_LIMIT { () => { 9 } }
pub(crate) use BZP_BLOCK_SIZE_LEVEL_UPPER_LIMIT;


macro_rules! BZP_BLOCK_SIZE_LEVEL_LOWER_LIMIT { () => { 1 } }
pub(crate) use BZP_BLOCK_SIZE_LEVEL_LOWER_LIMIT;


macro_rules! BZP_BLOCK_RESERVED_SPACE_SIZE { () => { 19 } }
pub(crate) use BZP_BLOCK_RESERVED_SPACE_SIZE;


macro_rules! BZP_THRESHOLD_SHELL_SORT { () => { 10 } }
pub(crate) use BZP_THRESHOLD_SHELL_SORT;


macro_rules! BZP_MAX_STACK_SIZE { () => { 100 } }
pub(crate) use BZP_MAX_STACK_SIZE;


macro_rules! BZP_ASCII_SIZE { () => { 256 } }
pub(crate) use BZP_ASCII_SIZE;


macro_rules! BZP_SHELL_SORT_INCREMENT_NUMS { () => { 2 } }
pub(crate) use BZP_SHELL_SORT_INCREMENT_NUMS;


macro_rules! BZP_SHELL_SORT_INCREMENT0 { () => { 1 } }
pub(crate) use BZP_SHELL_SORT_INCREMENT0;


macro_rules! BZP_SHELL_SORT_INCREMENT1 { () => { 4 } }
pub(crate) use BZP_SHELL_SORT_INCREMENT1;


macro_rules! BZP_MTF_ENCODE0 { () => { 0 } }
pub(crate) use BZP_MTF_ENCODE0;


macro_rules! BZP_MTF_ENCODE1 { () => { 1 } }
pub(crate) use BZP_MTF_ENCODE1;


macro_rules! BZP_MTF_ENCODE_BASE { () => { 2 } }
pub(crate) use BZP_MTF_ENCODE_BASE;


macro_rules! BZP_INIT_BLOCK_CRC { () => { 0xffffffff } }
pub(crate) use BZP_INIT_BLOCK_CRC;


macro_rules! BZP_MAX_ALPHA_SIZE { () => { 258 } }
pub(crate) use BZP_MAX_ALPHA_SIZE;


macro_rules! BZP_MAX_GROUPS_NUM { () => { 6 } }
pub(crate) use BZP_MAX_GROUPS_NUM;


macro_rules! BZP_MAX_ITER_NUM { () => { 4 } }
pub(crate) use BZP_MAX_ITER_NUM;


macro_rules! BZP_MAX_TREE_HEIGHT_ENCODE { () => { 17 } }
pub(crate) use BZP_MAX_TREE_HEIGHT_ENCODE;


macro_rules! BZP_NGROUPS_BLOCK_NUM_LIMIT0 { () => { 200 } }
pub(crate) use BZP_NGROUPS_BLOCK_NUM_LIMIT0;


macro_rules! BZP_NGROUPS_BLOCK_NUM_LIMIT1 { () => { 600 } }
pub(crate) use BZP_NGROUPS_BLOCK_NUM_LIMIT1;


macro_rules! BZP_NGROUPS_BLOCK_NUM_LIMIT2 { () => { 1200 } }
pub(crate) use BZP_NGROUPS_BLOCK_NUM_LIMIT2;


macro_rules! BZP_NGROUPS_BLOCK_NUM_LIMIT3 { () => { 2400 } }
pub(crate) use BZP_NGROUPS_BLOCK_NUM_LIMIT3;


macro_rules! BZP_NGROUPS_NUM_0 { () => { 2 } }
pub(crate) use BZP_NGROUPS_NUM_0;


macro_rules! BZP_NGROUPS_NUM_1 { () => { 3 } }
pub(crate) use BZP_NGROUPS_NUM_1;


macro_rules! BZP_NGROUPS_NUM_2 { () => { 4 } }
pub(crate) use BZP_NGROUPS_NUM_2;


macro_rules! BZP_NGROUPS_NUM_3 { () => { 5 } }
pub(crate) use BZP_NGROUPS_NUM_3;


macro_rules! BZP_NGROUPS_NUM_4 { () => { 6 } }
pub(crate) use BZP_NGROUPS_NUM_4;


macro_rules! BZP_ELEMS_NUM_IN_ONE_GROUP { () => { 50 } }
pub(crate) use BZP_ELEMS_NUM_IN_ONE_GROUP;


macro_rules! BZP_HUFFMAN_HEIGHT_WEIGHT_BITS { () => { 8 } }
pub(crate) use BZP_HUFFMAN_HEIGHT_WEIGHT_BITS;


macro_rules! BZP_HUFFMAN_LEN_MAX_COST { () => { 15 } }
pub(crate) use BZP_HUFFMAN_LEN_MAX_COST;


macro_rules! BZP_HUFFMAN_LEN_UPPER_LIMIT { () => { 20 } }
pub(crate) use BZP_HUFFMAN_LEN_UPPER_LIMIT;


macro_rules! BZP_HUFFMAN_MAX_SIZE_SELECT { () => { BZP_BLOCK_SIZE_LEVEL_UPPER_LIMIT!() * BZP_BASE_BLOCK_SIZE!() / BZP_ELEMS_NUM_IN_ONE_GROUP!() } }
pub(crate) use BZP_HUFFMAN_MAX_SIZE_SELECT;


macro_rules! BZP_INVALID_BLOCK_SIZE { ($blockSize:expr) => { ($blockSize < BZP_BLOCK_SIZE_LEVEL_LOWER_LIMIT!()) || ($blockSize > BZP_BLOCK_SIZE_LEVEL_UPPER_LIMIT!()) } }
pub(crate) use BZP_INVALID_BLOCK_SIZE;


macro_rules! BZP_INVALID_ALPHA_SIZE { ($alphaSize:expr) => { $alphaSize > BZP_MAX_ALPHA_SIZE!() || $alphaSize < 1 } }
pub(crate) use BZP_INVALID_ALPHA_SIZE;


macro_rules! BZP_MAX_FUN { ($a:expr, $b:expr) => { if $a > $b { $a } else { $b } } }
pub(crate) use BZP_MAX_FUN;


macro_rules! BZP_MIN_FUN { ($a:expr, $b:expr) => { if $a < $b { $a } else { $b } } }
pub(crate) use BZP_MIN_FUN;


