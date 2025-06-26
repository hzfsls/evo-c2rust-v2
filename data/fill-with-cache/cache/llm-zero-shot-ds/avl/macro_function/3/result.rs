macro_rules! VOS_AVL_DELETE {
    ($TREE:expr, $NODE:expr) => {
        VOS_AVL_Delete(&mut $TREE, &$NODE)
    };
}
pub(crate) use VOS_AVL_DELETE;
