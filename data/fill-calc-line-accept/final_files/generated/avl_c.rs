use crate::translation_utils::*;

pub type AVL_V2_COMPARE_FUNC = FuncPtr<fn(VoidPtr, VoidPtr) -> i32>;

#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct avl_node {
    pub pstParent: Ptr<avl_node>,
    pub pstLeft: Ptr<avl_node>,
    pub pstRight: Ptr<avl_node>,
    pub sLHeight: i16,
    pub sRHeight: i16,
    pub pSelf: VoidPtr,
    pub pKey: VoidPtr,
}

pub type AVL_NODE = avl_node;

#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct avl_tree {
    pub pfnCompare: AVL_V2_COMPARE_FUNC,
    pub pstRoot: Ptr<AVL_NODE>,
    pub pstFirst: Ptr<AVL_NODE>,
    pub pstLast: Ptr<AVL_NODE>,
}

pub type AVL_TREE = avl_tree;

pub type AVL3_COMPARE = FuncPtr<fn(VoidPtr, VoidPtr) -> i32>;

#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct avl3_tree_info {
    pub pfCompare: AVL3_COMPARE,
    pub usKeyOffset: u16,
    pub usNodeOffset: u16,
}

pub type AVL3_TREE_INFO = avl3_tree_info;

#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct avl3_tree {
    pub pstRoot: Ptr<AVL3_NODE>,
    pub pstFirst: Ptr<AVL3_NODE>,
    pub pstLast: Ptr<AVL3_NODE>,
}

pub type AVL3_TREE = avl3_tree;

#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct avl3_node {
    pub pstParent: Ptr<avl3_node>,
    pub pstLeft: Ptr<avl3_node>,
    pub pstRight: Ptr<avl3_node>,
    pub sLHeight: i16,
    pub sRHeight: i16,
}

pub type AVL3_NODE = avl3_node;

#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct AVLBaseNode {
    pub pstParent: Ptr<AVLBaseNode>,
    pub pstLeft: Ptr<AVLBaseNode>,
    pub pstRight: Ptr<AVLBaseNode>,
    pub sLHeight: i16,
    pub sRHeight: i16,
}

pub type AVLBASE_NODE_S = AVLBaseNode;

#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct AVLBaseTree {
    pub pstRoot: Ptr<AVLBASE_NODE_S>,
    pub pstFirst: Ptr<AVLBASE_NODE_S>,
    pub pstLast: Ptr<AVLBASE_NODE_S>,
}

pub type AVLBASE_TREE_S = AVLBaseTree;

macro_rules! AVL_NULL_PTR {
    () => {
        NULL!()
    };
}
pub(crate) use AVL_NULL_PTR;

macro_rules! AVL_TRUE {
    () => {
        1
    };
}
pub(crate) use AVL_TRUE;

macro_rules! AVL_FALSE {
    () => {
        0
    };
}
pub(crate) use AVL_FALSE;

macro_rules! VOS_AVL_INIT_TREE {
    ($TREE:expr, $COMPARE:expr) => {
        $TREE.pfnCompare = $COMPARE;
        $TREE.pstFirst = AVL_NULL_PTR!().cast::<Ptr<AVL_NODE>>();
        $TREE.pstLast = AVL_NULL_PTR!().cast::<Ptr<AVL_NODE>>();
        $TREE.pstRoot = AVL_NULL_PTR!().cast::<Ptr<AVL_NODE>>();
    };
}
pub(crate) use VOS_AVL_INIT_TREE;

macro_rules! VOS_AVL_INIT_NODE {
    ($NODE:expr, $SELF:expr, $KEY:expr) => {
        $NODE.pstParent = AVL_NULL_PTR!().cast::<Ptr<AVL_NODE>>();
        $NODE.pstLeft = AVL_NULL_PTR!().cast::<Ptr<AVL_NODE>>();
        $NODE.pstRight = AVL_NULL_PTR!().cast::<Ptr<AVL_NODE>>();
        $NODE.pSelf = $SELF;
        $NODE.pKey = $KEY;
        $NODE.sLHeight = -1;
        $NODE.sRHeight = -1;
    };
}
pub(crate) use VOS_AVL_INIT_NODE;

macro_rules! VOS_AVL_INSERT {
    ($TREE:expr, $NODE:expr) => {
        VOS_AVL_Insert_Or_Find(c_ref!($TREE), c_ref!($NODE)) == AVL_NULL_PTR!()
    };
}
pub(crate) use VOS_AVL_INSERT;

macro_rules! VOS_AVL_DELETE {
    ($TREE:expr, $NODE:expr) => {
        VOS_AVL_Delete(c_ref!($TREE), c_ref!($NODE))
    };
}
pub(crate) use VOS_AVL_DELETE;

macro_rules! VOS_AVL_FIND {
    ($TREE:expr, $KEY:expr) => {
        VOS_AVL_Find(c_ref!($TREE), $KEY)
    };
}
pub(crate) use VOS_AVL_FIND;

macro_rules! VOS_AVL_IN_TREE {
    ($NODE:expr) => {
        ($NODE.sLHeight != -1) && ($NODE.sRHeight != -1)
    };
}
pub(crate) use VOS_AVL_IN_TREE;

macro_rules! VOS_AVL_FIND_OR_FIND_NEXT {
    ($TREE:expr, $KEY:expr) => {
        VOS_AVL_Find_Or_Find_Next(c_ref!($TREE), $KEY, AVL_FALSE!())
    };
}
pub(crate) use VOS_AVL_FIND_OR_FIND_NEXT;

macro_rules! VOS_V2_AVL_MAX {
    ($X:expr, $Y:expr) => {
        if $X > $Y {
            $X
        } else {
            $Y
        }
    };
}
pub(crate) use VOS_V2_AVL_MAX;

macro_rules! VOS_AVL3_INIT_TREE {
    ($TREE:expr, $TREE_INFO:expr) => {
        $TREE.pstFirst = AVL_NULL_PTR!().cast::<Ptr<AVL3_NODE>>();
        $TREE.pstLast = AVL_NULL_PTR!().cast::<Ptr<AVL3_NODE>>();
        $TREE.pstRoot = AVL_NULL_PTR!().cast::<Ptr<AVL3_NODE>>();
    };
}
pub(crate) use VOS_AVL3_INIT_TREE;

