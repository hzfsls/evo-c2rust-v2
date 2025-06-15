use crate::translation_utils::*;
pub use crate::src::hash_table_c::_HashTableEntry;
pub use crate::src::hash_table_c::hash_table_lookup;
pub use crate::src::hash_table_c::hash_table_iterate;
pub use crate::src::hash_table_c::hash_table_register_free_functions;
pub use crate::src::hash_table_c::hash_table_remove;
pub use crate::src::hash_table_c::hash_table_iter_has_more;
pub use crate::src::hash_table_c::hash_table_num_entries;
pub use crate::src::hash_table_c::hash_table_insert;
pub use crate::src::hash_table_c::hash_table_new;
pub use crate::src::hash_table_c::hash_table_iter_next;
pub use crate::src::hash_table_c::hash_table_free;
pub use crate::src::hash_table_c::_HashTable;

pub type HashTable = i32;

pub type HashTableIterator = i32;

pub type HashTableEntry = i32;

pub type HashTableKey = i32;

pub type HashTableValue = i32;

pub type HashTablePair = i32;

pub type _HashTableIterator = i32;

pub type HashTableHashFunc = i32;

pub type HashTableEqualFunc = i32;

pub type HashTableKeyFreeFunc = i32;

pub type HashTableValueFreeFunc = i32;





