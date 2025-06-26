use std::ptr::null_mut;

// Assuming the following types are defined elsewhere in the Rust code:
// type AVL3_TREE = ...;
// type AVL3_NODE = ...;
// type AVL3_TREE_INFO = ...;
// const AVL_NULL_PTR: *mut () = null_mut();

pub unsafe fn AVL3_Find_Or_Find_Next(
    pstTree: *mut AVL3_TREE,
    pKey: *const (),
    bFlag: u32,
    pstTreeInfo: *mut AVL3_TREE_INFO,
) -> *mut () {
    if TREE_OR_TREEINFO_IS_NULL(pstTree, pstTreeInfo) {
        return AVL_NULL_PTR;
    }

    let mut pstNode = (*pstTree).pstRoot;
    if pstNode.is_null() {
        return AVL_NULL_PTR;
    }

    let iKeyOffset = GET_KEYOFFSET(pstTreeInfo);
    let mut pFoundNode = AVL_NULL_PTR;

    loop {
        let node_key_ptr = (pstNode as *mut u8).add(iKeyOffset) as *mut ();
        let iResult = ((*pstTreeInfo).pfCompare)(pKey, node_key_ptr);

        match iResult.cmp(&0) {
            std::cmp::Ordering::Greater => {
                if (*pstNode).pstRight.is_null() {
                    pFoundNode = VOS_AVL3_Next(pstNode, pstTreeInfo);
                    break;
                }
                pstNode = (*pstNode).pstRight;
            }
            std::cmp::Ordering::Less => {
                if (*pstNode).pstLeft.is_null() {
                    pFoundNode = ((pstNode as *mut u8).sub((*pstTreeInfo).usNodeOffset)) as *mut ();
                    break;
                }
                pstNode = (*pstNode).pstLeft;
            }
            std::cmp::Ordering::Equal => {
                if bFlag != 0 {
                    pFoundNode = VOS_AVL3_Next(pstNode, pstTreeInfo);
                } else {
                    pFoundNode = ((pstNode as *mut u8).sub((*pstTreeInfo).usNodeOffset)) as *mut ();
                }
                break;
            }
        }
    }

    pFoundNode
}