macro_rules! VOS_AVL3_INIT_NODE {
    ($NODE:expr) => {
        $NODE.pstParent = AVL_NULL_PTR!().cast::<Ptr<AVL3_NODE>>();
        $NODE.pstLeft = AVL_NULL_PTR!().cast::<Ptr<AVL3_NODE>>();
        $NODE.pstRight = AVL_NULL_PTR!().cast::<Ptr<AVL3_NODE>>();
        $NODE.sLHeight = -1;
        $NODE.sRHeight = -1;
    };
}
pub(crate) use VOS_AVL3_INIT_NODE;

macro_rules! VOS_AVL3_INSERT {
    ($TREE:expr, $NODE:expr, $TREE_INFO:expr) => {
        AVL_NULL_PTR!() == VOS_AVL3_Insert_Or_Find(c_ref!($TREE), c_ref!($NODE), c_ref!($TREE_INFO))
    };
}
pub(crate) use VOS_AVL3_INSERT;

macro_rules! VOS_AVL3_INSERT_OR_FIND {
    ($TREE:expr, $NODE:expr, $TREE_INFO:expr) => {
        VOS_AVL3_Insert_Or_Find(c_ref!($TREE), c_ref!($NODE), c_ref!($TREE_INFO))
    };
}
pub(crate) use VOS_AVL3_INSERT_OR_FIND;

macro_rules! VOS_AVL3_DELETE {
    ($TREE:expr, $NODE:expr) => {
        VOS_AVL3_Delete(c_ref!($TREE), c_ref!($NODE))
    };
}
pub(crate) use VOS_AVL3_DELETE;

macro_rules! VOS_AVL3_FIND {
    ($TREE:expr, $KEY:expr, $TREE_INFO:expr) => {
        VOS_AVL3_Find(c_ref!($TREE), $KEY.cast(), c_ref!($TREE_INFO))
    };
}
pub(crate) use VOS_AVL3_FIND;

macro_rules! VOS_AVL3_NEXT {
    ($NODE:expr, $TREE_INFO:expr) => {
        VOS_AVL3_Next(c_ref!($NODE), c_ref!($TREE_INFO))
    };
}
pub(crate) use VOS_AVL3_NEXT;

macro_rules! VOS_AVL3_PREV {
    ($NODE:expr, $TREE_INFO:expr) => {
        VOS_AVL3_Prev(c_ref!($NODE), c_ref!($TREE_INFO))
    };
}
pub(crate) use VOS_AVL3_PREV;

macro_rules! VOS_AVL3_FIRST {
    ($TREE:expr, $TREE_INFO:expr) => {
        VOS_AVL3_First(c_ref!($TREE), c_ref!($TREE_INFO))
    };
}
pub(crate) use VOS_AVL3_FIRST;

macro_rules! VOS_AVL3_LAST {
    ($TREE:expr, $TREE_INFO:expr) => {
        VOS_AVL3_Last(c_ref!($TREE), c_ref!($TREE_INFO))
    };
}
pub(crate) use VOS_AVL3_LAST;

macro_rules! VOS_AVL3_IN_TREE {
    ($NODE:expr) => {
        ($NODE.sLHeight != -1) && ($NODE.sRHeight != -1)
    };
}
pub(crate) use VOS_AVL3_IN_TREE;

macro_rules! VOS_AVL3_FIND_NEXT {
    ($TREE:expr, $KEY:expr, $TREE_INFO:expr) => {
        AVL3_Find_Or_Find_Next(c_ref!($TREE), $KEY, AVL_TRUE!(), c_ref!($TREE_INFO))
    };
}
pub(crate) use VOS_AVL3_FIND_NEXT;

macro_rules! VOS_AVL3_FIND_OR_FIND_NEXT {
    ($TREE:expr, $KEY:expr, $TREE_INFO:expr) => {
        AVL3_Find_Or_Find_Next(c_ref!($TREE), $KEY, AVL_FALSE!(), c_ref!($TREE_INFO))
    };
}
pub(crate) use VOS_AVL3_FIND_OR_FIND_NEXT;

macro_rules! TREE_OR_TREEINFO_IS_NULL {
    ($pstTree:expr, $pstTreeInfo:expr) => {
        ($pstTree == AVL_NULL_PTR!()) || ($pstTreeInfo == AVL_NULL_PTR!())
    };
}
pub(crate) use TREE_OR_TREEINFO_IS_NULL;

macro_rules! GET_NODE_START_ADDRESS {
    ($pstNode:expr, $usOffset:expr) => {
        if $pstNode != AVL_NULL_PTR!() {
            ($pstNode.cast::<Ptr<u8>>() - $usOffset).cast::<VoidPtr>()
        } else {
            AVL_NULL_PTR!()
        }
    };
}
pub(crate) use GET_NODE_START_ADDRESS;

macro_rules! GET_KEYOFFSET {
    ($pstTreeInfo:expr) => {
        ($pstTreeInfo.usKeyOffset - $pstTreeInfo.usNodeOffset) as i32
    };
}
pub(crate) use GET_KEYOFFSET;

macro_rules! FIND_LEFTMOST_NODE {
    ($pstNode:expr) => {
        while $pstNode.pstLeft != AVL_NULL_PTR!() {
            $pstNode = $pstNode.pstLeft;
        }
    };
}
pub(crate) use FIND_LEFTMOST_NODE;

macro_rules! FIND_RIGHTMOST_NODE {
    ($pstNode:expr) => {
        while $pstNode.pstRight != AVL_NULL_PTR!() {
            $pstNode = $pstNode.pstRight;
        }
    };
}
pub(crate) use FIND_RIGHTMOST_NODE;

