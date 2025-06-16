macro_rules! VOS_AVLL_NEXT { ($TREE:expr, $NODE:expr) => { VOS_AVL3_NEXT!($NODE, $TREE.stTreeInfo) } }
pub(crate) use VOS_AVLL_NEXT;
