macro_rules! VOS_AVL_NEXT { ($node:expr) => { VOS_AVL_Next(c_ref!($node)) } }
pub(crate) use VOS_AVL_NEXT;