pub fn AVL3_Find_Or_Find_Next(
    mut pstTree: Ptr<AVL3_TREE>,
    mut pKey: Ptr<Void>,
    mut bFlag: u32,
    mut pstTreeInfo: Ptr<AVL3_TREE_INFO>,
) -> Ptr<Void> {
    let mut pstNode: Ptr<AVL3_NODE> = Default::default();
    let mut pFoundNode: Ptr<Void> = AVL_NULL_PTR!();
    let mut iResult: i32 = Default::default();
    let mut iKeyOffset: i32 = Default::default();
    if TREE_OR_TREEINFO_IS_NULL!(pstTree, pstTreeInfo).as_bool() {
        return AVL_NULL_PTR!();
    }
    pstNode = pstTree.pstRoot.cast();
    if (pstNode == AVL_NULL_PTR!()).as_bool() {
        return AVL_NULL_PTR!();
    }
    iKeyOffset = GET_KEYOFFSET!(pstTreeInfo).cast();
    loop {
        iResult = (pstTreeInfo.pfCompare)(
            pKey.cast(),
            (pstNode.cast::<Ptr<u8>>() + iKeyOffset).cast::<Ptr<Void>>(),
        )
        .cast();
        if iResult > 0 {
            if (pstNode.pstRight == AVL_NULL_PTR!()).as_bool() {
                pFoundNode = VOS_AVL3_Next(pstNode.cast(), pstTreeInfo.cast()).cast();
                break;
            }
            pstNode = pstNode.pstRight.cast();
        } else if iResult < 0 {
            if (pstNode.pstLeft == AVL_NULL_PTR!()).as_bool() {
                pFoundNode =
                    (pstNode.cast::<Ptr<u8>>() - pstTreeInfo.usNodeOffset).cast::<Ptr<Void>>();
                break;
            }
            pstNode = pstNode.pstLeft.cast();
        } else {
            if (bFlag != 0).as_bool() {
                pFoundNode = VOS_AVL3_Next(pstNode.cast(), pstTreeInfo.cast()).cast();
            } else {
                pFoundNode =
                    (pstNode.cast::<Ptr<u8>>() - pstTreeInfo.usNodeOffset).cast::<Ptr<Void>>();
            }
            break;
        }
    }
    return pFoundNode.cast();
}

pub fn VOS_AVL3_Insert_Or_Find(
    mut pstTree: Ptr<AVL3_TREE>,
    mut pstNode: Ptr<AVL3_NODE>,
    mut pstTreeInfo: Ptr<AVL3_TREE_INFO>,
) -> Ptr<Void> {
    let mut pstParentNode: Ptr<AVL3_NODE> = Default::default();
    let mut iResult: i32 = Default::default();
    let mut iKeyOffset: i32 = Default::default();
    if TREE_OR_TREEINFO_IS_NULL!(pstTree, pstTreeInfo).as_bool()
        || (pstNode == AVL_NULL_PTR!()).as_bool()
    {
        return AVL_NULL_PTR!();
    }
    pstNode.sRHeight = 0;
    pstNode.sLHeight = 0;
    if (pstTree.pstRoot == AVL_NULL_PTR!()).as_bool() {
        pstTree.pstRoot = pstNode.cast();
        pstTree.pstFirst = pstNode.cast();
        pstTree.pstLast = pstNode.cast();
        return AVL_NULL_PTR!();
    }
    pstParentNode = pstTree.pstRoot.cast();
    iKeyOffset = GET_KEYOFFSET!(pstTreeInfo).cast();
    while (pstParentNode != AVL_NULL_PTR!()).as_bool() {
        iResult = (pstTreeInfo.pfCompare)(
            (pstNode.cast::<Ptr<u8>>() + iKeyOffset).cast::<Ptr<Void>>(),
            (pstParentNode.cast::<Ptr<u8>>() + iKeyOffset).cast::<Ptr<Void>>(),
        )
        .cast();
        if iResult > 0 {
            if (pstParentNode.pstRight != AVL_NULL_PTR!()).as_bool() {
                pstParentNode = pstParentNode.pstRight.cast();
                continue;
            }
            VosAvlNodeRightInsert(
                pstTree.cast::<Ptr<AVLBASE_TREE_S>>(),
                pstParentNode.cast::<Ptr<AVLBASE_NODE_S>>(),
                pstNode.cast::<Ptr<AVLBASE_NODE_S>>(),
            );
        } else if iResult < 0 {
            if (pstParentNode.pstLeft != AVL_NULL_PTR!()).as_bool() {
                pstParentNode = pstParentNode.pstLeft.cast();
                continue;
            }
            VosAvlNodeLeftInsert(
                pstTree.cast::<Ptr<AVLBASE_TREE_S>>(),
                pstParentNode.cast::<Ptr<AVLBASE_NODE_S>>(),
                pstNode.cast::<Ptr<AVLBASE_NODE_S>>(),
            );
        } else {
            pstNode.sRHeight = -1;
            pstNode.sLHeight = -1;
            return (pstParentNode.cast::<Ptr<u8>>() - pstTreeInfo.usNodeOffset)
                .cast::<Ptr<Void>>();
        }
        break;
    }
    VosAvlBalanceTree(
        pstTree.cast::<Ptr<AVLBASE_TREE_S>>(),
        pstParentNode.cast::<Ptr<AVLBASE_NODE_S>>(),
    );
    return AVL_NULL_PTR!();
}

pub fn VOS_AVL3_Delete(mut pstTree: Ptr<AVL3_TREE>, mut pstNode: Ptr<AVL3_NODE>) {
    let mut pstBaseNode: Ptr<AVLBASE_NODE_S> = Default::default();
    let mut pstBaseTree: Ptr<AVLBASE_TREE_S> = Default::default();
    if (pstTree == AVL_NULL_PTR!()).as_bool() || (pstNode == AVL_NULL_PTR!()).as_bool() {
        return;
    }
    pstBaseNode = pstNode.cast::<Ptr<AVLBASE_NODE_S>>();
    pstBaseTree = pstTree.cast::<Ptr<AVLBASE_TREE_S>>();
    VosAvlDelete(pstBaseNode.cast(), pstBaseTree.cast());
}

