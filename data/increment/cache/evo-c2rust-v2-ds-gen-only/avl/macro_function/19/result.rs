macro_rules! VOS_AVL_NEXT { ($NODE:expr) => { VOS_AVL_Next(c_ref!($NODE)) } }
pub(crate) use VOS_AVL_NEXT;
