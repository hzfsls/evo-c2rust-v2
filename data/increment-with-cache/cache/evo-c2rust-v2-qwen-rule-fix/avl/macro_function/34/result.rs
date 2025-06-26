macro_rules! VOS_AVL_PREV { ($node:expr) => { VOS_AVL_Prev(c_ref!($node)) } }
pub(crate) use VOS_AVL_PREV;