pub fn VOS_AVL3_Find(
    mut pstTree: Ptr<AVL3_TREE>,
    mut pstKey: Ptr<Void>,
    mut pstTreeInfo: Ptr<AVL3_TREE_INFO>,
) -> Ptr<Void> {
    let mut pstNode: Ptr<AVL3_NODE> = Default::default();
    let mut iResult: i32 = Default::default();
    let mut iKeyOffset: i32 = Default::default();
    if TREE_OR_TREEINFO_IS_NULL!(pstTree, pstTreeInfo).as_bool() {
        return AVL_NULL_PTR!();
    }
    pstNode = pstTree.pstRoot.cast();
    iKeyOffset = GET_KEYOFFSET!(pstTreeInfo).cast();
    while (pstNode != AVL_NULL_PTR!()).as_bool() {
        iResult = (pstTreeInfo.pfCompare)(
            pstKey.cast(),
            (pstNode.cast::<Ptr<u8>>() + iKeyOffset).cast::<Ptr<Void>>(),
        )
        .cast();
        if iResult > 0 {
            pstNode = pstNode.pstRight.cast();
        } else if iResult < 0 {
            pstNode = pstNode.pstLeft.cast();
        } else {
            break;
        }
    }
    return GET_NODE_START_ADDRESS!(pstNode, pstTreeInfo.usNodeOffset);
}

pub fn VOS_AVL3_First(
    mut pstTree: Ptr<AVL3_TREE>,
    mut pstTreeInfo: Ptr<AVL3_TREE_INFO>,
) -> Ptr<Void> {
    let mut pstNode: Ptr<AVL3_NODE> = Default::default();
    if TREE_OR_TREEINFO_IS_NULL!(pstTree, pstTreeInfo).as_bool() {
        return AVL_NULL_PTR!();
    }
    pstNode = pstTree.pstFirst.cast();
    return GET_NODE_START_ADDRESS!(pstNode, pstTreeInfo.usNodeOffset);
}

pub fn VOS_AVL3_Last(
    mut pstTree: Ptr<AVL3_TREE>,
    mut pstTreeInfo: Ptr<AVL3_TREE_INFO>,
) -> Ptr<Void> {
    let mut pstNode: Ptr<AVL3_NODE> = Default::default();
    if TREE_OR_TREEINFO_IS_NULL!(pstTree, pstTreeInfo).as_bool() {
        return AVL_NULL_PTR!();
    }
    pstNode = pstTree.pstLast.cast();
    return GET_NODE_START_ADDRESS!(pstNode, pstTreeInfo.usNodeOffset);
}

pub fn VOS_AVL3_Next(
    mut pstNode: Ptr<AVL3_NODE>,
    mut pstTreeInfo: Ptr<AVL3_TREE_INFO>,
) -> Ptr<Void> {
    let mut pstNodeTmp: Ptr<AVL3_NODE> = pstNode.cast();
    if (pstNodeTmp == AVL_NULL_PTR!()).as_bool() || (pstTreeInfo == AVL_NULL_PTR!()).as_bool() {
        return AVL_NULL_PTR!();
    }
    if (pstNodeTmp.pstRight != AVL_NULL_PTR!()).as_bool() {
        pstNodeTmp = pstNodeTmp.pstRight.cast();
        FIND_LEFTMOST_NODE!(pstNodeTmp);
    } else {
        while (pstNodeTmp != AVL_NULL_PTR!()).as_bool() {
            if (pstNodeTmp.pstParent == AVL_NULL_PTR!()).as_bool()
                || (pstNodeTmp.pstParent.pstLeft == pstNodeTmp).as_bool()
            {
                pstNodeTmp = pstNodeTmp.pstParent.cast();
                break;
            }
            pstNodeTmp = pstNodeTmp.pstParent.cast();
        }
    }
    return GET_NODE_START_ADDRESS!(pstNodeTmp, pstTreeInfo.usNodeOffset);
}

pub fn VOS_AVL3_Prev(
    mut pstNode: Ptr<AVL3_NODE>,
    mut pstTreeInfo: Ptr<AVL3_TREE_INFO>,
) -> Ptr<Void> {
    let mut pstNodeTmp: Ptr<AVL3_NODE> = pstNode.cast();
    if (pstNodeTmp == AVL_NULL_PTR!()).as_bool() || (pstTreeInfo == AVL_NULL_PTR!()).as_bool() {
        return AVL_NULL_PTR!();
    }
    if (pstNodeTmp.pstLeft != AVL_NULL_PTR!()).as_bool() {
        pstNodeTmp = pstNodeTmp.pstLeft.cast();
        FIND_RIGHTMOST_NODE!(pstNodeTmp);
    } else {
        while (pstNodeTmp != AVL_NULL_PTR!()).as_bool() {
            if (pstNodeTmp.pstParent == AVL_NULL_PTR!()).as_bool()
                || (pstNodeTmp.pstParent.pstRight == pstNodeTmp).as_bool()
            {
                pstNodeTmp = pstNodeTmp.pstParent.cast();
                break;
            }
            pstNodeTmp = pstNodeTmp.pstParent.cast();
        }
    }
    return GET_NODE_START_ADDRESS!(pstNodeTmp, pstTreeInfo.usNodeOffset);
}

pub fn VosAvlNodeRightInsert(
    mut pstTree: Ptr<AVLBASE_TREE_S>,
    mut pstParentNode: Ptr<AVLBASE_NODE_S>,
    mut pstNode: Ptr<AVLBASE_NODE_S>,
) {
    pstNode.pstParent = pstParentNode.cast();
    pstParentNode.pstRight = pstNode.cast();
    pstParentNode.sRHeight = 1;
    if (pstParentNode == pstTree.pstLast).as_bool() {
        pstTree.pstLast = pstNode.cast();
    }
}

pub fn VosAvlNodeLeftInsert(
    mut pstTree: Ptr<AVLBASE_TREE_S>,
    mut pstParentNode: Ptr<AVLBASE_NODE_S>,
    mut pstNode: Ptr<AVLBASE_NODE_S>,
) {
    pstNode.pstParent = pstParentNode.cast();
    pstParentNode.pstLeft = pstNode.cast();
    pstParentNode.sLHeight = 1;
    if (pstParentNode == pstTree.pstFirst).as_bool() {
        pstTree.pstFirst = pstNode.cast();
    }
}

