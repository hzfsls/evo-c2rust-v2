pub fn VosAvlDeleteCheck(
    mut pstTree: Ptr<AVLBASE_TREE_S>,
    mut pstNode: Ptr<AVLBASE_NODE_S>,
) -> Ptr<AVLBASE_NODE_S> {
    let mut pstReplaceNode: Ptr<AVLBASE_NODE_S>;
    if pstNode.pstLeft == AVL_NULL_PTR!() && pstNode.pstRight == AVL_NULL_PTR!() {
        pstReplaceNode = AVL_NULL_PTR!();
        if pstTree.pstFirst == pstNode {
            pstTree.pstFirst = pstNode.pstParent;
        }
        if pstTree.pstLast == pstNode {
            pstTree.pstLast = pstNode.pstParent;
        }
    } else if pstNode.pstLeft == AVL_NULL_PTR!() {
        pstReplaceNode = pstNode.pstRight;
        if pstTree.pstFirst == pstNode {
            pstTree.pstFirst = pstReplaceNode;
        }
    } else if pstNode.pstRight == AVL_NULL_PTR!() {
        pstReplaceNode = pstNode.pstLeft;
        if pstTree.pstLast == pstNode {
            pstTree.pstLast = pstReplaceNode;
        }
    } else {
        pstReplaceNode = VosAvlSearchReplaceNode(pstTree, pstNode);
    }
    return pstReplaceNode;
}