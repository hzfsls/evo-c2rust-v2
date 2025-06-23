macro_rules! VOS_AVL3_FIND { ($TREE:expr, $KEY:expr, $TREE_INFO:expr) => {
    VOS_AVL3_Find($TREE.cast::<Ptr<_>>(), $KEY.cast::<Ptr<_>>(), $TREE_INFO.cast::<Ptr<_>>())
} }
pub(crate) use VOS_AVL3_FIND;