pub fn VosAvlRotateRight(mut ppstSubTree: Ptr<Ptr<AVLBASE_NODE_S>>) {
    let mut pstLeftSon: Ptr<AVLBASE_NODE_S> = (*ppstSubTree).pstLeft.cast();
    (*ppstSubTree).pstLeft = pstLeftSon.pstRight.cast();
    if ((*ppstSubTree).pstLeft != AVL_NULL_PTR!()).as_bool() {
        (*ppstSubTree).pstLeft.pstParent = (*ppstSubTree).cast();
    }
    (*ppstSubTree).sLHeight = pstLeftSon.sRHeight.cast();
    pstLeftSon.pstParent = (*ppstSubTree).pstParent.cast();
    pstLeftSon.pstRight = (*ppstSubTree).cast();
    pstLeftSon.pstRight.pstParent = pstLeftSon.cast();
    pstLeftSon.sRHeight =
        (1 + VOS_V2_AVL_MAX!((*ppstSubTree).sRHeight, (*ppstSubTree).sLHeight)).cast();
    *ppstSubTree = pstLeftSon.cast();
    return;
}

pub fn VosAvlRotateLeft(mut ppstSubTree: Ptr<Ptr<AVLBASE_NODE_S>>) {
    let mut pstRightSon: Ptr<AVLBASE_NODE_S> = (*ppstSubTree).pstRight.cast();
    (*ppstSubTree).pstRight = pstRightSon.pstLeft.cast();
    if ((*ppstSubTree).pstRight != AVL_NULL_PTR!()).as_bool() {
        (*ppstSubTree).pstRight.pstParent = (*ppstSubTree).cast();
    }
    (*ppstSubTree).sRHeight = pstRightSon.sLHeight.cast();
    pstRightSon.pstParent = (*ppstSubTree).pstParent.cast();
    pstRightSon.pstLeft = (*ppstSubTree).cast();
    pstRightSon.pstLeft.pstParent = pstRightSon.cast();
    pstRightSon.sLHeight =
        (1 + VOS_V2_AVL_MAX!((*ppstSubTree).sRHeight, (*ppstSubTree).sLHeight)).cast();
    *ppstSubTree = pstRightSon.cast();
    return;
}

pub fn VosAvlUpdateSwapNode(
    mut pstTree: Ptr<AVLBASE_TREE_S>,
    mut pstSwapNode: Ptr<AVLBASE_NODE_S>,
    mut pstBaseNode: Ptr<AVLBASE_NODE_S>,
) {
    pstSwapNode.pstParent = pstBaseNode.pstParent.cast();
    pstSwapNode.pstRight = pstBaseNode.pstRight.cast();
    pstSwapNode.pstLeft = pstBaseNode.pstLeft.cast();
    pstSwapNode.sRHeight = pstBaseNode.sRHeight.cast();
    pstSwapNode.sLHeight = pstBaseNode.sLHeight.cast();
    pstSwapNode.pstRight.pstParent = pstSwapNode.cast();
    pstSwapNode.pstLeft.pstParent = pstSwapNode.cast();
    if (pstBaseNode.pstParent == AVL_NULL_PTR!()).as_bool() {
        pstTree.pstRoot = pstSwapNode.cast();
    } else if (pstBaseNode.pstParent.pstRight == pstBaseNode).as_bool() {
        pstSwapNode.pstParent.pstRight = pstSwapNode.cast();
    } else {
        pstSwapNode.pstParent.pstLeft = pstSwapNode.cast();
    }
}

pub fn VosAvlMoveNodeToNewPos(
    mut pstNode: Ptr<AVLBASE_NODE_S>,
    mut pstNewParent: Ptr<AVLBASE_NODE_S>,
    mut pstNewLeftSon: Ptr<AVLBASE_NODE_S>,
    mut pstNewRightSon: Ptr<AVLBASE_NODE_S>,
) {
    pstNode.pstParent = pstNewParent.cast();
    pstNode.pstLeft = pstNewLeftSon.cast();
    pstNode.pstRight = pstNewRightSon.cast();
    pstNode.sLHeight = 0;
    pstNode.sRHeight = 0;
    if (pstNewLeftSon != AVL_NULL_PTR!()).as_bool() {
        pstNode.pstLeft.pstParent = pstNode.cast();
        pstNode.sLHeight = 1;
    }
    if (pstNewRightSon != AVL_NULL_PTR!()).as_bool() {
        pstNode.pstRight.pstParent = pstNode.cast();
        pstNode.sRHeight = 1;
    }
}

pub fn VosAvlSwapRightMost(
    mut pstTree: Ptr<AVLBASE_TREE_S>,
    mut pstSubTree: Ptr<AVLBASE_NODE_S>,
    mut pstNode: Ptr<AVLBASE_NODE_S>,
) {
    let mut pstSwapNode: Ptr<AVLBASE_NODE_S> = pstSubTree.cast();
    let mut pstSwapParent: Ptr<AVLBASE_NODE_S> = Default::default();
    let mut pstSwapLeft: Ptr<AVLBASE_NODE_S> = Default::default();
    FIND_RIGHTMOST_NODE!(pstSwapNode);
    if (pstSwapNode.sRHeight != 0).as_bool() || (pstSwapNode.sLHeight > 1).as_bool() {
        return;
    }
    pstSwapParent = pstSwapNode.pstParent.cast();
    pstSwapLeft = pstSwapNode.pstLeft.cast();
    VosAvlUpdateSwapNode(pstTree.cast(), pstSwapNode.cast(), pstNode.cast());
    VosAvlMoveNodeToNewPos(
        pstNode.cast(),
        pstSwapParent.cast(),
        pstSwapLeft.cast(),
        AVL_NULL_PTR!(),
    );
    pstNode.pstParent.pstRight = pstNode.cast();
    return;
}

