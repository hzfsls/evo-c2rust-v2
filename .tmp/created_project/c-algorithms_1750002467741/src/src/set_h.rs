use crate::translation_utils::*;
pub use crate::src::set_c::set_remove;
pub use crate::src::set_c::set_intersection;
pub use crate::src::set_c::set_free;
pub use crate::src::set_c::set_insert;
pub use crate::src::set_c::set_iter_next;
pub use crate::src::set_c::set_to_array;
pub use crate::src::set_c::set_new;
pub use crate::src::set_c::set_iter_has_more;
pub use crate::src::set_c::_SetEntry;
pub use crate::src::set_c::set_union;
pub use crate::src::set_c::set_iterate;
pub use crate::src::set_c::set_register_free_function;
pub use crate::src::set_c::set_num_entries;
pub use crate::src::set_c::_Set;
pub use crate::src::set_c::set_query;

pub type Set = i32;

pub type SetIterator = i32;

pub type SetEntry = i32;

pub type SetValue = i32;

pub type _SetIterator = i32;

pub type SetHashFunc = i32;

pub type SetEqualFunc = i32;

pub type SetFreeFunc = i32;





