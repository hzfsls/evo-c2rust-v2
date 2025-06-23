macro_rules! VOS_AVLL_INIT_TREE {
    ($TREE:expr, $COMPARE:expr, $KEY_OFF:expr, $NODE_OFF:expr) => {
        $TREE.stTreeInfo.pfCompare = $COMPARE;
        $TREE.stTreeInfo.usKeyOffset = $KEY_OFF;
        $TREE.stTreeInfo.usNodeOffset = $NODE_OFF;
        VOS_AVL3_INIT_TREE!($TREE.stTree, $TREE.stTreeInfo);
    }
}
pub(crate) use VOS_AVLL_INIT_TREE;