pub fn VosAvlSwapLeftMost(
    mut pstTree: Ptr<AVLBASE_TREE_S>,
    mut pstSubTree: Ptr<AVLBASE_NODE_S>,
    mut pstNode: Ptr<AVLBASE_NODE_S>,
) {
    let mut pstSwapNode: Ptr<AVLBASE_NODE_S> = pstSubTree.cast();
    let mut pstSwapParent: Ptr<AVLBASE_NODE_S> = Default::default();
    let mut pstSwapRight: Ptr<AVLBASE_NODE_S> = Default::default();
    FIND_LEFTMOST_NODE!(pstSwapNode);
    if (pstSwapNode.sLHeight != 0).as_bool() || (pstSwapNode.sRHeight > 1).as_bool() {
        return;
    }
    pstSwapParent = pstSwapNode.pstParent.cast();
    pstSwapRight = pstSwapNode.pstRight.cast();
    VosAvlUpdateSwapNode(pstTree.cast(), pstSwapNode.cast(), pstNode.cast());
    VosAvlMoveNodeToNewPos(
        pstNode.cast(),
        pstSwapParent.cast(),
        AVL_NULL_PTR!(),
        pstSwapRight.cast(),
    );
    pstNode.pstParent.pstLeft = pstNode.cast();
    return;
}

pub fn VosAvlRebalance(mut ppstSubTree: Ptr<Ptr<AVLBASE_NODE_S>>) {
    let mut iMoment: i32;
    iMoment = ((*ppstSubTree).sRHeight - (*ppstSubTree).sLHeight).cast();
    if (iMoment > 1).as_bool() {
        if ((*ppstSubTree).pstRight.sLHeight > (*ppstSubTree).pstRight.sRHeight).as_bool() {
            VosAvlRotateRight(c_ref!((*ppstSubTree).pstRight).cast());
        }
        VosAvlRotateLeft(ppstSubTree.cast());
    } else if (iMoment < -1).as_bool() {
        if ((*ppstSubTree).pstLeft.sRHeight > (*ppstSubTree).pstLeft.sLHeight).as_bool() {
            VosAvlRotateLeft(c_ref!((*ppstSubTree).pstLeft).cast());
        }
        VosAvlRotateRight(ppstSubTree.cast());
    }
    return;
}

pub fn VosAvlBalanceTree(mut pstTree: Ptr<AVLBASE_TREE_S>, mut pstNode: Ptr<AVLBASE_NODE_S>) {
    let mut pstNodeTmp: Ptr<AVLBASE_NODE_S> = pstNode.cast();
    while (pstNodeTmp.pstParent != AVL_NULL_PTR!()).as_bool() {
        if (pstNodeTmp.pstParent.pstRight == pstNodeTmp).as_bool() {
            pstNodeTmp = pstNodeTmp.pstParent.cast();
            VosAvlRebalance(c_ref!(pstNodeTmp.pstRight).cast());
            pstNodeTmp.sRHeight =
                (1 + VOS_V2_AVL_MAX!(pstNodeTmp.pstRight.sRHeight, pstNodeTmp.pstRight.sLHeight))
                    .cast();
        } else {
            pstNodeTmp = pstNodeTmp.pstParent.cast();
            VosAvlRebalance(c_ref!(pstNodeTmp.pstLeft).cast());
            pstNodeTmp.sLHeight =
                (1 + VOS_V2_AVL_MAX!(pstNodeTmp.pstLeft.sRHeight, pstNodeTmp.pstLeft.sLHeight))
                    .cast();
        }
    }
    if (pstNodeTmp.sLHeight != pstNodeTmp.sRHeight).as_bool() {
        VosAvlRebalance(c_ref!(pstTree.pstRoot).cast());
    }
    return;
}

pub fn VosAVLSearchReplaceNodeInRTree(
    mut pstTree: Ptr<AVLBASE_TREE_S>,
    mut pstNode: Ptr<AVLBASE_NODE_S>,
) -> Ptr<AVLBASE_NODE_S> {
    let mut pstReplaceNode: Ptr<AVLBASE_NODE_S> = Default::default();
    if (pstNode.pstRight.pstLeft == AVL_NULL_PTR!()).as_bool() {
        pstReplaceNode = pstNode.pstRight.cast();
        pstReplaceNode.pstLeft = pstNode.pstLeft.cast();
        pstReplaceNode.pstLeft.pstParent = pstReplaceNode.cast();
        pstReplaceNode.sLHeight = pstNode.sLHeight.cast();
    } else {
        VosAvlSwapLeftMost(pstTree.cast(), pstNode.pstRight.cast(), pstNode.cast());
        pstReplaceNode = pstNode.pstRight.cast();
    }
    return pstReplaceNode.cast();
}

pub fn VosAvlSearchReplaceNodeInLTree(
    mut pstTree: Ptr<AVLBASE_TREE_S>,
    mut pstNode: Ptr<AVLBASE_NODE_S>,
) -> Ptr<AVLBASE_NODE_S> {
    let mut pstReplaceNode: Ptr<AVLBASE_NODE_S> = Default::default();
    if (pstNode.pstLeft.pstRight == AVL_NULL_PTR!()).as_bool() {
        pstReplaceNode = pstNode.pstLeft.cast();
        pstReplaceNode.pstRight = pstNode.pstRight.cast();
        pstReplaceNode.pstRight.pstParent = pstReplaceNode.cast();
        pstReplaceNode.sRHeight = pstNode.sRHeight.cast();
    } else {
        VosAvlSwapRightMost(pstTree.cast(), pstNode.pstLeft.cast(), pstNode.cast());
        pstReplaceNode = pstNode.pstLeft.cast();
    }
    return pstReplaceNode.cast();
}

pub fn VosAvlSearchReplaceNode(
    mut pstTree: Ptr<AVLBASE_TREE_S>,
    mut pstNode: Ptr<AVLBASE_NODE_S>,
) -> Ptr<AVLBASE_NODE_S> {
    let mut pstReplaceNode: Ptr<AVLBASE_NODE_S> = Default::default();
    if (pstNode.sRHeight > pstNode.sLHeight).as_bool() {
        pstReplaceNode = VosAVLSearchReplaceNodeInRTree(pstTree.cast(), pstNode.cast()).cast();
    } else {
        pstReplaceNode = VosAvlSearchReplaceNodeInLTree(pstTree.cast(), pstNode.cast()).cast();
    }
    return pstReplaceNode.cast();
}

