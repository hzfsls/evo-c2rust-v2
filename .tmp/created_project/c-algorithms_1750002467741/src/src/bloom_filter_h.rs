use crate::translation_utils::*;
pub use crate::src::bloom_filter_c::_BloomFilter;
pub use crate::src::bloom_filter_c::bloom_filter_insert;
pub use crate::src::bloom_filter_c::bloom_filter_intersection;
pub use crate::src::bloom_filter_c::bloom_filter_new;
pub use crate::src::bloom_filter_c::bloom_filter_read;
pub use crate::src::bloom_filter_c::bloom_filter_load;
pub use crate::src::bloom_filter_c::bloom_filter_free;
pub use crate::src::bloom_filter_c::bloom_filter_union;
pub use crate::src::bloom_filter_c::bloom_filter_query;

pub type BloomFilter = i32;

pub type BloomFilterValue = i32;

pub type BloomFilterHashFunc = i32;



