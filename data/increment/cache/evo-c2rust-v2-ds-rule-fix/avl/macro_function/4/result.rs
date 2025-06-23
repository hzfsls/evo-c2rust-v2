macro_rules! VOS_AVLL_DELETE { ($TREE:expr, $NODE:expr) => { VOS_AVL3_DELETE!($TREE.stTree, $NODE) } }
pub(crate) use VOS_AVLL_DELETE;