pub fn VosAvlDeleteCheck(
    mut pstTree: Ptr<AVLBASE_TREE_S>,
    mut pstNode: Ptr<AVLBASE_NODE_S>,
) -> Ptr<AVLBASE_NODE_S> {
    let mut pstReplaceNode: Ptr<AVLBASE_NODE_S> = Default::default();
    if (pstNode.pstLeft == AVL_NULL_PTR!()).as_bool()
        && (pstNode.pstRight == AVL_NULL_PTR!()).as_bool()
    {
        pstReplaceNode = AVL_NULL_PTR!();
        if (pstTree.pstFirst == pstNode).as_bool() {
            pstTree.pstFirst = pstNode.pstParent.cast();
        }
        if (pstTree.pstLast == pstNode).as_bool() {
            pstTree.pstLast = pstNode.pstParent.cast();
        }
    } else if (pstNode.pstLeft == AVL_NULL_PTR!()).as_bool() {
        pstReplaceNode = pstNode.pstRight.cast();
        if (pstTree.pstFirst == pstNode).as_bool() {
            pstTree.pstFirst = pstReplaceNode.cast();
        }
    } else if (pstNode.pstRight == AVL_NULL_PTR!()).as_bool() {
        pstReplaceNode = pstNode.pstLeft.cast();
        if (pstTree.pstLast == pstNode).as_bool() {
            pstTree.pstLast = pstReplaceNode.cast();
        }
    } else {
        pstReplaceNode = VosAvlSearchReplaceNode(pstTree.cast(), pstNode.cast()).cast();
    }
    return pstReplaceNode.cast();
}

pub fn VosAvlDelete(mut pstBaseNode: Ptr<AVLBASE_NODE_S>, mut pstBaseTree: Ptr<AVLBASE_TREE_S>) {
    let mut pstReplaceNode: Ptr<AVLBASE_NODE_S> = Default::default();
    let mut pstParentNode: Ptr<AVLBASE_NODE_S> = Default::default();
    let mut sNewHeight: i16 = 0;
    pstReplaceNode = VosAvlDeleteCheck(pstBaseTree.cast(), pstBaseNode.cast()).cast();
    pstParentNode = pstBaseNode.pstParent.cast();
    pstBaseNode.pstParent = AVL_NULL_PTR!();
    pstBaseNode.pstRight = AVL_NULL_PTR!();
    pstBaseNode.pstLeft = AVL_NULL_PTR!();
    pstBaseNode.sRHeight = -1;
    pstBaseNode.sLHeight = -1;
    if (pstReplaceNode != AVL_NULL_PTR!()).as_bool() {
        pstReplaceNode.pstParent = pstParentNode.cast();
        sNewHeight = (1 + VOS_V2_AVL_MAX!(pstReplaceNode.sLHeight, pstReplaceNode.sRHeight)).cast();
    }
    if (pstParentNode != AVL_NULL_PTR!()).as_bool() {
        if (pstParentNode.pstRight == pstBaseNode).as_bool() {
            pstParentNode.pstRight = pstReplaceNode.cast();
            pstParentNode.sRHeight = sNewHeight.cast();
        } else {
            pstParentNode.pstLeft = pstReplaceNode.cast();
            pstParentNode.sLHeight = sNewHeight.cast();
        }
        VosAvlBalanceTree(pstBaseTree.cast(), pstParentNode.cast());
    } else {
        pstBaseTree.pstRoot = pstReplaceNode.cast();
    }
    return;
}

pub fn VOS_AVL_Insert_Or_Find(mut pstTree: Ptr<AVL_TREE>, mut pstNode: Ptr<AVL_NODE>) -> Ptr<Void> {
    let mut pstParentNode: Ptr<AVL_NODE> = Default::default();
    let mut iResult: i32 = Default::default();
    if (pstTree == AVL_NULL_PTR!()).as_bool()
        || (pstNode == AVL_NULL_PTR!()).as_bool()
        || VOS_AVL_IN_TREE!(*pstNode).as_bool()
    {
        return AVL_NULL_PTR!();
    }
    pstNode.sRHeight = 0;
    pstNode.sLHeight = 0;
    if (pstTree.pstRoot == AVL_NULL_PTR!()).as_bool() {
        pstTree.pstRoot = pstNode.cast();
        pstTree.pstFirst = pstNode.cast();
        pstTree.pstLast = pstNode.cast();
        return AVL_NULL_PTR!();
    }
    pstParentNode = pstTree.pstRoot.cast();
    while (pstParentNode != AVL_NULL_PTR!()).as_bool() {
        iResult = (pstTree.pfnCompare)(pstNode.pKey.cast(), pstParentNode.pKey.cast()).cast();
        if iResult > 0 {
            if (pstParentNode.pstRight != AVL_NULL_PTR!()).as_bool() {
                pstParentNode = pstParentNode.pstRight.cast();
                continue;
            }
            VosAvlNodeRightInsert(
                c_ref!(pstTree.pstRoot).cast::<Ptr<AVLBASE_TREE_S>>(),
                pstParentNode.cast::<Ptr<AVLBASE_NODE_S>>(),
                pstNode.cast::<Ptr<AVLBASE_NODE_S>>(),
            );
            break;
        } else if iResult < 0 {
            if (pstParentNode.pstLeft != AVL_NULL_PTR!()).as_bool() {
                pstParentNode = pstParentNode.pstLeft.cast();
                continue;
            }
            VosAvlNodeLeftInsert(
                c_ref!(pstTree.pstRoot).cast::<Ptr<AVLBASE_TREE_S>>(),
                pstParentNode.cast::<Ptr<AVLBASE_NODE_S>>(),
                pstNode.cast::<Ptr<AVLBASE_NODE_S>>(),
            );
            break;
        }
        pstNode.sRHeight = -1;
        pstNode.sLHeight = -1;
        return pstParentNode.pSelf.cast();
    }
    if (pstParentNode != AVL_NULL_PTR!()).as_bool() {
        VosAvlBalanceTree(
            c_ref!(pstTree.pstRoot).cast::<Ptr<AVLBASE_TREE_S>>(),
            pstParentNode.cast::<Ptr<AVLBASE_NODE_S>>(),
        );
    }
    return AVL_NULL_PTR!();
}

pub fn VOS_AVL_Delete(mut pstTree: Ptr<AVL_TREE>, mut pstNode: Ptr<AVL_NODE>) {
    let mut pstBaseNode: Ptr<AVLBASE_NODE_S> = Default::default();
    let mut pstBaseTree: Ptr<AVLBASE_TREE_S> = Default::default();
    if (pstTree == AVL_NULL_PTR!()).as_bool()
        || (pstNode == AVL_NULL_PTR!()).as_bool()
        || (!VOS_AVL_IN_TREE!(*pstNode)).as_bool()
    {
        return;
    }
    pstBaseNode = pstNode.cast::<Ptr<AVLBASE_NODE_S>>();
    pstBaseTree = c_ref!(pstTree.pstRoot)
        .cast::<Ptr<Void>>()
        .cast::<Ptr<AVLBASE_TREE_S>>();
    VosAvlDelete(pstBaseNode.cast(), pstBaseTree.cast());
    return;
}

pub fn VOS_AVL_Find(mut pstTree: Ptr<AVL_TREE>, mut pKey: Ptr<Void>) -> Ptr<Void> {
    let mut pstNode: Ptr<AVL_NODE> = Default::default();
    let mut iResult: i32 = Default::default();
    if (pstTree == AVL_NULL_PTR!()).as_bool() {
        return AVL_NULL_PTR!();
    }
    pstNode = pstTree.pstRoot.cast();
    while (pstNode != AVL_NULL_PTR!()).as_bool() {
        iResult = (pstTree.pfnCompare)(pKey.cast(), pstNode.pKey.cast()).cast();
        if (iResult > 0).as_bool() {
            pstNode = pstNode.pstRight.cast();
        } else if (iResult < 0).as_bool() {
            pstNode = pstNode.pstLeft.cast();
        } else {
            break;
        }
    }
    return if (pstNode != AVL_NULL_PTR!()).as_bool() {
        pstNode.pSelf.cast()
    } else {
        AVL_NULL_PTR!()
    };
}

pub fn VOS_AVL_Next(mut pstNode: Ptr<AVL_NODE>) -> Ptr<Void> {
    let mut pstNodeTmp: Ptr<AVL_NODE> = pstNode.cast();
    if (pstNodeTmp == AVL_NULL_PTR!()).as_bool() || (!VOS_AVL_IN_TREE!(*pstNodeTmp)).as_bool() {
        return AVL_NULL_PTR!();
    }
    if (pstNodeTmp.pstRight != AVL_NULL_PTR!()).as_bool() {
        pstNodeTmp = pstNodeTmp.pstRight.cast();
        FIND_LEFTMOST_NODE!(pstNodeTmp);
    } else {
        while (pstNodeTmp != AVL_NULL_PTR!()).as_bool() {
            if (pstNodeTmp.pstParent == AVL_NULL_PTR!()).as_bool()
                || (pstNodeTmp.pstParent.pstLeft == pstNodeTmp).as_bool()
            {
                pstNodeTmp = pstNodeTmp.pstParent.cast();
                break;
            }
            pstNodeTmp = pstNodeTmp.pstParent.cast();
        }
    }
    return if pstNodeTmp != AVL_NULL_PTR!() {
        pstNodeTmp.pSelf.cast()
    } else {
        AVL_NULL_PTR!()
    };
}

pub fn VOS_AVL_Prev(mut pstNode: Ptr<AVL_NODE>) -> Ptr<Void> {
    let mut pstNodeTmp: Ptr<AVL_NODE> = pstNode.cast();
    if (pstNodeTmp == AVL_NULL_PTR!()).as_bool() || (!VOS_AVL_IN_TREE!(*pstNodeTmp)).as_bool() {
        return AVL_NULL_PTR!();
    }
    if (pstNodeTmp.pstLeft != AVL_NULL_PTR!()).as_bool() {
        pstNodeTmp = pstNodeTmp.pstLeft.cast();
        FIND_RIGHTMOST_NODE!(pstNodeTmp);
    } else {
        while (pstNodeTmp != AVL_NULL_PTR!()).as_bool() {
            if (pstNodeTmp.pstParent == AVL_NULL_PTR!()).as_bool()
                || (pstNodeTmp.pstParent.pstRight == pstNodeTmp).as_bool()
            {
                pstNodeTmp = pstNodeTmp.pstParent.cast();
                break;
            }
            pstNodeTmp = pstNodeTmp.pstParent.cast();
        }
    }
    return if pstNodeTmp != AVL_NULL_PTR!() {
        pstNodeTmp.pSelf.cast()
    } else {
        AVL_NULL_PTR!()
    };
}

pub fn VOS_AVL_Find_Or_Find_Next(
    mut pstTree: Ptr<AVL_TREE>,
    mut pKey: Ptr<Void>,
    mut bValue: u32,
) -> Ptr<Void> {
    let mut pstNode: Ptr<AVL_NODE> = Default::default();
    let mut pFoundNode: Ptr<Void> = AVL_NULL_PTR!();
    let mut iResult: i32 = Default::default();
    if (pstTree == AVL_NULL_PTR!()).as_bool() {
        return AVL_NULL_PTR!();
    }
    pstNode = pstTree.pstRoot.cast();
    if (pstNode == AVL_NULL_PTR!()).as_bool() {
        return pFoundNode.cast();
    }
    loop {
        iResult = (pstTree.pfnCompare)(pKey.cast(), pstNode.pKey.cast()).cast();
        if iResult > 0 {
            if (pstNode.pstRight == AVL_NULL_PTR!()).as_bool() {
                pFoundNode = VOS_AVL_Next(pstNode.cast()).cast();
                break;
            }
            pstNode = pstNode.pstRight.cast();
        } else if iResult < 0 {
            if (pstNode.pstLeft == AVL_NULL_PTR!()).as_bool() {
                pFoundNode = pstNode.pSelf.cast();
                break;
            }
            pstNode = pstNode.pstLeft.cast();
        } else {
            if (bValue != 0).as_bool() {
                pFoundNode = VOS_AVL_Next(pstNode.cast()).cast();
            } else {
                pFoundNode = pstNode.pSelf.cast();
            }
            break;
        }
    }
    return pFoundNode.cast();
}
