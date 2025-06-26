pub mod avl_c;

use avl_c::*;

use crate::translation_utils::*;

#[cfg(test)]
mod tests {
    use super::*;
    type VOS_UINT32 = u32;
    type VOS_CHAR = u8;
    macro_rules! VOS_TRUE {
        () => {
            1
        };
    }
    macro_rules! VOS_FALSE {
        () => {
            0
        };
    }
    #[derive(Default, Clone, Copy)]
    #[repr(C)]
    struct tagAvlUserStruct {
        pub ulID: VOS_UINT32,
        pub iKey: VOS_UINT32,
        pub stNode: AVL_NODE,
    }
    type AvlUserStruct = tagAvlUserStruct;
    fn AVL_TestCompareFn(mut pKey1: VoidPtr, mut pKey2: VoidPtr) -> i32 {
        if pKey1 == NULL!() || pKey2 == NULL!() {
            return 0;
        }
        return (*pKey1.cast::<Ptr<i32>>() - *pKey2.cast::<Ptr<i32>>()).cast();
    }
    fn TestInitAVLNodeKey(mut pstUserNode: Ptr<AvlUserStruct>, mut uiKey: VOS_UINT32) {
        if pstUserNode == NULL!() {
            return;
        }
        pstUserNode.iKey = uiKey;
        VOS_AVL_INIT_NODE!(
            pstUserNode.stNode,
            pstUserNode.cast(),
            c_ref!(pstUserNode.iKey).cast()
        );
    }
    fn AVLPreOrderSearch(
        mut pstNode: Ptr<AVL_NODE>,
        mut puiOrderArr: Ptr<VOS_UINT32>,
        mut puiNodeNum: Ptr<VOS_UINT32>,
    ) {
        let mut uiNodeSum = *puiNodeNum;
        let mut puiNextOrderAddr = puiOrderArr;
        if pstNode == NULL!() || *puiNodeNum == 0 {
            return;
        }
        puiNextOrderAddr[0] = *pstNode.pKey.cast::<Ptr<VOS_UINT32>>();
        *puiNodeNum -= 1;
        puiNextOrderAddr = puiOrderArr + 1;
        AVLPreOrderSearch(pstNode.pstLeft, puiNextOrderAddr, puiNodeNum);
        puiNextOrderAddr = puiOrderArr + uiNodeSum - *puiNodeNum;
        AVLPreOrderSearch(pstNode.pstRight, puiNextOrderAddr, puiNodeNum);
    }
    fn AVLInOrderSearch(
        mut pstNode: Ptr<AVL_NODE>,
        mut puiOrderArr: Ptr<VOS_UINT32>,
        mut puiNodeNum: Ptr<VOS_UINT32>,
    ) {
        let mut uiNodeSum = *puiNodeNum;
        let mut puiNextOrderAddr = puiOrderArr;
        if pstNode == NULL!() || *puiNodeNum == 0 {
            return;
        }
        AVLInOrderSearch(pstNode.pstLeft, puiNextOrderAddr, puiNodeNum);
        puiNextOrderAddr = puiOrderArr + uiNodeSum - *puiNodeNum;
        puiNextOrderAddr[0] = *pstNode.pKey.cast::<Ptr<VOS_UINT32>>();
        *puiNodeNum -= 1;
        puiNextOrderAddr += 1;
        AVLInOrderSearch(pstNode.pstRight, puiNextOrderAddr, puiNodeNum);
    }
    fn TestCheckAVLTree(
        mut pstTree: Ptr<AVL_TREE>,
        mut puiPreOrderNode: Ptr<VOS_UINT32>,
        mut puiInOrderNode: Ptr<VOS_UINT32>,
        mut uiNodeNum: VOS_UINT32,
    ) {
        let mut puiNowPreOrderNode: Ptr<VOS_UINT32> = NULL!();
        let mut puiNowInOrderNode: Ptr<VOS_UINT32> = NULL!();
        let mut uiPreOrderNum = uiNodeNum;
        let mut uiInOrderNum = uiNodeNum;
        let mut i: VOS_UINT32;
        puiNowPreOrderNode = c_malloc!(c_sizeof!(VOS_UINT32) * uiNodeNum);
        puiNowInOrderNode = c_malloc!(c_sizeof!(VOS_UINT32) * uiNodeNum);
        AVLPreOrderSearch(pstTree.pstRoot, puiNowPreOrderNode, c_ref!(uiPreOrderNum));
        AVLInOrderSearch(pstTree.pstRoot, puiNowInOrderNode, c_ref!(uiInOrderNum));
        c_for!(i = 0; i < uiNodeNum; i += 1; {
            assert_eq!(puiPreOrderNode[i], puiNowPreOrderNode[i]);
            assert_eq!(puiInOrderNode[i], puiNowInOrderNode[i]);
        });
        c_free!(puiNowPreOrderNode);
        c_free!(puiNowInOrderNode);
    }
    #[test]
    fn UT_VOS_AVL_Insert_Or_Find001() {
        let mut stTree: AVL_TREE = Default::default();
        let mut stUserNode: AvlUserStruct = Default::default();
        VOS_AVL_INIT_TREE!(stTree, func!(AVL_TestCompareFn));
        TestInitAVLNodeKey(c_ref!(stUserNode), 1);
        assert!(VOS_AVL_Insert_Or_Find(NULL!(), c_ref!(stUserNode.stNode)) == NULL!());
        assert!(VOS_AVL_Insert_Or_Find(c_ref!(stTree), NULL!()) == NULL!());
        test_no_memory_leak!();
    }
    #[test]
    fn UT_VOS_AVL_Insert_Or_Find002() {
        let mut stTree: AVL_TREE = Default::default();
        let mut stUserNode: Array<AvlUserStruct, 3> = Default::default();
        let mut auiPreOrder: Array<VOS_UINT32, 3> = arr![2, 1, 3];
        let mut auiInOrder: Array<VOS_UINT32, 3> = arr![1, 2, 3];
        VOS_AVL_INIT_TREE!(stTree, func!(AVL_TestCompareFn));
        TestInitAVLNodeKey(c_ref!(stUserNode[0]), 1);
        TestInitAVLNodeKey(c_ref!(stUserNode[1]), 2);
        TestInitAVLNodeKey(c_ref!(stUserNode[2]), 3);
        assert!(VOS_AVL_Insert_Or_Find(c_ref!(stTree), c_ref!(stUserNode[0].stNode)) == NULL!());
        assert!(VOS_AVL_Insert_Or_Find(c_ref!(stTree), c_ref!(stUserNode[1].stNode)) == NULL!());
        assert!(VOS_AVL_Insert_Or_Find(c_ref!(stTree), c_ref!(stUserNode[2].stNode)) == NULL!());
        TestCheckAVLTree(c_ref!(stTree), auiPreOrder.cast(), auiInOrder.cast(), 3);
        test_no_memory_leak!();
    }
    #[test]
    fn UT_VOS_AVL_Insert_Or_Find003() {
        let mut stTree: AVL_TREE = Default::default();
        let mut stUserNode: Array<AvlUserStruct, 3> = Default::default();
        let mut auiPreOrder: Array<VOS_UINT32, 3> = arr![2, 1, 3];
        let mut auiInOrder: Array<VOS_UINT32, 3> = arr![1, 2, 3];
        VOS_AVL_INIT_TREE!(stTree, func!(AVL_TestCompareFn));
        TestInitAVLNodeKey(c_ref!(stUserNode[0]), 3);
        TestInitAVLNodeKey(c_ref!(stUserNode[1]), 2);
        TestInitAVLNodeKey(c_ref!(stUserNode[2]), 1);
        assert!(VOS_AVL_Insert_Or_Find(c_ref!(stTree), c_ref!(stUserNode[0].stNode)) == NULL!());
        assert!(VOS_AVL_Insert_Or_Find(c_ref!(stTree), c_ref!(stUserNode[1].stNode)) == NULL!());
        assert!(VOS_AVL_Insert_Or_Find(c_ref!(stTree), c_ref!(stUserNode[2].stNode)) == NULL!());
        TestCheckAVLTree(c_ref!(stTree), auiPreOrder.cast(), auiInOrder.cast(), 3);
        test_no_memory_leak!();
    }
    #[test]
    fn UT_VOS_AVL_Insert_Or_Find004() {
        let mut stTree: AVL_TREE = Default::default();
        let mut stUserNode: Array<AvlUserStruct, 3> = Default::default();
        let mut auiPreOrder: Array<VOS_UINT32, 3> = arr![2, 1, 3];
        let mut auiInOrder: Array<VOS_UINT32, 3> = arr![1, 2, 3];
        VOS_AVL_INIT_TREE!(stTree, func!(AVL_TestCompareFn));
        TestInitAVLNodeKey(c_ref!(stUserNode[0]), 3);
        TestInitAVLNodeKey(c_ref!(stUserNode[1]), 1);
        TestInitAVLNodeKey(c_ref!(stUserNode[2]), 2);
        assert!(VOS_AVL_Insert_Or_Find(c_ref!(stTree), c_ref!(stUserNode[0].stNode)) == NULL!());
        assert!(VOS_AVL_Insert_Or_Find(c_ref!(stTree), c_ref!(stUserNode[1].stNode)) == NULL!());
        assert!(VOS_AVL_Insert_Or_Find(c_ref!(stTree), c_ref!(stUserNode[2].stNode)) == NULL!());
        TestCheckAVLTree(c_ref!(stTree), auiPreOrder.cast(), auiInOrder.cast(), 3);
        test_no_memory_leak!();
    }
    #[test]
    fn UT_VOS_AVL_Insert_Or_Find005() {
        let mut stTree: AVL_TREE = Default::default();
        let mut stUserNode: Array<AvlUserStruct, 3> = Default::default();
        let mut auiPreOrder: Array<VOS_UINT32, 3> = arr![2, 1, 3];
        let mut auiInOrder: Array<VOS_UINT32, 3> = arr![1, 2, 3];
        VOS_AVL_INIT_TREE!(stTree, func!(AVL_TestCompareFn));
        TestInitAVLNodeKey(c_ref!(stUserNode[0]), 1);
        TestInitAVLNodeKey(c_ref!(stUserNode[1]), 3);
        TestInitAVLNodeKey(c_ref!(stUserNode[2]), 2);
        assert!(VOS_AVL_Insert_Or_Find(c_ref!(stTree), c_ref!(stUserNode[0].stNode)) == NULL!());
        assert!(VOS_AVL_Insert_Or_Find(c_ref!(stTree), c_ref!(stUserNode[1].stNode)) == NULL!());
        assert!(VOS_AVL_Insert_Or_Find(c_ref!(stTree), c_ref!(stUserNode[2].stNode)) == NULL!());
        TestCheckAVLTree(c_ref!(stTree), auiPreOrder.cast(), auiInOrder.cast(), 3);
        test_no_memory_leak!();
    }
    #[test]
    fn UT_VOS_AVL_Insert_Or_Find006() {
        let mut stTree: AVL_TREE = Default::default();
        let mut stUserNode: Array<AvlUserStruct, 4> = Default::default();
        let mut auiPreOrder: Array<VOS_UINT32, 3> = arr![2, 1, 3];
        let mut auiInOrder: Array<VOS_UINT32, 3> = arr![1, 2, 3];
        VOS_AVL_INIT_TREE!(stTree, func!(AVL_TestCompareFn));
        TestInitAVLNodeKey(c_ref!(stUserNode[0]), 3);
        TestInitAVLNodeKey(c_ref!(stUserNode[1]), 1);
        TestInitAVLNodeKey(c_ref!(stUserNode[2]), 2);
        TestInitAVLNodeKey(c_ref!(stUserNode[3]), 1);
        assert!(VOS_AVL_Insert_Or_Find(c_ref!(stTree), c_ref!(stUserNode[0].stNode)) == NULL!());
        assert!(VOS_AVL_Insert_Or_Find(c_ref!(stTree), c_ref!(stUserNode[1].stNode)) == NULL!());
        assert!(VOS_AVL_Insert_Or_Find(c_ref!(stTree), c_ref!(stUserNode[2].stNode)) == NULL!());
        assert_eq!(
            VOS_AVL_Insert_Or_Find(c_ref!(stTree), c_ref!(stUserNode[3].stNode)),
            c_ref!(stUserNode[1]).cast()
        );
        TestCheckAVLTree(c_ref!(stTree), auiPreOrder.cast(), auiInOrder.cast(), 3);
        test_no_memory_leak!();
    }
    #[test]
    fn UT_VOS_AVL_Delete001() {
        let mut stTree: AVL_TREE = Default::default();
        let mut stUserNode: Array<AvlUserStruct, 3> = Default::default();
        let mut auiPreOrder: Array<VOS_UINT32, 3> = arr![2, 1, 3];
        let mut auiInOrder: Array<VOS_UINT32, 3> = arr![1, 2, 3];
        VOS_AVL_INIT_TREE!(stTree, func!(AVL_TestCompareFn));
        TestInitAVLNodeKey(c_ref!(stUserNode[0]), 3);
        TestInitAVLNodeKey(c_ref!(stUserNode[1]), 1);
        TestInitAVLNodeKey(c_ref!(stUserNode[2]), 2);
        assert!(VOS_AVL_Insert_Or_Find(c_ref!(stTree), c_ref!(stUserNode[0].stNode)) == NULL!());
        assert!(VOS_AVL_Insert_Or_Find(c_ref!(stTree), c_ref!(stUserNode[1].stNode)) == NULL!());
        assert!(VOS_AVL_Insert_Or_Find(c_ref!(stTree), c_ref!(stUserNode[2].stNode)) == NULL!());
        VOS_AVL_Delete(NULL!(), c_ref!(stUserNode[2].stNode));
        VOS_AVL_Delete(c_ref!(stTree), NULL!());
        TestCheckAVLTree(c_ref!(stTree), auiPreOrder.cast(), auiInOrder.cast(), 3);
        test_no_memory_leak!();
    }
    #[test]
    fn UT_VOS_AVL_Delete002() {
        let mut stTree: AVL_TREE = Default::default();
        let mut stUserNode: Array<AvlUserStruct, 4> = Default::default();
        let mut auiPreOrder: Array<VOS_UINT32, 3> = arr![3, 2, 4];
        let mut auiInOrder: Array<VOS_UINT32, 3> = arr![2, 3, 4];
        VOS_AVL_INIT_TREE!(stTree, func!(AVL_TestCompareFn));
        TestInitAVLNodeKey(c_ref!(stUserNode[0]), 3);
        TestInitAVLNodeKey(c_ref!(stUserNode[1]), 1);
        TestInitAVLNodeKey(c_ref!(stUserNode[2]), 4);
        TestInitAVLNodeKey(c_ref!(stUserNode[3]), 2);
        assert!(VOS_AVL_Insert_Or_Find(c_ref!(stTree), c_ref!(stUserNode[0].stNode)) == NULL!());
        assert!(VOS_AVL_Insert_Or_Find(c_ref!(stTree), c_ref!(stUserNode[1].stNode)) == NULL!());
        assert!(VOS_AVL_Insert_Or_Find(c_ref!(stTree), c_ref!(stUserNode[2].stNode)) == NULL!());
        assert!(VOS_AVL_Insert_Or_Find(c_ref!(stTree), c_ref!(stUserNode[3].stNode)) == NULL!());
        VOS_AVL_Delete(c_ref!(stTree), c_ref!(stUserNode[1].stNode));
        TestCheckAVLTree(c_ref!(stTree), auiPreOrder.cast(), auiInOrder.cast(), 3);
        test_no_memory_leak!();
    }
    #[test]
    fn UT_VOS_AVL_Delete003() {
        let mut stTree: AVL_TREE = Default::default();
        let mut stUserNode: Array<AvlUserStruct, 4> = Default::default();
        let mut auiPreOrder: Array<VOS_UINT32, 3> = arr![2, 1, 3];
        let mut auiInOrder: Array<VOS_UINT32, 3> = arr![1, 2, 3];
        VOS_AVL_INIT_TREE!(stTree, func!(AVL_TestCompareFn));
        TestInitAVLNodeKey(c_ref!(stUserNode[0]), 2);
        TestInitAVLNodeKey(c_ref!(stUserNode[1]), 1);
        TestInitAVLNodeKey(c_ref!(stUserNode[2]), 4);
        TestInitAVLNodeKey(c_ref!(stUserNode[3]), 3);
        assert!(VOS_AVL_Insert_Or_Find(c_ref!(stTree), c_ref!(stUserNode[0].stNode)) == NULL!());
        assert!(VOS_AVL_Insert_Or_Find(c_ref!(stTree), c_ref!(stUserNode[1].stNode)) == NULL!());
        assert!(VOS_AVL_Insert_Or_Find(c_ref!(stTree), c_ref!(stUserNode[2].stNode)) == NULL!());
        assert!(VOS_AVL_Insert_Or_Find(c_ref!(stTree), c_ref!(stUserNode[3].stNode)) == NULL!());
        VOS_AVL_Delete(c_ref!(stTree), c_ref!(stUserNode[2].stNode));
        TestCheckAVLTree(c_ref!(stTree), auiPreOrder.cast(), auiInOrder.cast(), 3);
        test_no_memory_leak!();
    }
    #[test]
    fn UT_VOS_AVL_Delete004() {
        let mut stTree: AVL_TREE = Default::default();
        let mut stUserNode: Array<AvlUserStruct, 3> = Default::default();
        let mut auiPreOrder: Array<VOS_UINT32, 2> = arr![1, 3];
        let mut auiInOrder: Array<VOS_UINT32, 2> = arr![1, 3];
        VOS_AVL_INIT_TREE!(stTree, func!(AVL_TestCompareFn));
        TestInitAVLNodeKey(c_ref!(stUserNode[0]), 2);
        TestInitAVLNodeKey(c_ref!(stUserNode[1]), 1);
        TestInitAVLNodeKey(c_ref!(stUserNode[2]), 3);
        assert!(VOS_AVL_Insert_Or_Find(c_ref!(stTree), c_ref!(stUserNode[0].stNode)) == NULL!());
        assert!(VOS_AVL_Insert_Or_Find(c_ref!(stTree), c_ref!(stUserNode[1].stNode)) == NULL!());
        assert!(VOS_AVL_Insert_Or_Find(c_ref!(stTree), c_ref!(stUserNode[2].stNode)) == NULL!());
        VOS_AVL_Delete(c_ref!(stTree), c_ref!(stUserNode[0].stNode));
        TestCheckAVLTree(c_ref!(stTree), auiPreOrder.cast(), auiInOrder.cast(), 2);
        test_no_memory_leak!();
    }
    #[test]
    fn UT_VOS_AVL_Delete005() {
        let mut stTree: AVL_TREE = Default::default();
        let mut stUserNode: Array<AvlUserStruct, 9> = Default::default();
        let mut auiPreOrderBefore: Array<VOS_UINT32, 9> = arr![5, 3, 2, 4, 9, 7, 6, 8, 10];
        let mut auiInOrderBefore: Array<VOS_UINT32, 9> = arr![2, 3, 4, 5, 6, 7, 8, 9, 10];
        let mut auiPreOrderAfter1: Array<VOS_UINT32, 8> = arr![5, 3, 2, 4, 8, 7, 6, 10];
        let mut auiInOrderAfter1: Array<VOS_UINT32, 8> = arr![2, 3, 4, 5, 6, 7, 8, 10];
        let mut auiPreOrderAfter2: Array<VOS_UINT32, 7> = arr![6, 3, 2, 4, 8, 7, 10];
        let mut auiInOrderAfter2: Array<VOS_UINT32, 7> = arr![2, 3, 4, 6, 7, 8, 10];
        let mut uiNodeCount = 9;
        let mut i: VOS_UINT32;
        VOS_AVL_INIT_TREE!(stTree, func!(AVL_TestCompareFn));
        TestInitAVLNodeKey(c_ref!(stUserNode[0]), 5);
        TestInitAVLNodeKey(c_ref!(stUserNode[1]), 3);
        TestInitAVLNodeKey(c_ref!(stUserNode[2]), 9);
        TestInitAVLNodeKey(c_ref!(stUserNode[3]), 2);
        TestInitAVLNodeKey(c_ref!(stUserNode[4]), 4);
        TestInitAVLNodeKey(c_ref!(stUserNode[5]), 7);
        TestInitAVLNodeKey(c_ref!(stUserNode[6]), 10);
        TestInitAVLNodeKey(c_ref!(stUserNode[7]), 6);
        TestInitAVLNodeKey(c_ref!(stUserNode[8]), 8);
        c_for!(i = 0; i < uiNodeCount; i += 1; {
            assert!(VOS_AVL_Insert_Or_Find(c_ref!(stTree), c_ref!(stUserNode[i].stNode)) == NULL!());
        });
        TestCheckAVLTree(
            c_ref!(stTree),
            auiPreOrderBefore.cast(),
            auiInOrderBefore.cast(),
            uiNodeCount,
        );
        VOS_AVL_Delete(c_ref!(stTree), c_ref!(stUserNode[2].stNode));
        TestCheckAVLTree(
            c_ref!(stTree),
            auiPreOrderAfter1.cast(),
            auiInOrderAfter1.cast(),
            uiNodeCount - 1,
        );
        VOS_AVL_Delete(c_ref!(stTree), c_ref!(stUserNode[0].stNode));
        TestCheckAVLTree(
            c_ref!(stTree),
            auiPreOrderAfter2.cast(),
            auiInOrderAfter2.cast(),
            uiNodeCount - 2,
        );
        test_no_memory_leak!();
    }
    #[test]
    fn UT_VOS_AVL_Delete006() {
        let mut stTree: AVL_TREE = Default::default();
        let mut stUserNode: Array<AvlUserStruct, 8> = Default::default();
        let mut auiPreOrderBefore: Array<VOS_UINT32, 8> = arr![5, 3, 2, 4, 9, 7, 6, 10];
        let mut auiInOrderBefore: Array<VOS_UINT32, 8> = arr![2, 3, 4, 5, 6, 7, 9, 10];
        let mut auiPreOrderAfter: Array<VOS_UINT32, 7> = arr![5, 3, 2, 4, 7, 6, 10];
        let mut auiInOrderAfter: Array<VOS_UINT32, 7> = arr![2, 3, 4, 5, 6, 7, 10];
        let mut uiNodeCount = 8;
        let mut i: VOS_UINT32;
        VOS_AVL_INIT_TREE!(stTree, func!(AVL_TestCompareFn));
        TestInitAVLNodeKey(c_ref!(stUserNode[0]), 5);
        TestInitAVLNodeKey(c_ref!(stUserNode[1]), 3);
        TestInitAVLNodeKey(c_ref!(stUserNode[2]), 9);
        TestInitAVLNodeKey(c_ref!(stUserNode[3]), 2);
        TestInitAVLNodeKey(c_ref!(stUserNode[4]), 4);
        TestInitAVLNodeKey(c_ref!(stUserNode[5]), 7);
        TestInitAVLNodeKey(c_ref!(stUserNode[6]), 10);
        TestInitAVLNodeKey(c_ref!(stUserNode[7]), 6);
        c_for!(i = 0; i < uiNodeCount; i += 1; {
            assert!(VOS_AVL_Insert_Or_Find(c_ref!(stTree), c_ref!(stUserNode[i].stNode)) == NULL!());
        });
        TestCheckAVLTree(
            c_ref!(stTree),
            auiPreOrderBefore.cast(),
            auiInOrderBefore.cast(),
            uiNodeCount,
        );
        VOS_AVL_Delete(c_ref!(stTree), c_ref!(stUserNode[2].stNode));
        TestCheckAVLTree(
            c_ref!(stTree),
            auiPreOrderAfter.cast(),
            auiInOrderAfter.cast(),
            uiNodeCount - 1,
        );
        test_no_memory_leak!();
    }
    #[test]
    fn UT_VOS_AVL_Delete007() {
        let mut stTree: AVL_TREE = Default::default();
        let mut stUserNode: Array<AvlUserStruct, 9> = Default::default();
        let mut auiPreOrderBefore: Array<VOS_UINT32, 9> = arr![5, 3, 2, 4, 9, 7, 12, 11, 13];
        let mut auiInOrderBefore: Array<VOS_UINT32, 9> = arr![2, 3, 4, 5, 7, 9, 11, 12, 13];
        let mut auiPreOrderAfter: Array<VOS_UINT32, 8> = arr![5, 3, 2, 4, 11, 7, 12, 13];
        let mut auiInOrderAfter: Array<VOS_UINT32, 8> = arr![2, 3, 4, 5, 7, 11, 12, 13];
        let mut uiNodeCount = 9;
        let mut i: VOS_UINT32;
        VOS_AVL_INIT_TREE!(stTree, func!(AVL_TestCompareFn));
        TestInitAVLNodeKey(c_ref!(stUserNode[0]), 5);
        TestInitAVLNodeKey(c_ref!(stUserNode[1]), 3);
        TestInitAVLNodeKey(c_ref!(stUserNode[2]), 9);
        TestInitAVLNodeKey(c_ref!(stUserNode[3]), 2);
        TestInitAVLNodeKey(c_ref!(stUserNode[4]), 4);
        TestInitAVLNodeKey(c_ref!(stUserNode[5]), 7);
        TestInitAVLNodeKey(c_ref!(stUserNode[6]), 12);
        TestInitAVLNodeKey(c_ref!(stUserNode[7]), 11);
        TestInitAVLNodeKey(c_ref!(stUserNode[8]), 13);
        c_for!(i = 0; i < uiNodeCount; i += 1; {
            assert!(VOS_AVL_Insert_Or_Find(c_ref!(stTree), c_ref!(stUserNode[i].stNode)) == NULL!());
        });
        TestCheckAVLTree(
            c_ref!(stTree),
            auiPreOrderBefore.cast(),
            auiInOrderBefore.cast(),
            uiNodeCount,
        );
        VOS_AVL_Delete(c_ref!(stTree), c_ref!(stUserNode[2].stNode));
        TestCheckAVLTree(
            c_ref!(stTree),
            auiPreOrderAfter.cast(),
            auiInOrderAfter.cast(),
            uiNodeCount - 1,
        );
        test_no_memory_leak!();
    }
    #[test]
    fn UT_VOS_AVL_Delete008() {
        let mut stTree: AVL_TREE = Default::default();
        let mut stUserNode: Array<AvlUserStruct, 14> = Default::default();
        let mut auiPreOrderBefore: Array<VOS_UINT32, 12> =
            arr![9, 3, 2, 1, 7, 5, 6, 8, 11, 10, 12, 13];
        let mut auiInOrderBefore: Array<VOS_UINT32, 12> =
            arr![1, 2, 3, 5, 6, 7, 8, 9, 10, 11, 12, 13];
        let mut auiPreOrderAfter: Array<VOS_UINT32, 11> = arr![9, 5, 2, 1, 7, 6, 8, 11, 10, 12, 13];
        let mut auiInOrderAfter: Array<VOS_UINT32, 11> = arr![1, 2, 5, 6, 7, 8, 9, 10, 11, 12, 13];
        let mut uiNodeCount = 12;
        let mut i: VOS_UINT32;
        VOS_AVL_INIT_TREE!(stTree, func!(AVL_TestCompareFn));
        TestInitAVLNodeKey(c_ref!(stUserNode[0]), 9);
        TestInitAVLNodeKey(c_ref!(stUserNode[1]), 3);
        TestInitAVLNodeKey(c_ref!(stUserNode[2]), 11);
        TestInitAVLNodeKey(c_ref!(stUserNode[3]), 2);
        TestInitAVLNodeKey(c_ref!(stUserNode[4]), 7);
        TestInitAVLNodeKey(c_ref!(stUserNode[5]), 10);
        TestInitAVLNodeKey(c_ref!(stUserNode[6]), 12);
        TestInitAVLNodeKey(c_ref!(stUserNode[7]), 1);
        TestInitAVLNodeKey(c_ref!(stUserNode[8]), 5);
        TestInitAVLNodeKey(c_ref!(stUserNode[9]), 8);
        TestInitAVLNodeKey(c_ref!(stUserNode[10]), 13);
        TestInitAVLNodeKey(c_ref!(stUserNode[11]), 6);
        TestInitAVLNodeKey(c_ref!(stUserNode[12]), 4);
        TestInitAVLNodeKey(c_ref!(stUserNode[13]), 3);
        c_for!(i = 0; i < uiNodeCount; i += 1; {
            assert!(VOS_AVL_Insert_Or_Find(c_ref!(stTree), c_ref!(stUserNode[i].stNode)) == NULL!());
        });
        TestCheckAVLTree(
            c_ref!(stTree),
            auiPreOrderBefore.cast(),
            auiInOrderBefore.cast(),
            uiNodeCount,
        );
        VOS_AVL_Delete(c_ref!(stTree), c_ref!(stUserNode[1].stNode));
        TestCheckAVLTree(
            c_ref!(stTree),
            auiPreOrderAfter.cast(),
            auiInOrderAfter.cast(),
            uiNodeCount - 1,
        );
        assert!(VOS_AVL_Insert_Or_Find(c_ref!(stTree), c_ref!(stUserNode[12].stNode)) == NULL!());
        assert!(VOS_AVL_Insert_Or_Find(c_ref!(stTree), c_ref!(stUserNode[13].stNode)) == NULL!());
        VOS_AVL_Delete(c_ref!(stTree), c_ref!(stUserNode[8].stNode));
        VOS_AVL_Delete(c_ref!(stTree), c_ref!(stUserNode[2].stNode));
        VOS_AVL_Delete(c_ref!(stTree), c_ref!(stUserNode[11].stNode));
        VOS_AVL_Delete(c_ref!(stTree), c_ref!(stUserNode[9].stNode));
        VOS_AVL_Delete(c_ref!(stTree), c_ref!(stUserNode[4].stNode));
        VOS_AVL_Delete(c_ref!(stTree), c_ref!(stUserNode[7].stNode));
        VOS_AVL_Delete(c_ref!(stTree), c_ref!(stUserNode[3].stNode));
        VOS_AVL_Delete(c_ref!(stTree), c_ref!(stUserNode[12].stNode));
        VOS_AVL_Delete(c_ref!(stTree), c_ref!(stUserNode[13].stNode));
        VOS_AVL_Delete(c_ref!(stTree), c_ref!(stUserNode[6].stNode));
        VOS_AVL_Delete(c_ref!(stTree), c_ref!(stUserNode[10].stNode));
        VOS_AVL_Delete(c_ref!(stTree), c_ref!(stUserNode[0].stNode));
        VOS_AVL_Delete(c_ref!(stTree), c_ref!(stUserNode[5].stNode));
        test_no_memory_leak!();
    }
    #[test]
    fn UT_VOS_AVL_Find001() {
        let mut stTree: AVL_TREE = Default::default();
        let mut uiKey = 0;
        VOS_AVL_INIT_TREE!(stTree, func!(AVL_TestCompareFn));
        assert!(VOS_AVL_Find(NULL!(), c_ref!(uiKey)) == NULL!());
        assert!(VOS_AVL_Find(c_ref!(stTree), c_ref!(uiKey)) == NULL!());
        assert!(VOS_AVL_Find(c_ref!(stTree), c_ref!(uiKey)) == NULL!());
        test_no_memory_leak!();
    }
    #[test]
    fn UT_VOS_AVL_Find002() {
        let mut stTree: AVL_TREE = Default::default();
        let mut stUserNode: Array<AvlUserStruct, 3> = Default::default();
        let mut auiPreOrder: Array<VOS_UINT32, 3> = arr![2, 1, 3];
        let mut auiInOrder: Array<VOS_UINT32, 3> = arr![1, 2, 3];
        let mut uiKey = 4;
        VOS_AVL_INIT_TREE!(stTree, func!(AVL_TestCompareFn));
        TestInitAVLNodeKey(c_ref!(stUserNode[0]), 1);
        TestInitAVLNodeKey(c_ref!(stUserNode[1]), 2);
        TestInitAVLNodeKey(c_ref!(stUserNode[2]), 3);
        assert!(VOS_AVL_Insert_Or_Find(c_ref!(stTree), c_ref!(stUserNode[0].stNode)) == NULL!());
        assert!(VOS_AVL_Insert_Or_Find(c_ref!(stTree), c_ref!(stUserNode[1].stNode)) == NULL!());
        assert!(VOS_AVL_Insert_Or_Find(c_ref!(stTree), c_ref!(stUserNode[2].stNode)) == NULL!());
        TestCheckAVLTree(c_ref!(stTree), auiPreOrder.cast(), auiInOrder.cast(), 3);
        assert!(VOS_AVL_Find(c_ref!(stTree), c_ref!(uiKey)) == NULL!());
        test_no_memory_leak!();
    }
    #[test]
    fn UT_VOS_AVL_Find003() {
        let mut stTree: AVL_TREE = Default::default();
        let mut stUserNode: Array<AvlUserStruct, 8> = Default::default();
        let mut uiNodeCount = 8;
        let mut i: VOS_UINT32;
        let mut uiKey: VOS_UINT32;
        VOS_AVL_INIT_TREE!(stTree, func!(AVL_TestCompareFn));
        TestInitAVLNodeKey(c_ref!(stUserNode[0]), 5);
        TestInitAVLNodeKey(c_ref!(stUserNode[1]), 3);
        TestInitAVLNodeKey(c_ref!(stUserNode[2]), 9);
        TestInitAVLNodeKey(c_ref!(stUserNode[3]), 2);
        TestInitAVLNodeKey(c_ref!(stUserNode[4]), 4);
        TestInitAVLNodeKey(c_ref!(stUserNode[5]), 7);
        TestInitAVLNodeKey(c_ref!(stUserNode[6]), 10);
        TestInitAVLNodeKey(c_ref!(stUserNode[7]), 6);
        c_for!(i = 0; i < uiNodeCount; i += 1; {
            assert!(VOS_AVL_Insert_Or_Find(c_ref!(stTree), c_ref!(stUserNode[i].stNode)) == NULL!());
        });
        c_for!(i = 0; i < uiNodeCount; i += 1; {
            uiKey = stUserNode[i].iKey;
            assert_eq!(VOS_AVL_Find(c_ref!(stTree), c_ref!(uiKey).cast()), c_ref!(stUserNode[i]).cast());
        });
        test_no_memory_leak!();
    }
    #[test]
    fn UT_VOS_AVL_Next001() {
        let mut stUserNode: AvlUserStruct = Default::default();
        TestInitAVLNodeKey(c_ref!(stUserNode), 1);
        assert!(VOS_AVL_Next(NULL!()) == NULL!());
        assert!(VOS_AVL_Next(c_ref!(stUserNode.stNode)) == NULL!());
        test_no_memory_leak!();
    }
    #[test]
    fn UT_VOS_AVL_Next002() {
        let mut stTree: AVL_TREE = Default::default();
        let mut stUserNode: Array<AvlUserStruct, 4> = Default::default();
        let mut auiPreOrder: Array<VOS_UINT32, 3> = arr![2, 1, 3];
        let mut auiInOrder: Array<VOS_UINT32, 3> = arr![1, 2, 3];
        VOS_AVL_INIT_TREE!(stTree, func!(AVL_TestCompareFn));
        TestInitAVLNodeKey(c_ref!(stUserNode[0]), 1);
        TestInitAVLNodeKey(c_ref!(stUserNode[1]), 2);
        TestInitAVLNodeKey(c_ref!(stUserNode[2]), 3);
        TestInitAVLNodeKey(c_ref!(stUserNode[3]), 4);
        assert!(VOS_AVL_Insert_Or_Find(c_ref!(stTree), c_ref!(stUserNode[0].stNode)) == NULL!());
        assert!(VOS_AVL_Insert_Or_Find(c_ref!(stTree), c_ref!(stUserNode[1].stNode)) == NULL!());
        assert!(VOS_AVL_Insert_Or_Find(c_ref!(stTree), c_ref!(stUserNode[2].stNode)) == NULL!());
        TestCheckAVLTree(c_ref!(stTree), auiPreOrder.cast(), auiInOrder.cast(), 3);
        assert!(VOS_AVL_Next(c_ref!(stUserNode[3].stNode)) == NULL!());
        test_no_memory_leak!();
    }
    #[test]
    fn UT_VOS_AVL_Next003() {
        let mut stTree: AVL_TREE = Default::default();
        let mut stUserNode: Array<AvlUserStruct, 8> = Default::default();
        let mut auiPreOrder: Array<VOS_UINT32, 8> = arr![5, 3, 2, 4, 7, 6, 9, 10];
        let mut auiInOrder: Array<VOS_UINT32, 8> = arr![2, 3, 4, 5, 6, 7, 9, 10];
        let mut uiNodeCount = 8;
        let mut i: VOS_UINT32;
        VOS_AVL_INIT_TREE!(stTree, func!(AVL_TestCompareFn));
        TestInitAVLNodeKey(c_ref!(stUserNode[0]), 2);
        TestInitAVLNodeKey(c_ref!(stUserNode[1]), 3);
        TestInitAVLNodeKey(c_ref!(stUserNode[2]), 4);
        TestInitAVLNodeKey(c_ref!(stUserNode[3]), 5);
        TestInitAVLNodeKey(c_ref!(stUserNode[4]), 6);
        TestInitAVLNodeKey(c_ref!(stUserNode[5]), 7);
        TestInitAVLNodeKey(c_ref!(stUserNode[6]), 9);
        TestInitAVLNodeKey(c_ref!(stUserNode[7]), 10);
        c_for!(i = 0; i < uiNodeCount; i += 1; {
            assert!(VOS_AVL_Insert_Or_Find(c_ref!(stTree), c_ref!(stUserNode[i].stNode)) == NULL!());
        });
        c_for!(i = 0; i < uiNodeCount - 1; i += 1; {
            assert_eq!(VOS_AVL_Next(c_ref!(stUserNode[i].stNode)), c_ref!(stUserNode[i + 1]).cast());
        });
        assert!(VOS_AVL_Next(c_ref!(stUserNode[i].stNode)) == NULL!());
        TestCheckAVLTree(
            c_ref!(stTree),
            auiPreOrder.cast(),
            auiInOrder.cast(),
            uiNodeCount,
        );
        test_no_memory_leak!();
    }
    #[test]
    fn UT_VOS_AVL_Prev001() {
        let mut stUserNode: AvlUserStruct = Default::default();
        TestInitAVLNodeKey(c_ref!(stUserNode), 1);
        assert!(VOS_AVL_Prev(NULL!()) == NULL!());
        assert!(VOS_AVL_Prev(c_ref!(stUserNode.stNode)) == NULL!());
        test_no_memory_leak!();
    }
    #[test]
    fn UT_VOS_AVL_Prev002() {
        let mut stTree: AVL_TREE = Default::default();
        let mut stUserNode: Array<AvlUserStruct, 4> = Default::default();
        let mut auiPreOrder: Array<VOS_UINT32, 3> = arr![2, 1, 3];
        let mut auiInOrder: Array<VOS_UINT32, 3> = arr![1, 2, 3];
        VOS_AVL_INIT_TREE!(stTree, func!(AVL_TestCompareFn));
        TestInitAVLNodeKey(c_ref!(stUserNode[0]), 1);
        TestInitAVLNodeKey(c_ref!(stUserNode[1]), 2);
        TestInitAVLNodeKey(c_ref!(stUserNode[2]), 3);
        TestInitAVLNodeKey(c_ref!(stUserNode[3]), (-1).cast());
        assert!(VOS_AVL_Insert_Or_Find(c_ref!(stTree), c_ref!(stUserNode[0].stNode)) == NULL!());
        assert!(VOS_AVL_Insert_Or_Find(c_ref!(stTree), c_ref!(stUserNode[1].stNode)) == NULL!());
        assert!(VOS_AVL_Insert_Or_Find(c_ref!(stTree), c_ref!(stUserNode[2].stNode)) == NULL!());
        TestCheckAVLTree(c_ref!(stTree), auiPreOrder.cast(), auiInOrder.cast(), 3);
        assert!(VOS_AVL_Prev(c_ref!(stUserNode[3].stNode)) == NULL!());
        test_no_memory_leak!();
    }
    #[test]
    fn UT_VOS_AVL_Prev003() {
        let mut stTree: AVL_TREE = Default::default();
        let mut stUserNode: Array<AvlUserStruct, 8> = Default::default();
        let mut auiPreOrder: Array<VOS_UINT32, 8> = arr![5, 3, 2, 4, 7, 6, 9, 10];
        let mut auiInOrder: Array<VOS_UINT32, 8> = arr![2, 3, 4, 5, 6, 7, 9, 10];
        let mut uiNodeCount = 8;
        let mut i: VOS_UINT32;
        VOS_AVL_INIT_TREE!(stTree, func!(AVL_TestCompareFn));
        TestInitAVLNodeKey(c_ref!(stUserNode[0]), 2);
        TestInitAVLNodeKey(c_ref!(stUserNode[1]), 3);
        TestInitAVLNodeKey(c_ref!(stUserNode[2]), 4);
        TestInitAVLNodeKey(c_ref!(stUserNode[3]), 5);
        TestInitAVLNodeKey(c_ref!(stUserNode[4]), 6);
        TestInitAVLNodeKey(c_ref!(stUserNode[5]), 7);
        TestInitAVLNodeKey(c_ref!(stUserNode[6]), 9);
        TestInitAVLNodeKey(c_ref!(stUserNode[7]), 10);
        c_for!(i = 0; i < uiNodeCount; i += 1; {
            assert!(VOS_AVL_Insert_Or_Find(c_ref!(stTree), c_ref!(stUserNode[i].stNode)) == NULL!());
        });
        TestCheckAVLTree(
            c_ref!(stTree),
            auiPreOrder.cast(),
            auiInOrder.cast(),
            uiNodeCount,
        );
        assert!(VOS_AVL_Prev(c_ref!(stUserNode[0].stNode)) == NULL!());
        c_for!(i = 1; i < uiNodeCount; i += 1; {
            assert_eq!(VOS_AVL_Prev(c_ref!(stUserNode[i].stNode)), c_ref!(stUserNode[i - 1]).cast());
        });
        TestCheckAVLTree(
            c_ref!(stTree),
            auiPreOrder.cast(),
            auiInOrder.cast(),
            uiNodeCount,
        );
        assert!(VOS_AVL_Prev(c_ref!(stUserNode[0].stNode)) == NULL!());
        c_for!(i = 1; i < uiNodeCount; i += 1; {
            assert_eq!(VOS_AVL_Prev(c_ref!(stUserNode[i].stNode)), c_ref!(stUserNode[i - 1]).cast());
        });
        TestCheckAVLTree(
            c_ref!(stTree),
            auiPreOrder.cast(),
            auiInOrder.cast(),
            uiNodeCount,
        );
        test_no_memory_leak!();
    }
    #[test]
    fn UT_VOS_AVL_Find_Or_Find_Next001() {
        let mut ulVar: VOS_UINT32;
        let mut stTree: AVL_TREE = Default::default();
        let mut pstNode: Ptr<AVL_NODE> = NULL!();
        let mut stNode1: AvlUserStruct = Default::default();
        let mut stNode2: AvlUserStruct = Default::default();
        let mut stNode3: AvlUserStruct = Default::default();
        let mut stNode4: AvlUserStruct = Default::default();
        VOS_AVL_INIT_TREE!(stTree, func!(AVL_TestCompareFn));
        TestInitAVLNodeKey(c_ref!(stNode1), 9);
        assert_eq!(1.as_bool(), VOS_AVL_INSERT!(stTree, stNode1.stNode));
        ulVar = 9;
        if VOS_AVL_FIND!(stTree, c_ref!(ulVar).cast()) == NULL!() {
            assert!(VOS_FALSE!().as_bool());
        }
        TestInitAVLNodeKey(c_ref!(stNode2), 8);
        assert_eq!(1.as_bool(), VOS_AVL_INSERT!(stTree, stNode2.stNode));
        ulVar = 8;
        if VOS_AVL_FIND!(stTree, c_ref!(ulVar).cast()) == NULL!() {
            assert!(VOS_FALSE!().as_bool());
        }
        TestInitAVLNodeKey(c_ref!(stNode3), 11);
        assert_eq!(1.as_bool(), VOS_AVL_INSERT!(stTree, stNode3.stNode));
        ulVar = 8;
        stNode4 = *(VOS_AVL_FIND_OR_FIND_NEXT!(stTree, c_ref!(ulVar).cast())
            .cast::<Ptr<AvlUserStruct>>());
        pstNode = c_ref!(stNode4.stNode);
        if 8 != *(pstNode.pKey.cast::<Ptr<VOS_UINT32>>()) {
            assert!(VOS_FALSE!().as_bool());
        }
        test_no_memory_leak!();
    }
    #[test]
    fn UT_VOS_AVL_Find_Or_Find_Next002() {
        let mut ulVar: VOS_UINT32;
        let mut stTree: AVL_TREE = Default::default();
        let mut stNode1: AvlUserStruct = Default::default();
        let mut stNode2: AvlUserStruct = Default::default();
        let mut stNode3: AvlUserStruct = Default::default();
        let mut stNode4: AvlUserStruct = Default::default();
        VOS_AVL_INIT_TREE!(stTree, func!(AVL_TestCompareFn));
        TestInitAVLNodeKey(c_ref!(stNode1), 9);
        assert_eq!(1.as_bool(), VOS_AVL_INSERT!(stTree, stNode1.stNode));
        ulVar = 9;
        if VOS_AVL_FIND!(stTree, c_ref!(ulVar).cast()) == NULL!() {
            assert!(VOS_FALSE!().as_bool());
        }
        TestInitAVLNodeKey(c_ref!(stNode2), 8);
        assert_eq!(1.as_bool(), VOS_AVL_INSERT!(stTree, stNode2.stNode));
        ulVar = 8;
        if VOS_AVL_FIND!(stTree, c_ref!(ulVar).cast()) == NULL!() {
            assert!(VOS_FALSE!().as_bool());
        }
        TestInitAVLNodeKey(c_ref!(stNode3), 11);
        assert_eq!(1.as_bool(), VOS_AVL_INSERT!(stTree, stNode3.stNode));
        ulVar = 11;
        if VOS_AVL_FIND!(stTree, c_ref!(ulVar).cast()) == NULL!() {
            assert!(VOS_FALSE!().as_bool());
        }
        ulVar = 9;
        stNode4 = *(VOS_AVL_FIND_OR_FIND_NEXT!(stTree, c_ref!(ulVar).cast())
            .cast::<Ptr<AvlUserStruct>>());
        if 9 != *(stNode4.stNode.pKey.cast::<Ptr<VOS_UINT32>>()) {
            assert!(VOS_FALSE!().as_bool());
        }
        test_no_memory_leak!();
    }
    #[test]
    fn UT_VOS_AVL_Find_Or_Find_Next003() {
        let mut ulVar: VOS_UINT32;
        let mut stTree: AVL_TREE = Default::default();
        let mut stNode1: AvlUserStruct = Default::default();
        let mut stNode2: AvlUserStruct = Default::default();
        let mut stNode3: AvlUserStruct = Default::default();
        let mut stNode4: AvlUserStruct = Default::default();
        VOS_AVL_INIT_TREE!(stTree, func!(AVL_TestCompareFn));
        TestInitAVLNodeKey(c_ref!(stNode1), 9);
        assert_eq!(1.as_bool(), VOS_AVL_INSERT!(stTree, stNode1.stNode));
        ulVar = 9;
        if VOS_AVL_FIND!(stTree, c_ref!(ulVar).cast()) == NULL!() {
            assert!(VOS_FALSE!().as_bool());
        }
        TestInitAVLNodeKey(c_ref!(stNode2), 8);
        assert_eq!(1.as_bool(), VOS_AVL_INSERT!(stTree, stNode2.stNode));
        ulVar = 8;
        if VOS_AVL_FIND!(stTree, c_ref!(ulVar).cast()) == NULL!() {
            assert!(VOS_FALSE!().as_bool());
        }
        TestInitAVLNodeKey(c_ref!(stNode3), 11);
        assert_eq!(1.as_bool(), VOS_AVL_INSERT!(stTree, stNode3.stNode));
        ulVar = 11;
        if VOS_AVL_FIND!(stTree, c_ref!(ulVar).cast()) == NULL!() {
            assert!(VOS_FALSE!().as_bool());
        }
        ulVar = 12;
        if VOS_AVL_FIND_OR_FIND_NEXT!(stTree, c_ref!(ulVar).cast()) != NULL!() {
            stNode4 = *(VOS_AVL_FIND_OR_FIND_NEXT!(stTree, c_ref!(ulVar).cast())
                .cast::<Ptr<AvlUserStruct>>());
            assert!(VOS_FALSE!().as_bool());
        }
        test_no_memory_leak!();
    }
    #[test]
    fn UT_VOS_AVL_Find_Or_Find_Next004() {
        let mut ulVar: VOS_UINT32;
        let mut stTree: AVL_TREE = Default::default();
        let mut stNode1: AvlUserStruct = Default::default();
        let mut stNode2: AvlUserStruct = Default::default();
        let mut stNode3: AvlUserStruct = Default::default();
        let mut stNode4: AvlUserStruct = Default::default();
        VOS_AVL_INIT_TREE!(stTree, func!(AVL_TestCompareFn));
        TestInitAVLNodeKey(c_ref!(stNode1), 9);
        assert_eq!(1.as_bool(), VOS_AVL_INSERT!(stTree, stNode1.stNode));
        ulVar = 9;
        if VOS_AVL_FIND!(stTree, c_ref!(ulVar).cast()) == NULL!() {
            assert!(VOS_FALSE!().as_bool());
        }
        TestInitAVLNodeKey(c_ref!(stNode2), 8);
        assert_eq!(1.as_bool(), VOS_AVL_INSERT!(stTree, stNode2.stNode));
        ulVar = 8;
        if VOS_AVL_FIND!(stTree, c_ref!(ulVar).cast()) == NULL!() {
            assert!(VOS_FALSE!().as_bool());
        }
        TestInitAVLNodeKey(c_ref!(stNode3), 11);
        assert_eq!(1.as_bool(), VOS_AVL_INSERT!(stTree, stNode3.stNode));
        ulVar = 11;
        if VOS_AVL_FIND!(stTree, c_ref!(ulVar).cast()) == NULL!() {
            assert!(VOS_FALSE!().as_bool());
        }
        VOS_AVL_DELETE!(stTree, stNode2.stNode);
        ulVar = 8;
        if VOS_AVL_FIND!(stTree, c_ref!(ulVar).cast()) != NULL!() {
            assert!(VOS_FALSE!().as_bool());
        }
        TestInitAVLNodeKey(c_ref!(stNode2), 12);
        assert_eq!(1.as_bool(), VOS_AVL_INSERT!(stTree, stNode2.stNode));
        ulVar = 12;
        if VOS_AVL_FIND!(stTree, c_ref!(ulVar).cast()) == NULL!() {
            assert!(VOS_FALSE!().as_bool());
        }
        ulVar = 12;
        if VOS_AVL_FIND_OR_FIND_NEXT!(stTree, c_ref!(ulVar).cast()) != NULL!() {
            stNode4 = *(VOS_AVL_FIND_OR_FIND_NEXT!(stTree, c_ref!(ulVar).cast())
                .cast::<Ptr<AvlUserStruct>>());
            if 12 != *(stNode4.stNode.pKey.cast::<Ptr<VOS_UINT32>>()) {
                assert!(VOS_FALSE!().as_bool());
            }
        } else {
            assert!(VOS_FALSE!().as_bool());
        }
        test_no_memory_leak!();
    }
    #[test]
    fn UT_VOS_AVL_Find_Or_Find_Next005() {
        let mut ulVar: VOS_UINT32;
        let mut stTree: AVL_TREE = Default::default();
        let mut stNode1: AvlUserStruct = Default::default();
        let mut stNode2: AvlUserStruct = Default::default();
        let mut stNode3: AvlUserStruct = Default::default();
        let mut stNode4: AvlUserStruct = Default::default();
        VOS_AVL_INIT_TREE!(stTree, func!(AVL_TestCompareFn));
        TestInitAVLNodeKey(c_ref!(stNode1), 9);
        assert_eq!(1.as_bool(), VOS_AVL_INSERT!(stTree, stNode1.stNode));
        ulVar = 9;
        if VOS_AVL_FIND!(stTree, c_ref!(ulVar).cast()) == NULL!() {
            assert!(VOS_FALSE!().as_bool());
        }
        TestInitAVLNodeKey(c_ref!(stNode2), 8);
        assert_eq!(1.as_bool(), VOS_AVL_INSERT!(stTree, stNode2.stNode));
        ulVar = 8;
        if VOS_AVL_FIND!(stTree, c_ref!(ulVar).cast()) == NULL!() {
            assert!(VOS_FALSE!().as_bool());
        }
        TestInitAVLNodeKey(c_ref!(stNode3), 11);
        assert_eq!(1.as_bool(), VOS_AVL_INSERT!(stTree, stNode3.stNode));
        ulVar = 11;
        if VOS_AVL_FIND!(stTree, c_ref!(ulVar).cast()) == NULL!() {
            assert!(VOS_FALSE!().as_bool());
        }
        VOS_AVL_DELETE!(stTree, stNode2.stNode);
        ulVar = 8;
        if VOS_AVL_FIND_OR_FIND_NEXT!(stTree, c_ref!(ulVar).cast()) != NULL!() {
            stNode4 = *(VOS_AVL_FIND_OR_FIND_NEXT!(stTree, c_ref!(ulVar).cast())
                .cast::<Ptr<AvlUserStruct>>());
            if 9 != *(stNode4.stNode.pKey.cast::<Ptr<VOS_UINT32>>()) {
                assert!(VOS_FALSE!().as_bool());
            }
        } else {
            assert!(VOS_FALSE!().as_bool());
        }
        test_no_memory_leak!();
    }
    #[test]
    fn UT_VOS_AVL_Find_Or_Find_Next006() {
        let mut ulVar: VOS_UINT32;
        let mut stTree: AVL_TREE = Default::default();
        VOS_AVL_INIT_TREE!(stTree, func!(AVL_TestCompareFn));
        ulVar = 8;
        if VOS_AVL_FIND_OR_FIND_NEXT!(stTree, c_ref!(ulVar).cast()) != NULL!() {
            VOS_AVL_FIND_OR_FIND_NEXT!(stTree, c_ref!(ulVar).cast());
            assert!(VOS_FALSE!().as_bool());
        }
        test_no_memory_leak!();
    }
    #[test]
    fn UT_VOS_AVL_Find_Or_Find_Next007() {
        let mut ulVar: VOS_UINT32;
        let mut stTree: AVL_TREE = Default::default();
        let mut stNode1: AvlUserStruct = Default::default();
        let mut stNode2: AvlUserStruct = Default::default();
        let mut stNode3: AvlUserStruct = Default::default();
        let mut stNode4: AvlUserStruct = Default::default();
        VOS_AVL_INIT_TREE!(stTree, func!(AVL_TestCompareFn));
        TestInitAVLNodeKey(c_ref!(stNode1), 9);
        assert_eq!(1.as_bool(), VOS_AVL_INSERT!(stTree, stNode1.stNode));
        ulVar = 9;
        if VOS_AVL_FIND!(stTree, c_ref!(ulVar).cast()) == NULL!() {
            assert!(VOS_FALSE!().as_bool());
        }
        TestInitAVLNodeKey(c_ref!(stNode2), 8);
        assert_eq!(1.as_bool(), VOS_AVL_INSERT!(stTree, stNode2.stNode));
        ulVar = 8;
        if VOS_AVL_FIND!(stTree, c_ref!(ulVar).cast()) == NULL!() {
            assert!(VOS_FALSE!().as_bool());
        }
        TestInitAVLNodeKey(c_ref!(stNode3), 11);
        assert_eq!(1.as_bool(), VOS_AVL_INSERT!(stTree, stNode3.stNode));
        ulVar = 11;
        if VOS_AVL_FIND!(stTree, c_ref!(ulVar).cast()) == NULL!() {
            assert!(VOS_FALSE!().as_bool());
        }
        ulVar = 11;
        if VOS_AVL_FIND_OR_FIND_NEXT!(stTree, c_ref!(ulVar).cast()) != NULL!() {
            stNode4 = *(VOS_AVL_FIND_OR_FIND_NEXT!(stTree, c_ref!(ulVar).cast())
                .cast::<Ptr<AvlUserStruct>>());
            if 11 != *(stNode4.stNode.pKey.cast::<Ptr<VOS_UINT32>>()) {
                assert!(VOS_FALSE!().as_bool());
            }
        } else {
            assert!(VOS_FALSE!().as_bool());
        }
        test_no_memory_leak!();
    }
    type VOS_INTPTR = usize;
    #[derive(Default, Clone, Copy)]
    struct tagUserStruct {
        pub ulID: VOS_UINT32,
        pub iKey: VOS_UINT32,
        pub stNode: AVL3_NODE,
    }
    type UserStruct = tagUserStruct;
    const g_usKeyoffset: u16 = {
        let s: UserStruct = unsafe { std::mem::zeroed() };
        let s_ptr = &s as *const UserStruct as *const u8;
        let iKey_ptr = &s.iKey as *const VOS_UINT32 as *const u8;
        let diff = unsafe { iKey_ptr.offset_from(s_ptr) };
        diff as u16
    };
    const g_usNodeoffset: u16 = {
        let s: UserStruct = unsafe { std::mem::zeroed() };
        let s_ptr = &s as *const UserStruct as *const u8;
        let stNode_ptr = &s.stNode as *const AVL3_NODE as *const u8;
        let diff = unsafe { stNode_ptr.offset_from(s_ptr) };
        diff as u16
    };
    fn AVL3_CompareFn(mut pKey1: VoidPtr, mut pKey2: VoidPtr) -> i32 {
        if pKey1 == NULL!() || pKey2 == NULL!() {
            return 0;
        }
        (*(pKey1.cast::<Ptr<i32>>())) - (*(pKey2.cast::<Ptr<i32>>()))
    }
    fn TestInitTreeInfo(mut pstTreeInfo: Ptr<AVL3_TREE_INFO>) {
        if pstTreeInfo == NULL!() {
            return;
        }
        pstTreeInfo.usKeyOffset = g_usKeyoffset;
        pstTreeInfo.pfCompare = func!(AVL3_CompareFn);
        pstTreeInfo.usNodeOffset = g_usNodeoffset;
    }
    fn TestInitAVL3NodeKey(mut pstUserNode: Ptr<UserStruct>, mut uiKey: VOS_UINT32) {
        if pstUserNode == NULL!() {
            return;
        }
        VOS_AVL3_INIT_NODE!(pstUserNode.stNode);
        pstUserNode.iKey = uiKey;
    }
    fn AVL3PreOrderSearch(
        mut pstNode: Ptr<AVL3_NODE>,
        mut puiOrderArr: Ptr<VOS_UINT32>,
        mut puiNodeNum: Ptr<VOS_UINT32>,
    ) {
        let mut uiNodeSum = *puiNodeNum;
        let mut puiNextOrderAddr = puiOrderArr;
        if pstNode == NULL!() || *puiNodeNum == 0 {
            return;
        }
        puiNextOrderAddr[0] = *((pstNode.cast::<Ptr<u8>>() - g_usNodeoffset + g_usKeyoffset)
            .cast::<Ptr<VOS_UINT32>>());
        *puiNodeNum -= 1;
        puiNextOrderAddr = puiOrderArr + 1;
        AVL3PreOrderSearch(pstNode.pstLeft, puiNextOrderAddr, puiNodeNum);
        puiNextOrderAddr = puiOrderArr + uiNodeSum - *puiNodeNum;
        AVL3PreOrderSearch(pstNode.pstRight, puiNextOrderAddr, puiNodeNum);
    }
    fn AVL3InOrderSearch(
        mut pstNode: Ptr<AVL3_NODE>,
        mut puiOrderArr: Ptr<VOS_UINT32>,
        mut puiNodeNum: Ptr<VOS_UINT32>,
    ) {
        let mut uiNodeSum = *puiNodeNum;
        let mut puiNextOrderAddr = puiOrderArr;
        if pstNode == NULL!() || *puiNodeNum == 0 {
            return;
        }
        AVL3InOrderSearch(pstNode.pstLeft, puiNextOrderAddr, puiNodeNum);
        puiNextOrderAddr = puiOrderArr + uiNodeSum - *puiNodeNum;
        puiNextOrderAddr[0] = *((pstNode.cast::<Ptr<u8>>() - g_usNodeoffset + g_usKeyoffset)
            .cast::<Ptr<VOS_UINT32>>());
        *puiNodeNum -= 1;
        puiNextOrderAddr += 1;
        AVL3InOrderSearch(pstNode.pstRight, puiNextOrderAddr, puiNodeNum);
    }
    fn TestCheckAVL3Tree(
        mut pstTree: Ptr<AVL3_TREE>,
        mut puiPreOrderNode: Ptr<VOS_UINT32>,
        mut puiInOrderNode: Ptr<VOS_UINT32>,
        mut uiNodeNum: VOS_UINT32,
    ) {
        let mut puiNowPreOrderNode: Ptr<VOS_UINT32> = NULL!();
        let mut puiNowInOrderNode: Ptr<VOS_UINT32> = NULL!();
        let mut uiPreOrderNum = uiNodeNum;
        let mut uiInOrderNum = uiNodeNum;
        let mut i: VOS_UINT32;
        puiNowPreOrderNode = c_malloc!(c_sizeof!(VOS_UINT32) * uiNodeNum);
        puiNowInOrderNode = c_malloc!(c_sizeof!(VOS_UINT32) * uiNodeNum);
        AVL3PreOrderSearch(pstTree.pstRoot, puiNowPreOrderNode, c_ref!(uiPreOrderNum));
        AVL3InOrderSearch(pstTree.pstRoot, puiNowInOrderNode, c_ref!(uiInOrderNum));
        c_for!(i = 0; i < uiNodeNum; i += 1; {
            assert_eq!(puiPreOrderNode[i], puiNowPreOrderNode[i]);
            assert_eq!(puiInOrderNode[i], puiNowInOrderNode[i]);
        });
        c_free!(puiNowPreOrderNode);
        c_free!(puiNowInOrderNode);
    }
    #[test]
    fn UT_VOS_AVL3_Insert_Or_Find001() {
        let mut stTree: AVL3_TREE = Default::default();
        let mut stUserNode: UserStruct = Default::default();
        let mut stTreeInfo: AVL3_TREE_INFO = Default::default();
        TestInitTreeInfo(c_ref!(stTreeInfo));
        VOS_AVL3_INIT_TREE!(stTree, stTreeInfo);
        TestInitAVL3NodeKey(c_ref!(stUserNode), 1);
        assert!(
            VOS_AVL3_Insert_Or_Find(NULL!(), c_ref!(stUserNode.stNode), c_ref!(stTreeInfo))
                == NULL!()
        );
        assert!(VOS_AVL3_Insert_Or_Find(c_ref!(stTree), NULL!(), c_ref!(stTreeInfo)) == NULL!());
        assert!(
            VOS_AVL3_Insert_Or_Find(c_ref!(stTree), c_ref!(stUserNode.stNode), NULL!()) == NULL!()
        );
        test_no_memory_leak!();
    }
    #[test]
    fn UT_VOS_AVL3_Insert_Or_Find002() {
        let mut stTree: AVL3_TREE = Default::default();
        let mut stUserNode: Array<UserStruct, 3> = Default::default();
        let mut stTreeInfo: AVL3_TREE_INFO = Default::default();
        let mut auiPreOrder: Array<VOS_UINT32, 3> = arr![2, 1, 3];
        let mut auiInOrder: Array<VOS_UINT32, 3> = arr![1, 2, 3];
        TestInitTreeInfo(c_ref!(stTreeInfo));
        VOS_AVL3_INIT_TREE!(stTree, stTreeInfo);
        TestInitAVL3NodeKey(c_ref!(stUserNode[0]), 1);
        TestInitAVL3NodeKey(c_ref!(stUserNode[1]), 2);
        TestInitAVL3NodeKey(c_ref!(stUserNode[2]), 3);
        assert!(
            VOS_AVL3_Insert_Or_Find(
                c_ref!(stTree),
                c_ref!(stUserNode[0].stNode),
                c_ref!(stTreeInfo)
            ) == NULL!()
        );
        assert!(
            VOS_AVL3_Insert_Or_Find(
                c_ref!(stTree),
                c_ref!(stUserNode[1].stNode),
                c_ref!(stTreeInfo)
            ) == NULL!()
        );
        assert!(
            VOS_AVL3_Insert_Or_Find(
                c_ref!(stTree),
                c_ref!(stUserNode[2].stNode),
                c_ref!(stTreeInfo)
            ) == NULL!()
        );
        TestCheckAVL3Tree(c_ref!(stTree), auiPreOrder.cast(), auiInOrder.cast(), 3);
        test_no_memory_leak!();
    }
    #[test]
    fn UT_VOS_AVL3_Insert_Or_Find003() {
        let mut stTree: AVL3_TREE = Default::default();
        let mut stUserNode: Array<UserStruct, 3> = Default::default();
        let mut auiPreOrder: Array<VOS_UINT32, 3> = arr![2, 1, 3];
        let mut auiInOrder: Array<VOS_UINT32, 3> = arr![1, 2, 3];
        let mut stTreeInfo: AVL3_TREE_INFO = Default::default();
        TestInitTreeInfo(c_ref!(stTreeInfo));
        VOS_AVL3_INIT_TREE!(stTree, stTreeInfo);
        TestInitAVL3NodeKey(c_ref!(stUserNode[0]), 3);
        TestInitAVL3NodeKey(c_ref!(stUserNode[1]), 2);
        TestInitAVL3NodeKey(c_ref!(stUserNode[2]), 1);
        assert!(
            VOS_AVL3_Insert_Or_Find(
                c_ref!(stTree),
                c_ref!(stUserNode[0].stNode),
                c_ref!(stTreeInfo)
            ) == NULL!()
        );
        assert!(
            VOS_AVL3_Insert_Or_Find(
                c_ref!(stTree),
                c_ref!(stUserNode[1].stNode),
                c_ref!(stTreeInfo)
            ) == NULL!()
        );
        assert!(
            VOS_AVL3_Insert_Or_Find(
                c_ref!(stTree),
                c_ref!(stUserNode[2].stNode),
                c_ref!(stTreeInfo)
            ) == NULL!()
        );
        TestCheckAVL3Tree(c_ref!(stTree), auiPreOrder.cast(), auiInOrder.cast(), 3);
        test_no_memory_leak!();
    }
    #[test]
    fn UT_VOS_AVL3_Insert_Or_Find004() {
        let mut stTree: AVL3_TREE = Default::default();
        let mut stUserNode: Array<UserStruct, 3> = Default::default();
        let mut auiPreOrder: Array<VOS_UINT32, 3> = arr![2, 1, 3];
        let mut auiInOrder: Array<VOS_UINT32, 3> = arr![1, 2, 3];
        let mut stTreeInfo: AVL3_TREE_INFO = Default::default();
        TestInitTreeInfo(c_ref!(stTreeInfo));
        VOS_AVL3_INIT_TREE!(stTree, stTreeInfo);
        TestInitAVL3NodeKey(c_ref!(stUserNode[0]), 3);
        TestInitAVL3NodeKey(c_ref!(stUserNode[1]), 1);
        TestInitAVL3NodeKey(c_ref!(stUserNode[2]), 2);
        assert!(
            VOS_AVL3_Insert_Or_Find(
                c_ref!(stTree),
                c_ref!(stUserNode[0].stNode),
                c_ref!(stTreeInfo)
            ) == NULL!()
        );
        assert!(
            VOS_AVL3_Insert_Or_Find(
                c_ref!(stTree),
                c_ref!(stUserNode[1].stNode),
                c_ref!(stTreeInfo)
            ) == NULL!()
        );
        assert!(
            VOS_AVL3_Insert_Or_Find(
                c_ref!(stTree),
                c_ref!(stUserNode[2].stNode),
                c_ref!(stTreeInfo)
            ) == NULL!()
        );
        TestCheckAVL3Tree(c_ref!(stTree), auiPreOrder.cast(), auiInOrder.cast(), 3);
        test_no_memory_leak!();
    }
    #[test]
    fn UT_VOS_AVL3_Insert_Or_Find005() {
        let mut stTree: AVL3_TREE = Default::default();
        let mut stUserNode: Array<UserStruct, 3> = Default::default();
        let mut auiPreOrder: Array<VOS_UINT32, 3> = arr![2, 1, 3];
        let mut auiInOrder: Array<VOS_UINT32, 3> = arr![1, 2, 3];
        let mut stTreeInfo: AVL3_TREE_INFO = Default::default();
        TestInitTreeInfo(c_ref!(stTreeInfo));
        VOS_AVL3_INIT_TREE!(stTree, stTreeInfo);
        TestInitAVL3NodeKey(c_ref!(stUserNode[0]), 1);
        TestInitAVL3NodeKey(c_ref!(stUserNode[1]), 3);
        TestInitAVL3NodeKey(c_ref!(stUserNode[2]), 2);
        assert!(
            VOS_AVL3_Insert_Or_Find(
                c_ref!(stTree),
                c_ref!(stUserNode[0].stNode),
                c_ref!(stTreeInfo)
            ) == NULL!()
        );
        assert!(
            VOS_AVL3_Insert_Or_Find(
                c_ref!(stTree),
                c_ref!(stUserNode[1].stNode),
                c_ref!(stTreeInfo)
            ) == NULL!()
        );
        assert!(
            VOS_AVL3_Insert_Or_Find(
                c_ref!(stTree),
                c_ref!(stUserNode[2].stNode),
                c_ref!(stTreeInfo)
            ) == NULL!()
        );
        TestCheckAVL3Tree(c_ref!(stTree), auiPreOrder.cast(), auiInOrder.cast(), 3);
        test_no_memory_leak!();
    }
    #[test]
    fn UT_VOS_AVL3_Insert_Or_Find006() {
        let mut stTree: AVL3_TREE = Default::default();
        let mut stUserNode: Array<UserStruct, 4> = Default::default();
        let mut auiPreOrder: Array<VOS_UINT32, 3> = arr![2, 1, 3];
        let mut auiInOrder: Array<VOS_UINT32, 3> = arr![1, 2, 3];
        let mut stTreeInfo: AVL3_TREE_INFO = Default::default();
        TestInitTreeInfo(c_ref!(stTreeInfo));
        VOS_AVL3_INIT_TREE!(stTree, stTreeInfo);
        TestInitAVL3NodeKey(c_ref!(stUserNode[0]), 3);
        TestInitAVL3NodeKey(c_ref!(stUserNode[1]), 1);
        TestInitAVL3NodeKey(c_ref!(stUserNode[2]), 2);
        TestInitAVL3NodeKey(c_ref!(stUserNode[3]), 1);
        assert!(
            VOS_AVL3_Insert_Or_Find(
                c_ref!(stTree),
                c_ref!(stUserNode[0].stNode),
                c_ref!(stTreeInfo)
            ) == NULL!()
        );
        assert!(
            VOS_AVL3_Insert_Or_Find(
                c_ref!(stTree),
                c_ref!(stUserNode[1].stNode),
                c_ref!(stTreeInfo)
            ) == NULL!()
        );
        assert!(
            VOS_AVL3_Insert_Or_Find(
                c_ref!(stTree),
                c_ref!(stUserNode[2].stNode),
                c_ref!(stTreeInfo)
            ) == NULL!()
        );
        assert_eq!(
            VOS_AVL3_Insert_Or_Find(
                c_ref!(stTree),
                c_ref!(stUserNode[3].stNode),
                c_ref!(stTreeInfo)
            ),
            c_ref!(stUserNode[1]).cast()
        );
        TestCheckAVL3Tree(c_ref!(stTree), auiPreOrder.cast(), auiInOrder.cast(), 3);
        test_no_memory_leak!();
    }
    #[test]
    fn UT_VOS_AVL3_Delete001() {
        let mut stTree: AVL3_TREE = Default::default();
        let mut stUserNode: Array<UserStruct, 3> = Default::default();
        let mut auiPreOrder: Array<VOS_UINT32, 3> = arr![2, 1, 3];
        let mut auiInOrder: Array<VOS_UINT32, 3> = arr![1, 2, 3];
        let mut stTreeInfo: AVL3_TREE_INFO = Default::default();
        TestInitTreeInfo(c_ref!(stTreeInfo));
        VOS_AVL3_INIT_TREE!(stTree, stTreeInfo);
        TestInitAVL3NodeKey(c_ref!(stUserNode[0]), 3);
        TestInitAVL3NodeKey(c_ref!(stUserNode[1]), 1);
        TestInitAVL3NodeKey(c_ref!(stUserNode[2]), 2);
        assert!(
            VOS_AVL3_Insert_Or_Find(
                c_ref!(stTree),
                c_ref!(stUserNode[0].stNode),
                c_ref!(stTreeInfo)
            ) == NULL!()
        );
        assert!(
            VOS_AVL3_Insert_Or_Find(
                c_ref!(stTree),
                c_ref!(stUserNode[1].stNode),
                c_ref!(stTreeInfo)
            ) == NULL!()
        );
        assert!(
            VOS_AVL3_Insert_Or_Find(
                c_ref!(stTree),
                c_ref!(stUserNode[2].stNode),
                c_ref!(stTreeInfo)
            ) == NULL!()
        );
        VOS_AVL3_Delete(NULL!(), c_ref!(stUserNode[2].stNode));
        VOS_AVL3_Delete(c_ref!(stTree), NULL!());
        TestCheckAVL3Tree(c_ref!(stTree), auiPreOrder.cast(), auiInOrder.cast(), 3);
        test_no_memory_leak!();
    }
    #[test]
    fn UT_VOS_AVL3_Delete002() {
        let mut stTree: AVL3_TREE = Default::default();
        let mut stUserNode: Array<UserStruct, 4> = Default::default();
        let mut auiPreOrder: Array<VOS_UINT32, 4> = arr![3, 2, 4, 0];
        let mut auiInOrder: Array<VOS_UINT32, 4> = arr![2, 3, 4, 0];
        let mut stTreeInfo: AVL3_TREE_INFO = Default::default();
        TestInitTreeInfo(c_ref!(stTreeInfo));
        VOS_AVL3_INIT_TREE!(stTree, stTreeInfo);
        TestInitAVL3NodeKey(c_ref!(stUserNode[0]), 3);
        TestInitAVL3NodeKey(c_ref!(stUserNode[1]), 1);
        TestInitAVL3NodeKey(c_ref!(stUserNode[2]), 4);
        TestInitAVL3NodeKey(c_ref!(stUserNode[3]), 2);
        assert!(
            VOS_AVL3_Insert_Or_Find(
                c_ref!(stTree),
                c_ref!(stUserNode[0].stNode),
                c_ref!(stTreeInfo)
            ) == NULL!()
        );
        assert!(
            VOS_AVL3_Insert_Or_Find(
                c_ref!(stTree),
                c_ref!(stUserNode[1].stNode),
                c_ref!(stTreeInfo)
            ) == NULL!()
        );
        assert!(
            VOS_AVL3_Insert_Or_Find(
                c_ref!(stTree),
                c_ref!(stUserNode[2].stNode),
                c_ref!(stTreeInfo)
            ) == NULL!()
        );
        assert!(
            VOS_AVL3_Insert_Or_Find(
                c_ref!(stTree),
                c_ref!(stUserNode[3].stNode),
                c_ref!(stTreeInfo)
            ) == NULL!()
        );
        VOS_AVL3_Delete(c_ref!(stTree), c_ref!(stUserNode[1].stNode));
        TestCheckAVL3Tree(c_ref!(stTree), auiPreOrder.cast(), auiInOrder.cast(), 3);
        test_no_memory_leak!();
    }
    #[test]
    fn UT_VOS_AVL3_Delete003() {
        let mut stTree: AVL3_TREE = Default::default();
        let mut stUserNode: Array<UserStruct, 4> = Default::default();
        let mut auiPreOrder: Array<VOS_UINT32, 4> = arr![2, 1, 3, 0];
        let mut auiInOrder: Array<VOS_UINT32, 4> = arr![1, 2, 3, 0];
        let mut stTreeInfo: AVL3_TREE_INFO = Default::default();
        TestInitTreeInfo(c_ref!(stTreeInfo));
        VOS_AVL3_INIT_TREE!(stTree, stTreeInfo);
        TestInitAVL3NodeKey(c_ref!(stUserNode[0]), 2);
        TestInitAVL3NodeKey(c_ref!(stUserNode[1]), 1);
        TestInitAVL3NodeKey(c_ref!(stUserNode[2]), 4);
        TestInitAVL3NodeKey(c_ref!(stUserNode[3]), 3);
        assert!(
            VOS_AVL3_Insert_Or_Find(
                c_ref!(stTree),
                c_ref!(stUserNode[0].stNode),
                c_ref!(stTreeInfo)
            ) == NULL!()
        );
        assert!(
            VOS_AVL3_Insert_Or_Find(
                c_ref!(stTree),
                c_ref!(stUserNode[1].stNode),
                c_ref!(stTreeInfo)
            ) == NULL!()
        );
        assert!(
            VOS_AVL3_Insert_Or_Find(
                c_ref!(stTree),
                c_ref!(stUserNode[2].stNode),
                c_ref!(stTreeInfo)
            ) == NULL!()
        );
        assert!(
            VOS_AVL3_Insert_Or_Find(
                c_ref!(stTree),
                c_ref!(stUserNode[3].stNode),
                c_ref!(stTreeInfo)
            ) == NULL!()
        );
        VOS_AVL3_Delete(c_ref!(stTree), c_ref!(stUserNode[2].stNode));
        TestCheckAVL3Tree(c_ref!(stTree), auiPreOrder.cast(), auiInOrder.cast(), 3);
        test_no_memory_leak!();
    }
    #[test]
    fn UT_VOS_AVL3_Delete004() {
        let mut stTree: AVL3_TREE = Default::default();
        let mut stUserNode: Array<UserStruct, 3> = Default::default();
        let mut auiPreOrder: Array<VOS_UINT32, 3> = arr![1, 3, 0];
        let mut auiInOrder: Array<VOS_UINT32, 3> = arr![1, 3, 0];
        let mut stTreeInfo: AVL3_TREE_INFO = Default::default();
        TestInitTreeInfo(c_ref!(stTreeInfo));
        VOS_AVL3_INIT_TREE!(stTree, stTreeInfo);
        TestInitAVL3NodeKey(c_ref!(stUserNode[0]), 2);
        TestInitAVL3NodeKey(c_ref!(stUserNode[1]), 1);
        TestInitAVL3NodeKey(c_ref!(stUserNode[2]), 3);
        assert!(
            VOS_AVL3_Insert_Or_Find(
                c_ref!(stTree),
                c_ref!(stUserNode[0].stNode),
                c_ref!(stTreeInfo)
            ) == NULL!()
        );
        assert!(
            VOS_AVL3_Insert_Or_Find(
                c_ref!(stTree),
                c_ref!(stUserNode[1].stNode),
                c_ref!(stTreeInfo)
            ) == NULL!()
        );
        assert!(
            VOS_AVL3_Insert_Or_Find(
                c_ref!(stTree),
                c_ref!(stUserNode[2].stNode),
                c_ref!(stTreeInfo)
            ) == NULL!()
        );
        VOS_AVL3_Delete(c_ref!(stTree), c_ref!(stUserNode[0].stNode));
        TestCheckAVL3Tree(c_ref!(stTree), auiPreOrder.cast(), auiInOrder.cast(), 2);
        test_no_memory_leak!();
    }
    #[test]
    fn UT_VOS_AVL3_Delete005() {
        let mut stTree: AVL3_TREE = Default::default();
        let mut stUserNode: Array<UserStruct, 9> = Default::default();
        let mut auiPreOrderBefore: Array<VOS_UINT32, 9> = arr![5, 3, 2, 4, 9, 7, 6, 8, 10];
        let mut auiInOrderBefore: Array<VOS_UINT32, 9> = arr![2, 3, 4, 5, 6, 7, 8, 9, 10];
        let mut auiPreOrderAfter1: Array<VOS_UINT32, 9> = arr![5, 3, 2, 4, 8, 7, 6, 10, 0];
        let mut auiInOrderAfter1: Array<VOS_UINT32, 9> = arr![2, 3, 4, 5, 6, 7, 8, 10, 0];
        let mut auiPreOrderAfter2: Array<VOS_UINT32, 9> = arr![6, 3, 2, 4, 8, 7, 10, 0, 0];
        let mut auiInOrderAfter2: Array<VOS_UINT32, 9> = arr![2, 3, 4, 6, 7, 8, 10, 0, 0];
        let mut uiNodeCount: VOS_UINT32 = 9;
        let mut i: VOS_UINT32;
        let mut stTreeInfo: AVL3_TREE_INFO = Default::default();
        TestInitTreeInfo(c_ref!(stTreeInfo));
        VOS_AVL3_INIT_TREE!(stTree, stTreeInfo);
        TestInitAVL3NodeKey(c_ref!(stUserNode[0]), 5);
        TestInitAVL3NodeKey(c_ref!(stUserNode[1]), 3);
        TestInitAVL3NodeKey(c_ref!(stUserNode[2]), 9);
        TestInitAVL3NodeKey(c_ref!(stUserNode[3]), 2);
        TestInitAVL3NodeKey(c_ref!(stUserNode[4]), 4);
        TestInitAVL3NodeKey(c_ref!(stUserNode[5]), 7);
        TestInitAVL3NodeKey(c_ref!(stUserNode[6]), 10);
        TestInitAVL3NodeKey(c_ref!(stUserNode[7]), 6);
        TestInitAVL3NodeKey(c_ref!(stUserNode[8]), 8);
        c_for!(i = 0; i < uiNodeCount; i += 1; {
            assert!(VOS_AVL3_Insert_Or_Find(c_ref!(stTree), c_ref!(stUserNode[i].stNode), c_ref!(stTreeInfo)) == NULL!());
        });
        TestCheckAVL3Tree(
            c_ref!(stTree),
            auiPreOrderBefore.cast(),
            auiInOrderBefore.cast(),
            uiNodeCount,
        );
        VOS_AVL3_Delete(c_ref!(stTree), c_ref!(stUserNode[2].stNode));
        TestCheckAVL3Tree(
            c_ref!(stTree),
            auiPreOrderAfter1.cast(),
            auiInOrderAfter1.cast(),
            uiNodeCount - 1,
        );
        VOS_AVL3_Delete(c_ref!(stTree), c_ref!(stUserNode[0].stNode));
        TestCheckAVL3Tree(
            c_ref!(stTree),
            auiPreOrderAfter2.cast(),
            auiInOrderAfter2.cast(),
            uiNodeCount - 2,
        );
        test_no_memory_leak!();
    }
    #[test]
    fn UT_VOS_AVL3_Delete006() {
        let mut stTree: AVL3_TREE = Default::default();
        let mut stUserNode: Array<UserStruct, 8> = Default::default();
        let mut auiPreOrderBefore: Array<VOS_UINT32, 8> = arr![5, 3, 2, 4, 9, 7, 6, 10];
        let mut auiInOrderBefore: Array<VOS_UINT32, 8> = arr![2, 3, 4, 5, 6, 7, 9, 10];
        let mut auiPreOrderAfter: Array<VOS_UINT32, 8> = arr![5, 3, 2, 4, 7, 6, 10, 0];
        let mut auiInOrderAfter: Array<VOS_UINT32, 8> = arr![2, 3, 4, 5, 6, 7, 10, 0];
        let mut uiNodeCount: VOS_UINT32 = 8;
        let mut i: VOS_UINT32;
        let mut stTreeInfo: AVL3_TREE_INFO = Default::default();
        TestInitTreeInfo(c_ref!(stTreeInfo));
        VOS_AVL3_INIT_TREE!(stTree, stTreeInfo);
        TestInitAVL3NodeKey(c_ref!(stUserNode[0]), 5);
        TestInitAVL3NodeKey(c_ref!(stUserNode[1]), 3);
        TestInitAVL3NodeKey(c_ref!(stUserNode[2]), 9);
        TestInitAVL3NodeKey(c_ref!(stUserNode[3]), 2);
        TestInitAVL3NodeKey(c_ref!(stUserNode[4]), 4);
        TestInitAVL3NodeKey(c_ref!(stUserNode[5]), 7);
        TestInitAVL3NodeKey(c_ref!(stUserNode[6]), 10);
        TestInitAVL3NodeKey(c_ref!(stUserNode[7]), 6);
        c_for!(i = 0; i < uiNodeCount; i += 1; {
            assert!(VOS_AVL3_Insert_Or_Find(c_ref!(stTree), c_ref!(stUserNode[i].stNode), c_ref!(stTreeInfo)) == NULL!());
        });
        TestCheckAVL3Tree(
            c_ref!(stTree),
            auiPreOrderBefore.cast(),
            auiInOrderBefore.cast(),
            uiNodeCount,
        );
        VOS_AVL3_Delete(c_ref!(stTree), c_ref!(stUserNode[2].stNode));
        TestCheckAVL3Tree(
            c_ref!(stTree),
            auiPreOrderAfter.cast(),
            auiInOrderAfter.cast(),
            uiNodeCount - 1,
        );
        test_no_memory_leak!();
    }
    #[test]
    fn UT_VOS_AVL3_Delete007() {
        let mut stTree: AVL3_TREE = Default::default();
        let mut stUserNode: Array<UserStruct, 9> = Default::default();
        let mut auiPreOrderBefore: Array<VOS_UINT32, 9> = arr![5, 3, 2, 4, 9, 7, 12, 11, 13];
        let mut auiInOrderBefore: Array<VOS_UINT32, 9> = arr![2, 3, 4, 5, 7, 9, 11, 12, 13];
        let mut auiPreOrderAfter: Array<VOS_UINT32, 9> = arr![5, 3, 2, 4, 11, 7, 12, 13, 0];
        let mut auiInOrderAfter: Array<VOS_UINT32, 9> = arr![2, 3, 4, 5, 7, 11, 12, 13, 0];
        let mut uiNodeCount: VOS_UINT32 = 9;
        let mut i: VOS_UINT32;
        let mut stTreeInfo: AVL3_TREE_INFO = Default::default();
        TestInitTreeInfo(c_ref!(stTreeInfo));
        VOS_AVL3_INIT_TREE!(stTree, stTreeInfo);
        TestInitAVL3NodeKey(c_ref!(stUserNode[0]), 5);
        TestInitAVL3NodeKey(c_ref!(stUserNode[1]), 3);
        TestInitAVL3NodeKey(c_ref!(stUserNode[2]), 9);
        TestInitAVL3NodeKey(c_ref!(stUserNode[3]), 2);
        TestInitAVL3NodeKey(c_ref!(stUserNode[4]), 4);
        TestInitAVL3NodeKey(c_ref!(stUserNode[5]), 7);
        TestInitAVL3NodeKey(c_ref!(stUserNode[6]), 12);
        TestInitAVL3NodeKey(c_ref!(stUserNode[7]), 11);
        TestInitAVL3NodeKey(c_ref!(stUserNode[8]), 13);
        c_for!(i = 0; i < uiNodeCount; i += 1; {
            assert!(VOS_AVL3_Insert_Or_Find(c_ref!(stTree), c_ref!(stUserNode[i].stNode), c_ref!(stTreeInfo)) == NULL!());
        });
        TestCheckAVL3Tree(
            c_ref!(stTree),
            auiPreOrderBefore.cast(),
            auiInOrderBefore.cast(),
            uiNodeCount,
        );
        VOS_AVL3_Delete(c_ref!(stTree), c_ref!(stUserNode[2].stNode));
        TestCheckAVL3Tree(
            c_ref!(stTree),
            auiPreOrderAfter.cast(),
            auiInOrderAfter.cast(),
            uiNodeCount - 1,
        );
        test_no_memory_leak!();
    }
    #[test]
    fn UT_VOS_AVL3_Delete008() {
        let mut stTree: AVL3_TREE = Default::default();
        let mut stUserNode: Array<UserStruct, 14> = Default::default();
        let mut auiPreOrderBefore: Array<VOS_UINT32, 12> =
            arr![9, 3, 2, 1, 7, 5, 6, 8, 11, 10, 12, 13];
        let mut auiInOrderBefore: Array<VOS_UINT32, 12> =
            arr![1, 2, 3, 5, 6, 7, 8, 9, 10, 11, 12, 13];
        let mut auiPreOrderAfter: Array<VOS_UINT32, 12> =
            arr![9, 5, 2, 1, 7, 6, 8, 11, 10, 12, 13, 0];
        let mut auiInOrderAfter: Array<VOS_UINT32, 12> =
            arr![1, 2, 5, 6, 7, 8, 9, 10, 11, 12, 13, 0];
        let mut uiNodeCount: VOS_UINT32 = 12;
        let mut i: VOS_UINT32;
        let mut stTreeInfo: AVL3_TREE_INFO = Default::default();
        TestInitTreeInfo(c_ref!(stTreeInfo));
        VOS_AVL3_INIT_TREE!(stTree, stTreeInfo);
        TestInitAVL3NodeKey(c_ref!(stUserNode[0]), 9);
        TestInitAVL3NodeKey(c_ref!(stUserNode[1]), 3);
        TestInitAVL3NodeKey(c_ref!(stUserNode[2]), 11);
        TestInitAVL3NodeKey(c_ref!(stUserNode[3]), 2);
        TestInitAVL3NodeKey(c_ref!(stUserNode[4]), 7);
        TestInitAVL3NodeKey(c_ref!(stUserNode[5]), 10);
        TestInitAVL3NodeKey(c_ref!(stUserNode[6]), 12);
        TestInitAVL3NodeKey(c_ref!(stUserNode[7]), 1);
        TestInitAVL3NodeKey(c_ref!(stUserNode[8]), 5);
        TestInitAVL3NodeKey(c_ref!(stUserNode[9]), 8);
        TestInitAVL3NodeKey(c_ref!(stUserNode[10]), 13);
        TestInitAVL3NodeKey(c_ref!(stUserNode[11]), 6);
        TestInitAVL3NodeKey(c_ref!(stUserNode[12]), 4);
        TestInitAVL3NodeKey(c_ref!(stUserNode[13]), 3);
        c_for!(i = 0; i < uiNodeCount; i += 1; {
            assert!(VOS_AVL3_Insert_Or_Find(c_ref!(stTree), c_ref!(stUserNode[i].stNode), c_ref!(stTreeInfo)) == NULL!());
        });
        TestCheckAVL3Tree(
            c_ref!(stTree),
            auiPreOrderBefore.cast(),
            auiInOrderBefore.cast(),
            uiNodeCount,
        );
        VOS_AVL3_Delete(c_ref!(stTree), c_ref!(stUserNode[1].stNode));
        TestCheckAVL3Tree(
            c_ref!(stTree),
            auiPreOrderAfter.cast(),
            auiInOrderAfter.cast(),
            uiNodeCount - 1,
        );
        assert!(
            VOS_AVL3_Insert_Or_Find(
                c_ref!(stTree),
                c_ref!(stUserNode[12].stNode),
                c_ref!(stTreeInfo)
            ) == NULL!()
        );
        assert!(
            VOS_AVL3_Insert_Or_Find(
                c_ref!(stTree),
                c_ref!(stUserNode[13].stNode),
                c_ref!(stTreeInfo)
            ) == NULL!()
        );
        VOS_AVL3_Delete(c_ref!(stTree), c_ref!(stUserNode[8].stNode));
        VOS_AVL3_Delete(c_ref!(stTree), c_ref!(stUserNode[2].stNode));
        VOS_AVL3_Delete(c_ref!(stTree), c_ref!(stUserNode[11].stNode));
        VOS_AVL3_Delete(c_ref!(stTree), c_ref!(stUserNode[9].stNode));
        VOS_AVL3_Delete(c_ref!(stTree), c_ref!(stUserNode[4].stNode));
        VOS_AVL3_Delete(c_ref!(stTree), c_ref!(stUserNode[7].stNode));
        VOS_AVL3_Delete(c_ref!(stTree), c_ref!(stUserNode[3].stNode));
        VOS_AVL3_Delete(c_ref!(stTree), c_ref!(stUserNode[12].stNode));
        VOS_AVL3_Delete(c_ref!(stTree), c_ref!(stUserNode[13].stNode));
        VOS_AVL3_Delete(c_ref!(stTree), c_ref!(stUserNode[6].stNode));
        VOS_AVL3_Delete(c_ref!(stTree), c_ref!(stUserNode[10].stNode));
        VOS_AVL3_Delete(c_ref!(stTree), c_ref!(stUserNode[0].stNode));
        VOS_AVL3_Delete(c_ref!(stTree), c_ref!(stUserNode[5].stNode));
        test_no_memory_leak!();
    }
    #[test]
    fn UT_VOS_AVL3_Find001() {
        let mut stTree: AVL3_TREE = Default::default();
        let mut uiKey: VOS_UINT32 = 0;
        let mut stTreeInfo: AVL3_TREE_INFO = Default::default();
        TestInitTreeInfo(c_ref!(stTreeInfo));
        VOS_AVL3_INIT_TREE!(stTree, stTreeInfo);
        assert!(VOS_AVL3_Find(NULL!(), c_ref!(uiKey).cast(), c_ref!(stTreeInfo)) == NULL!());
        assert!(VOS_AVL3_Find(c_ref!(stTree), c_ref!(uiKey).cast(), NULL!()) == NULL!());
        assert!(VOS_AVL3_Find(c_ref!(stTree), c_ref!(uiKey).cast(), c_ref!(stTreeInfo)) == NULL!());
        test_no_memory_leak!();
    }
    #[test]
    fn UT_VOS_AVL3_Find002() {
        let mut stTree: AVL3_TREE = Default::default();
        let mut stUserNode: Array<UserStruct, 3> = Default::default();
        let mut auiPreOrder: Array<VOS_UINT32, 3> = arr![2, 1, 3];
        let mut auiInOrder: Array<VOS_UINT32, 3> = arr![1, 2, 3];
        let mut uiKey: VOS_UINT32 = 4;
        let mut stTreeInfo: AVL3_TREE_INFO = Default::default();
        TestInitTreeInfo(c_ref!(stTreeInfo));
        VOS_AVL3_INIT_TREE!(stTree, stTreeInfo);
        TestInitAVL3NodeKey(c_ref!(stUserNode[0]), 1);
        TestInitAVL3NodeKey(c_ref!(stUserNode[1]), 2);
        TestInitAVL3NodeKey(c_ref!(stUserNode[2]), 3);
        assert!(
            VOS_AVL3_Insert_Or_Find(
                c_ref!(stTree),
                c_ref!(stUserNode[0].stNode),
                c_ref!(stTreeInfo)
            ) == NULL!()
        );
        assert!(
            VOS_AVL3_Insert_Or_Find(
                c_ref!(stTree),
                c_ref!(stUserNode[1].stNode),
                c_ref!(stTreeInfo)
            ) == NULL!()
        );
        assert!(
            VOS_AVL3_Insert_Or_Find(
                c_ref!(stTree),
                c_ref!(stUserNode[2].stNode),
                c_ref!(stTreeInfo)
            ) == NULL!()
        );
        TestCheckAVL3Tree(c_ref!(stTree), auiPreOrder.cast(), auiInOrder.cast(), 3);
        assert!(VOS_AVL3_Find(c_ref!(stTree), c_ref!(uiKey).cast(), c_ref!(stTreeInfo)) == NULL!());
        test_no_memory_leak!();
    }
    #[test]
    fn UT_VOS_AVL3_Find003() {
        let mut stTree: AVL3_TREE = Default::default();
        let mut stUserNode: Array<UserStruct, 8> = Default::default();
        let mut uiNodeCount: VOS_UINT32 = 8;
        let mut i: VOS_UINT32;
        let mut uiKey: VOS_UINT32;
        let mut stTreeInfo: AVL3_TREE_INFO = Default::default();
        TestInitTreeInfo(c_ref!(stTreeInfo));
        VOS_AVL3_INIT_TREE!(stTree, stTreeInfo);
        TestInitAVL3NodeKey(c_ref!(stUserNode[0]), 5);
        TestInitAVL3NodeKey(c_ref!(stUserNode[1]), 3);
        TestInitAVL3NodeKey(c_ref!(stUserNode[2]), 9);
        TestInitAVL3NodeKey(c_ref!(stUserNode[3]), 2);
        TestInitAVL3NodeKey(c_ref!(stUserNode[4]), 4);
        TestInitAVL3NodeKey(c_ref!(stUserNode[5]), 7);
        TestInitAVL3NodeKey(c_ref!(stUserNode[6]), 10);
        TestInitAVL3NodeKey(c_ref!(stUserNode[7]), 6);
        c_for!(i = 0; i < uiNodeCount; i += 1; {
            assert!(VOS_AVL3_Insert_Or_Find(c_ref!(stTree), c_ref!(stUserNode[i].stNode), c_ref!(stTreeInfo)) == NULL!());
        });
        c_for!(i = 0; i < uiNodeCount; i += 1; {
            uiKey = stUserNode[i].iKey;
            assert_eq!(VOS_AVL3_Find(c_ref!(stTree), c_ref!(uiKey).cast(), c_ref!(stTreeInfo)), c_ref!(stUserNode[i]).cast());
        });
        test_no_memory_leak!();
    }
    #[test]
    fn UT_VOS_AVL3_Next001() {
        let mut stTreeInfo: AVL3_TREE_INFO = Default::default();
        let mut stUserNode: UserStruct = Default::default();
        TestInitTreeInfo(c_ref!(stTreeInfo));
        TestInitAVL3NodeKey(c_ref!(stUserNode), 1);
        assert!(VOS_AVL3_Next(NULL!(), c_ref!(stTreeInfo)) == NULL!());
        assert!(VOS_AVL3_Next(c_ref!(stUserNode.stNode), NULL!()) == NULL!());
        test_no_memory_leak!();
    }
    #[test]
    fn UT_VOS_AVL3_Next002() {
        let mut stTree: AVL3_TREE = Default::default();
        let mut stUserNode: Array<UserStruct, 4> = Default::default();
        let mut auiPreOrder: Array<VOS_UINT32, 3> = arr![2, 1, 3];
        let mut auiInOrder: Array<VOS_UINT32, 3> = arr![1, 2, 3];
        let mut stTreeInfo: AVL3_TREE_INFO = Default::default();
        TestInitTreeInfo(c_ref!(stTreeInfo));
        VOS_AVL3_INIT_TREE!(stTree, stTreeInfo);
        TestInitAVL3NodeKey(c_ref!(stUserNode[0]), 1);
        TestInitAVL3NodeKey(c_ref!(stUserNode[1]), 2);
        TestInitAVL3NodeKey(c_ref!(stUserNode[2]), 3);
        TestInitAVL3NodeKey(c_ref!(stUserNode[3]), 4);
        assert!(
            VOS_AVL3_Insert_Or_Find(
                c_ref!(stTree),
                c_ref!(stUserNode[0].stNode),
                c_ref!(stTreeInfo)
            ) == NULL!()
        );
        assert!(
            VOS_AVL3_Insert_Or_Find(
                c_ref!(stTree),
                c_ref!(stUserNode[1].stNode),
                c_ref!(stTreeInfo)
            ) == NULL!()
        );
        assert!(
            VOS_AVL3_Insert_Or_Find(
                c_ref!(stTree),
                c_ref!(stUserNode[2].stNode),
                c_ref!(stTreeInfo)
            ) == NULL!()
        );
        TestCheckAVL3Tree(c_ref!(stTree), auiPreOrder.cast(), auiInOrder.cast(), 3);
        assert!(VOS_AVL3_Next(c_ref!(stUserNode[3].stNode), c_ref!(stTreeInfo)) == NULL!());
        test_no_memory_leak!();
    }
    #[test]
    fn UT_VOS_AVL3_Next003() {
        let mut stTree: AVL3_TREE = Default::default();
        let mut stUserNode: Array<UserStruct, 8> = Default::default();
        let mut auiPreOrder: Array<VOS_UINT32, 8> = arr![5, 3, 2, 4, 7, 6, 9, 10];
        let mut auiInOrder: Array<VOS_UINT32, 8> = arr![2, 3, 4, 5, 6, 7, 9, 10];
        let mut uiNodeCount: VOS_UINT32 = 8;
        let mut i: VOS_UINT32;
        let mut stTreeInfo: AVL3_TREE_INFO = Default::default();
        TestInitTreeInfo(c_ref!(stTreeInfo));
        VOS_AVL3_INIT_TREE!(stTree, stTreeInfo);
        TestInitAVL3NodeKey(c_ref!(stUserNode[0]), 2);
        TestInitAVL3NodeKey(c_ref!(stUserNode[1]), 3);
        TestInitAVL3NodeKey(c_ref!(stUserNode[2]), 4);
        TestInitAVL3NodeKey(c_ref!(stUserNode[3]), 5);
        TestInitAVL3NodeKey(c_ref!(stUserNode[4]), 6);
        TestInitAVL3NodeKey(c_ref!(stUserNode[5]), 7);
        TestInitAVL3NodeKey(c_ref!(stUserNode[6]), 9);
        TestInitAVL3NodeKey(c_ref!(stUserNode[7]), 10);
        c_for!(i = 0; i < uiNodeCount; i += 1; {
            assert!(VOS_AVL3_Insert_Or_Find(c_ref!(stTree), c_ref!(stUserNode[i].stNode), c_ref!(stTreeInfo)) == NULL!());
        });
        c_for!(i = 0; i < uiNodeCount - 1; i += 1; {
            assert_eq!(VOS_AVL3_Next(c_ref!(stUserNode[i].stNode), c_ref!(stTreeInfo)), c_ref!(stUserNode[i + 1]).cast());
        });
        assert!(VOS_AVL3_Next(c_ref!(stUserNode[i].stNode), c_ref!(stTreeInfo)) == NULL!());
        TestCheckAVL3Tree(
            c_ref!(stTree),
            auiPreOrder.cast(),
            auiInOrder.cast(),
            uiNodeCount,
        );
        test_no_memory_leak!();
    }
    #[test]
    fn UT_VOS_AVL3_Prev001() {
        let mut stUserNode: UserStruct = Default::default();
        let mut stTreeInfo: AVL3_TREE_INFO = Default::default();
        TestInitTreeInfo(c_ref!(stTreeInfo));
        TestInitAVL3NodeKey(c_ref!(stUserNode), 1);
        assert!(VOS_AVL3_Prev(NULL!(), c_ref!(stTreeInfo)) == NULL!());
        assert!(VOS_AVL3_Prev(c_ref!(stUserNode.stNode), NULL!()) == NULL!());
        test_no_memory_leak!();
    }
    #[test]
    fn UT_VOS_AVL3_Prev002() {
        let mut stTree: AVL3_TREE = Default::default();
        let mut stUserNode: Array<UserStruct, 4> = Default::default();
        let mut auiPreOrder: Array<VOS_UINT32, 3> = arr![2, 1, 3];
        let mut auiInOrder: Array<VOS_UINT32, 3> = arr![1, 2, 3];
        let mut stTreeInfo: AVL3_TREE_INFO = Default::default();
        TestInitTreeInfo(c_ref!(stTreeInfo));
        VOS_AVL3_INIT_TREE!(stTree, stTreeInfo);
        TestInitAVL3NodeKey(c_ref!(stUserNode[0]), 1);
        TestInitAVL3NodeKey(c_ref!(stUserNode[1]), 2);
        TestInitAVL3NodeKey(c_ref!(stUserNode[2]), 3);
        TestInitAVL3NodeKey(c_ref!(stUserNode[3]), (-1).cast());
        assert!(
            VOS_AVL3_Insert_Or_Find(
                c_ref!(stTree),
                c_ref!(stUserNode[0].stNode),
                c_ref!(stTreeInfo)
            ) == NULL!()
        );
        assert!(
            VOS_AVL3_Insert_Or_Find(
                c_ref!(stTree),
                c_ref!(stUserNode[1].stNode),
                c_ref!(stTreeInfo)
            ) == NULL!()
        );
        assert!(
            VOS_AVL3_Insert_Or_Find(
                c_ref!(stTree),
                c_ref!(stUserNode[2].stNode),
                c_ref!(stTreeInfo)
            ) == NULL!()
        );
        TestCheckAVL3Tree(c_ref!(stTree), auiPreOrder.cast(), auiInOrder.cast(), 3);
        assert!(VOS_AVL3_Prev(c_ref!(stUserNode[3].stNode), c_ref!(stTreeInfo)) == NULL!());
        test_no_memory_leak!();
    }
    #[test]
    fn UT_VOS_AVL3_Prev003() {
        let mut stTree: AVL3_TREE = Default::default();
        let mut stUserNode: Array<UserStruct, 8> = Default::default();
        let mut auiPreOrder: Array<VOS_UINT32, 8> = arr![5, 3, 2, 4, 7, 6, 9, 10];
        let mut auiInOrder: Array<VOS_UINT32, 8> = arr![2, 3, 4, 5, 6, 7, 9, 10];
        let mut uiNodeCount: VOS_UINT32 = 8;
        let mut i: VOS_UINT32;
        let mut stTreeInfo: AVL3_TREE_INFO = Default::default();
        TestInitTreeInfo(c_ref!(stTreeInfo));
        VOS_AVL3_INIT_TREE!(stTree, stTreeInfo);
        TestInitAVL3NodeKey(c_ref!(stUserNode[0]), 2);
        TestInitAVL3NodeKey(c_ref!(stUserNode[1]), 3);
        TestInitAVL3NodeKey(c_ref!(stUserNode[2]), 4);
        TestInitAVL3NodeKey(c_ref!(stUserNode[3]), 5);
        TestInitAVL3NodeKey(c_ref!(stUserNode[4]), 6);
        TestInitAVL3NodeKey(c_ref!(stUserNode[5]), 7);
        TestInitAVL3NodeKey(c_ref!(stUserNode[6]), 9);
        TestInitAVL3NodeKey(c_ref!(stUserNode[7]), 10);
        c_for!(i = 0; i < uiNodeCount; i += 1; {
            assert!(VOS_AVL3_Insert_Or_Find(c_ref!(stTree), c_ref!(stUserNode[i].stNode), c_ref!(stTreeInfo)) == NULL!());
        });
        c_for!(i = 1; i < uiNodeCount; i += 1; {
            assert_eq!(VOS_AVL3_Prev(c_ref!(stUserNode[i].stNode), c_ref!(stTreeInfo)), c_ref!(stUserNode[i - 1]).cast());
        });
        assert!(VOS_AVL3_Prev(c_ref!(stUserNode[0].stNode), c_ref!(stTreeInfo)) == NULL!());
        TestCheckAVL3Tree(
            c_ref!(stTree),
            auiPreOrder.cast(),
            auiInOrder.cast(),
            uiNodeCount,
        );
        test_no_memory_leak!();
    }
    #[test]
    fn UT_VOS_AVL3_First001() {
        let mut stTree: AVL3_TREE = Default::default();
        let mut stTreeInfo: AVL3_TREE_INFO = Default::default();
        TestInitTreeInfo(c_ref!(stTreeInfo));
        VOS_AVL3_INIT_TREE!(stTree, stTreeInfo);
        assert!(VOS_AVL3_First(c_ref!(stTree), NULL!()) == NULL!());
        assert!(VOS_AVL3_First(NULL!(), c_ref!(stTreeInfo)) == NULL!());
        test_no_memory_leak!();
    }
    #[test]
    fn UT_VOS_AVL3_First002() {
        let mut stTree: AVL3_TREE = Default::default();
        let mut stTreeInfo: AVL3_TREE_INFO = Default::default();
        TestInitTreeInfo(c_ref!(stTreeInfo));
        VOS_AVL3_INIT_TREE!(stTree, stTreeInfo);
        assert!(VOS_AVL3_First(c_ref!(stTree), c_ref!(stTreeInfo)) == NULL!());
        test_no_memory_leak!();
    }
    #[test]
    fn UT_VOS_AVL3_First003() {
        let mut stTree: AVL3_TREE = Default::default();
        let mut stUserNode: Array<UserStruct, 8> = Default::default();
        let mut uiNodeCount: VOS_UINT32 = 8;
        let mut i: VOS_UINT32;
        let mut stTreeInfo: AVL3_TREE_INFO = Default::default();
        TestInitTreeInfo(c_ref!(stTreeInfo));
        VOS_AVL3_INIT_TREE!(stTree, stTreeInfo);
        TestInitAVL3NodeKey(c_ref!(stUserNode[0]), 5);
        TestInitAVL3NodeKey(c_ref!(stUserNode[1]), 3);
        TestInitAVL3NodeKey(c_ref!(stUserNode[2]), 4);
        TestInitAVL3NodeKey(c_ref!(stUserNode[3]), 12);
        TestInitAVL3NodeKey(c_ref!(stUserNode[4]), 6);
        TestInitAVL3NodeKey(c_ref!(stUserNode[5]), 7);
        TestInitAVL3NodeKey(c_ref!(stUserNode[6]), 1);
        TestInitAVL3NodeKey(c_ref!(stUserNode[7]), 10);
        c_for!(i = 0; i < uiNodeCount; i += 1; {
            assert!(VOS_AVL3_Insert_Or_Find(c_ref!(stTree), c_ref!(stUserNode[i].stNode), c_ref!(stTreeInfo)) == NULL!());
        });
        assert_eq!(
            VOS_AVL3_First(c_ref!(stTree), c_ref!(stTreeInfo)),
            c_ref!(stUserNode[6]).cast()
        );
        test_no_memory_leak!();
    }
    #[test]
    fn UT_VOS_AVL3_First004() {
        let mut stTree: AVL3_TREE = Default::default();
        let mut stUserNode: Array<UserStruct, 4> = Default::default();
        let mut uiNodeCount: VOS_UINT32 = 4;
        let mut i: VOS_UINT32;
        let mut stTreeInfo: AVL3_TREE_INFO = Default::default();
        TestInitTreeInfo(c_ref!(stTreeInfo));
        VOS_AVL3_INIT_TREE!(stTree, stTreeInfo);
        TestInitAVL3NodeKey(c_ref!(stUserNode[0]), 3);
        TestInitAVL3NodeKey(c_ref!(stUserNode[1]), 1);
        TestInitAVL3NodeKey(c_ref!(stUserNode[2]), 4);
        TestInitAVL3NodeKey(c_ref!(stUserNode[3]), 2);
        c_for!(i = 0; i < uiNodeCount; i += 1; {
            assert!(VOS_AVL3_Insert_Or_Find(c_ref!(stTree), c_ref!(stUserNode[i].stNode), c_ref!(stTreeInfo)) == NULL!());
        });
        assert_eq!(
            VOS_AVL3_First(c_ref!(stTree), c_ref!(stTreeInfo)),
            c_ref!(stUserNode[1]).cast()
        );
        VOS_AVL3_Delete(c_ref!(stTree), c_ref!(stUserNode[1].stNode));
        assert_eq!(
            VOS_AVL3_First(c_ref!(stTree), c_ref!(stTreeInfo)),
            c_ref!(stUserNode[3]).cast()
        );
        assert!(
            VOS_AVL3_Insert_Or_Find(
                c_ref!(stTree),
                c_ref!(stUserNode[1].stNode),
                c_ref!(stTreeInfo)
            ) == NULL!()
        );
        assert_eq!(
            VOS_AVL3_First(c_ref!(stTree), c_ref!(stTreeInfo)),
            c_ref!(stUserNode[1]).cast()
        );
        test_no_memory_leak!();
    }
    #[test]
    fn UT_VOS_AVL3_Last001() {
        let mut stTree: AVL3_TREE = Default::default();
        let mut stTreeInfo: AVL3_TREE_INFO = Default::default();
        TestInitTreeInfo(c_ref!(stTreeInfo));
        VOS_AVL3_INIT_TREE!(stTree, stTreeInfo);
        assert!(VOS_AVL3_Last(c_ref!(stTree), NULL!()) == NULL!());
        assert!(VOS_AVL3_Last(NULL!(), c_ref!(stTreeInfo)) == NULL!());
        test_no_memory_leak!();
    }
    #[test]
    fn UT_VOS_AVL3_Last002() {
        let mut stTree: AVL3_TREE = Default::default();
        let mut stTreeInfo: AVL3_TREE_INFO = Default::default();
        TestInitTreeInfo(c_ref!(stTreeInfo));
        VOS_AVL3_INIT_TREE!(stTree, stTreeInfo);
        assert!(VOS_AVL3_Last(c_ref!(stTree), c_ref!(stTreeInfo)) == NULL!());
        test_no_memory_leak!();
    }
    #[test]
    fn UT_VOS_AVL3_Last003() {
        let mut stTree: AVL3_TREE = Default::default();
        let mut stUserNode: Array<UserStruct, 8> = Default::default();
        let mut uiNodeCount: VOS_UINT32 = 8;
        let mut i: VOS_UINT32;
        let mut stTreeInfo: AVL3_TREE_INFO = Default::default();
        TestInitTreeInfo(c_ref!(stTreeInfo));
        VOS_AVL3_INIT_TREE!(stTree, stTreeInfo);
        TestInitAVL3NodeKey(c_ref!(stUserNode[0]), 5);
        TestInitAVL3NodeKey(c_ref!(stUserNode[1]), 3);
        TestInitAVL3NodeKey(c_ref!(stUserNode[2]), 4);
        TestInitAVL3NodeKey(c_ref!(stUserNode[3]), 12);
        TestInitAVL3NodeKey(c_ref!(stUserNode[4]), 6);
        TestInitAVL3NodeKey(c_ref!(stUserNode[5]), 7);
        TestInitAVL3NodeKey(c_ref!(stUserNode[6]), 1);
        TestInitAVL3NodeKey(c_ref!(stUserNode[7]), 10);
        c_for!(i = 0; i < uiNodeCount; i += 1; {
            assert!(VOS_AVL3_Insert_Or_Find(c_ref!(stTree), c_ref!(stUserNode[i].stNode), c_ref!(stTreeInfo)) == NULL!());
        });
        assert_eq!(
            VOS_AVL3_Last(c_ref!(stTree), c_ref!(stTreeInfo)),
            c_ref!(stUserNode[3]).cast()
        );
        test_no_memory_leak!();
    }
    #[test]
    fn UT_VOS_AVL3_Last004() {
        let mut stTree: AVL3_TREE = Default::default();
        let mut stUserNode: Array<UserStruct, 4> = Default::default();
        let mut uiNodeCount: VOS_UINT32 = 4;
        let mut i: VOS_UINT32;
        let mut stTreeInfo: AVL3_TREE_INFO = Default::default();
        TestInitTreeInfo(c_ref!(stTreeInfo));
        VOS_AVL3_INIT_TREE!(stTree, stTreeInfo);
        TestInitAVL3NodeKey(c_ref!(stUserNode[0]), 2);
        TestInitAVL3NodeKey(c_ref!(stUserNode[1]), 1);
        TestInitAVL3NodeKey(c_ref!(stUserNode[2]), 4);
        TestInitAVL3NodeKey(c_ref!(stUserNode[3]), 3);
        c_for!(i = 0; i < uiNodeCount; i += 1; {
            assert!(VOS_AVL3_Insert_Or_Find(c_ref!(stTree), c_ref!(stUserNode[i].stNode), c_ref!(stTreeInfo)) == NULL!());
        });
        assert_eq!(
            VOS_AVL3_Last(c_ref!(stTree), c_ref!(stTreeInfo)),
            c_ref!(stUserNode[2]).cast()
        );
        VOS_AVL3_Delete(c_ref!(stTree), c_ref!(stUserNode[2].stNode));
        assert_eq!(
            VOS_AVL3_Last(c_ref!(stTree), c_ref!(stTreeInfo)),
            c_ref!(stUserNode[3]).cast()
        );
        assert!(
            VOS_AVL3_Insert_Or_Find(
                c_ref!(stTree),
                c_ref!(stUserNode[2].stNode),
                c_ref!(stTreeInfo)
            ) == NULL!()
        );
        assert_eq!(
            VOS_AVL3_Last(c_ref!(stTree), c_ref!(stTreeInfo)),
            c_ref!(stUserNode[2]).cast()
        );
        test_no_memory_leak!();
    }

    static mut g_ulKeyoffset: u16 = 0;
    static mut g_ulNodeoffset: u16 = 0;

    fn AVL3_CompareFunc(mut pulVal1: VoidPtr, mut pulVal2: VoidPtr) -> i32 {
        if pulVal1 == NULL!() || pulVal2 == NULL!() {
            return 0;
        }
        return *(pulVal1.cast::<Ptr<VOS_UINT32>>()) as i32
            - *(pulVal2.cast::<Ptr<VOS_UINT32>>()) as i32;
    }

    fn VOS_GetAVL3Node(mut pstNode: Ptr<AVL3_NODE>, mut pulTemp: Ptr<VOS_UINT32>) {
        unsafe {
            *pulTemp = *((pstNode.cast::<Ptr<VOS_CHAR>>() - g_ulNodeoffset + g_ulKeyoffset)
                .cast::<Ptr<VOS_UINT32>>());
        }
    }

    #[test]
    fn ITest_AVL3_INIT_NODE_001() {
        let mut stNode1: UserStruct = Default::default();
        VOS_AVL3_INIT_NODE!(stNode1.stNode);
        stNode1.iKey = 9;
        assert!(stNode1.stNode.pstLeft == NULL!());
        assert!(stNode1.stNode.pstRight == NULL!());
        assert!(stNode1.stNode.pstParent == NULL!());
        test_no_memory_leak!();
    }

    #[test]
    fn ITest_AVL3_INIT_TREE_001() {
        unsafe {
            let mut stTree: AVL3_TREE = Default::default();
            let mut stTreeInfo: AVL3_TREE_INFO = Default::default();
            let mut stNode1: UserStruct = Default::default();
            g_ulKeyoffset = (c_ref!(stNode1.iKey).cast::<Ptr<VOS_CHAR>>()
                - c_ref!(stNode1).cast::<Ptr<VOS_CHAR>>()) as u16;
            g_ulNodeoffset = (c_ref!(stNode1.stNode).cast::<Ptr<VOS_CHAR>>()
                - c_ref!(stNode1).cast::<Ptr<VOS_CHAR>>()) as u16;
            stTreeInfo.usKeyOffset = g_ulKeyoffset;
            stTreeInfo.pfCompare = func!(AVL3_CompareFunc);
            stTreeInfo.usNodeOffset = g_ulNodeoffset;
            VOS_AVL3_INIT_TREE!(stTree, stTreeInfo);
            assert!(stTree.pstFirst == NULL!());
            assert!(stTree.pstLast == NULL!());
            assert!(stTree.pstRoot == NULL!());
            test_no_memory_leak!();
        }
    }

    #[test]
    fn ITest_AVL3_INSERT_001() {
        unsafe {
            let mut stTree: AVL3_TREE = Default::default();
            let mut stNode1: UserStruct = Default::default();
            let mut stTreeInfo: AVL3_TREE_INFO = Default::default();
            let mut ulVar: VOS_UINT32;
            g_ulKeyoffset = (c_ref!(stNode1.iKey).cast::<Ptr<VOS_CHAR>>()
                - c_ref!(stNode1).cast::<Ptr<VOS_CHAR>>()) as u16;
            g_ulNodeoffset = (c_ref!(stNode1.stNode).cast::<Ptr<VOS_CHAR>>()
                - c_ref!(stNode1).cast::<Ptr<VOS_CHAR>>()) as u16;
            stTreeInfo.usKeyOffset = g_ulKeyoffset;
            stTreeInfo.pfCompare = func!(AVL3_CompareFunc);
            stTreeInfo.usNodeOffset = g_ulNodeoffset;
            VOS_AVL3_INIT_TREE!(stTree, stTreeInfo);
            VOS_AVL3_INIT_NODE!(stNode1.stNode);
            stNode1.iKey = 2;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode1.stNode, stTreeInfo), true);
            ulVar = 2;
            assert!(VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) != NULL!());
            test_no_memory_leak!();
        }
    }

    #[test]
    fn ITest_AVL3_INSERT_002() {
        unsafe {
            let mut stTree: AVL3_TREE = Default::default();
            let mut stNode1: UserStruct = Default::default();
            let mut stNode2: UserStruct = Default::default();
            let mut stNode3: UserStruct = Default::default();
            let mut stNode4: UserStruct = Default::default();
            let mut stNode5: UserStruct = Default::default();
            let mut stNode6: UserStruct = Default::default();
            let mut stTreeInfo: AVL3_TREE_INFO = Default::default();
            let mut ulVar: VOS_UINT32;
            g_ulKeyoffset = (c_ref!(stNode1.iKey).cast::<Ptr<VOS_CHAR>>()
                - c_ref!(stNode1).cast::<Ptr<VOS_CHAR>>()) as u16;
            g_ulNodeoffset = (c_ref!(stNode1.stNode).cast::<Ptr<VOS_CHAR>>()
                - c_ref!(stNode1).cast::<Ptr<VOS_CHAR>>()) as u16;
            stTreeInfo.usKeyOffset = g_ulKeyoffset;
            stTreeInfo.pfCompare = func!(AVL3_CompareFunc);
            stTreeInfo.usNodeOffset = g_ulNodeoffset;
            VOS_AVL3_INIT_TREE!(stTree, stTreeInfo);
            VOS_AVL3_INIT_NODE!(stNode1.stNode);
            stNode1.iKey = 2;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode1.stNode, stTreeInfo), true);
            ulVar = 2;
            assert!(VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) != NULL!());
            VOS_AVL3_INIT_NODE!(stNode2.stNode);
            stNode2.iKey = 5;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode2.stNode, stTreeInfo), true);
            ulVar = 5;
            assert!(VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) != NULL!());
            VOS_AVL3_INIT_NODE!(stNode3.stNode);
            stNode3.iKey = 16;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode3.stNode, stTreeInfo), true);
            ulVar = 16;
            assert!(VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) != NULL!());
            VOS_AVL3_INIT_NODE!(stNode4.stNode);
            stNode4.iKey = 32;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode4.stNode, stTreeInfo), true);
            ulVar = 32;
            assert!(VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) != NULL!());
            VOS_AVL3_INIT_NODE!(stNode5.stNode);
            stNode5.iKey = 45;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode5.stNode, stTreeInfo), true);
            ulVar = 45;
            assert!(VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) != NULL!());
            VOS_AVL3_INIT_NODE!(stNode6.stNode);
            stNode6.iKey = 72;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode6.stNode, stTreeInfo), true);
            ulVar = 72;
            assert!(VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) != NULL!());
            test_no_memory_leak!();
        }
    }

    #[test]
    fn ITest_AVL3_INSERT_003() {
        unsafe {
            let mut stTree: AVL3_TREE = Default::default();
            let mut stNode1: UserStruct = Default::default();
            let mut stNode2: UserStruct = Default::default();
            let mut stNode3: UserStruct = Default::default();
            let mut stNode4: UserStruct = Default::default();
            let mut stNode5: UserStruct = Default::default();
            let mut stNode6: UserStruct = Default::default();
            let mut stTreeInfo: AVL3_TREE_INFO = Default::default();
            let mut ulVar: VOS_UINT32;
            g_ulKeyoffset = (c_ref!(stNode1.iKey).cast::<Ptr<VOS_CHAR>>()
                - c_ref!(stNode1).cast::<Ptr<VOS_CHAR>>()) as u16;
            g_ulNodeoffset = (c_ref!(stNode1.stNode).cast::<Ptr<VOS_CHAR>>()
                - c_ref!(stNode1).cast::<Ptr<VOS_CHAR>>()) as u16;
            stTreeInfo.usKeyOffset = g_ulKeyoffset;
            stTreeInfo.pfCompare = func!(AVL3_CompareFunc);
            stTreeInfo.usNodeOffset = g_ulNodeoffset;
            VOS_AVL3_INIT_TREE!(stTree, stTreeInfo);
            VOS_AVL3_INIT_NODE!(stNode1.stNode);
            stNode1.iKey = 2;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode1.stNode, stTreeInfo), true);
            ulVar = 2;
            assert!(VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) != NULL!());
            VOS_AVL3_INIT_NODE!(stNode2.stNode);
            stNode2.iKey = 5;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode2.stNode, stTreeInfo), true);
            ulVar = 5;
            assert!(VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) != NULL!());
            VOS_AVL3_INIT_NODE!(stNode3.stNode);
            stNode3.iKey = 16;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode3.stNode, stTreeInfo), true);
            ulVar = 16;
            assert!(VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) != NULL!());
            VOS_AVL3_INIT_NODE!(stNode4.stNode);
            stNode4.iKey = 32;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode4.stNode, stTreeInfo), true);
            ulVar = 32;
            if VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) == NULL!() {
                assert!(false);
            }
            VOS_AVL3_INIT_NODE!(stNode5.stNode);
            stNode5.iKey = 33;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode5.stNode, stTreeInfo), true);
            ulVar = 33;
            assert!(VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) != NULL!());
            VOS_AVL3_INIT_NODE!(stNode6.stNode);
            stNode6.iKey = 72;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode6.stNode, stTreeInfo), true);
            ulVar = 72;
            assert!(VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) != NULL!());
            test_no_memory_leak!();
        }
    }

    #[test]
    fn ITest_AVL3_INSERT_004() {
        unsafe {
            let mut stTree: AVL3_TREE = Default::default();
            let mut stNode1: UserStruct = Default::default();
            let mut stNode2: UserStruct = Default::default();
            let mut stNode3: UserStruct = Default::default();
            let mut stNode4: UserStruct = Default::default();
            let mut stNode5: UserStruct = Default::default();
            let mut stNode6: UserStruct = Default::default();
            let mut stNode7: UserStruct = Default::default();
            let mut stNode8: UserStruct = Default::default();
            let mut stTreeInfo: AVL3_TREE_INFO = Default::default();
            let mut ulVar: VOS_UINT32;
            g_ulKeyoffset = (c_ref!(stNode1.iKey).cast::<Ptr<VOS_CHAR>>()
                - c_ref!(stNode1).cast::<Ptr<VOS_CHAR>>()) as u16;
            g_ulNodeoffset = (c_ref!(stNode1.stNode).cast::<Ptr<VOS_CHAR>>()
                - c_ref!(stNode1).cast::<Ptr<VOS_CHAR>>()) as u16;
            stTreeInfo.usKeyOffset = g_ulKeyoffset;
            stTreeInfo.pfCompare = func!(AVL3_CompareFunc);
            stTreeInfo.usNodeOffset = g_ulNodeoffset;
            VOS_AVL3_INIT_TREE!(stTree, stTreeInfo);
            VOS_AVL3_INIT_NODE!(stNode1.stNode);
            stNode1.iKey = 2;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode1.stNode, stTreeInfo), true);
            ulVar = 2;
            assert!(VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) != NULL!());
            VOS_AVL3_INIT_NODE!(stNode2.stNode);
            stNode2.iKey = 5;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode2.stNode, stTreeInfo), true);
            ulVar = 5;
            assert!(VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) != NULL!());
            VOS_AVL3_INIT_NODE!(stNode3.stNode);
            stNode3.iKey = 12;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode3.stNode, stTreeInfo), true);

            ulVar = 12;
            assert!(VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) != NULL!());
            VOS_AVL3_INIT_NODE!(stNode4.stNode);
            stNode4.iKey = 20;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode4.stNode, stTreeInfo), true);
            ulVar = 20;
            assert!(VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) != NULL!());
            VOS_AVL3_INIT_NODE!(stNode5.stNode);
            stNode5.iKey = 18;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode5.stNode, stTreeInfo), true);
            ulVar = 18;
            assert!(VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) != NULL!());
            VOS_AVL3_INIT_NODE!(stNode6.stNode);
            stNode6.iKey = 19;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode6.stNode, stTreeInfo), true);
            ulVar = 19;

            assert!(VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) != NULL!());
            VOS_AVL3_INIT_NODE!(stNode7.stNode);
            stNode7.iKey = 11;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode7.stNode, stTreeInfo), true);
            ulVar = 11;
            assert!(VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) != NULL!());
            VOS_AVL3_INIT_NODE!(stNode8.stNode);
            stNode8.iKey = 45;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode8.stNode, stTreeInfo), true);
            ulVar = 45;
            assert!(VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) != NULL!());
            test_no_memory_leak!();
        }
    }

    #[test]
    fn ITest_AVL3_INSERT_005() {
        unsafe {
            let mut stTree: AVL3_TREE = Default::default();
            let mut stNode1: UserStruct = Default::default();
            let mut stNode2: UserStruct = Default::default();
            let mut stNode3: UserStruct = Default::default();
            let mut stNode4: UserStruct = Default::default();
            let mut stNode5: UserStruct = Default::default();
            let mut stNode6: UserStruct = Default::default();
            let mut stTreeInfo: AVL3_TREE_INFO = Default::default();
            let mut ulVar: VOS_UINT32;
            g_ulKeyoffset = (c_ref!(stNode1.iKey).cast::<Ptr<VOS_CHAR>>()
                - c_ref!(stNode1).cast::<Ptr<VOS_CHAR>>()) as u16;
            g_ulNodeoffset = (c_ref!(stNode1.stNode).cast::<Ptr<VOS_CHAR>>()
                - c_ref!(stNode1).cast::<Ptr<VOS_CHAR>>()) as u16;
            stTreeInfo.usKeyOffset = g_ulKeyoffset;
            stTreeInfo.pfCompare = func!(AVL3_CompareFunc);
            stTreeInfo.usNodeOffset = g_ulNodeoffset;
            VOS_AVL3_INIT_TREE!(stTree, stTreeInfo);
            VOS_AVL3_INIT_NODE!(stNode1.stNode);
            stNode1.iKey = 9;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode1.stNode, stTreeInfo), true);
            ulVar = 9;
            assert!(VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) != NULL!());
            VOS_AVL3_INIT_NODE!(stNode2.stNode);
            stNode2.iKey = 8;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode2.stNode, stTreeInfo), true);
            ulVar = 8;
            assert!(VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) != NULL!());
            VOS_AVL3_INIT_NODE!(stNode3.stNode);
            stNode3.iKey = 11;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode3.stNode, stTreeInfo), true);
            ulVar = 11;
            assert!(VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) != NULL!());
            VOS_AVL3_INIT_NODE!(stNode4.stNode);
            stNode4.iKey = 7;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode4.stNode, stTreeInfo), true);
            ulVar = 7;
            assert!(VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) != NULL!());
            VOS_AVL3_INIT_NODE!(stNode5.stNode);
            stNode5.iKey = 6;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode5.stNode, stTreeInfo), true);
            ulVar = 6;
            assert!(VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) != NULL!());
            VOS_AVL3_INIT_NODE!(stNode6.stNode);
            stNode6.iKey = 3;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode6.stNode, stTreeInfo), true);
            ulVar = 3;
            assert!(VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) != NULL!());
            test_no_memory_leak!();
        }
    }

    #[test]
    fn ITest_AVL3_INSERT_006() {
        unsafe {
            let mut stTree: AVL3_TREE = Default::default();
            let mut stNode1: UserStruct = Default::default();
            let mut stNode2: UserStruct = Default::default();
            let mut stNode3: UserStruct = Default::default();
            let mut stNode4: UserStruct = Default::default();
            let mut stNode5: UserStruct = Default::default();
            let mut stNode6: UserStruct = Default::default();
            let mut stNode7: UserStruct = Default::default();
            let mut stNode8: UserStruct = Default::default();
            let mut stTreeInfo: AVL3_TREE_INFO = Default::default();
            let mut ulVar: VOS_UINT32;
            g_ulKeyoffset = (c_ref!(stNode1.iKey).cast::<Ptr<VOS_CHAR>>()
                - c_ref!(stNode1).cast::<Ptr<VOS_CHAR>>()) as u16;
            g_ulNodeoffset = (c_ref!(stNode1.stNode).cast::<Ptr<VOS_CHAR>>()
                - c_ref!(stNode1).cast::<Ptr<VOS_CHAR>>()) as u16;
            stTreeInfo.usKeyOffset = g_ulKeyoffset;
            stTreeInfo.pfCompare = func!(AVL3_CompareFunc);
            stTreeInfo.usNodeOffset = g_ulNodeoffset;
            VOS_AVL3_INIT_TREE!(stTree, stTreeInfo);
            VOS_AVL3_INIT_NODE!(stNode1.stNode);
            stNode1.iKey = 9;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode1.stNode, stTreeInfo), true);
            ulVar = 9;
            assert!(VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) != NULL!());
            VOS_AVL3_INIT_NODE!(stNode2.stNode);
            stNode2.iKey = 8;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode2.stNode, stTreeInfo), true);
            ulVar = 8;
            assert!(VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) != NULL!());
            VOS_AVL3_INIT_NODE!(stNode3.stNode);
            stNode3.iKey = 11;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode3.stNode, stTreeInfo), true);
            ulVar = 11;
            assert!(VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) != NULL!());
            VOS_AVL3_INIT_NODE!(stNode4.stNode);
            stNode4.iKey = 7;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode4.stNode, stTreeInfo), true);
            ulVar = 7;
            assert!(VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) != NULL!());
            VOS_AVL3_INIT_NODE!(stNode5.stNode);
            stNode5.iKey = 6;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode5.stNode, stTreeInfo), true);
            ulVar = 6;
            assert!(VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) != NULL!());
            VOS_AVL3_INIT_NODE!(stNode6.stNode);
            stNode6.iKey = 3;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode6.stNode, stTreeInfo), true);
            ulVar = 3;
            assert!(VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) != NULL!());
            VOS_AVL3_INIT_NODE!(stNode7.stNode);
            stNode7.iKey = 20;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode7.stNode, stTreeInfo), true);
            ulVar = 20;
            assert!(VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) != NULL!());
            VOS_AVL3_INIT_NODE!(stNode8.stNode);
            stNode8.iKey = 12;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode8.stNode, stTreeInfo), true);
            ulVar = 12;
            assert!(VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) != NULL!());
            test_no_memory_leak!();
        }
    }

    #[test]
    fn ITest_AVL3_INSERT_007() {
        unsafe {
            let mut stTree: AVL3_TREE = Default::default();
            let mut stNode1: UserStruct = Default::default();
            let mut stNode2: UserStruct = Default::default();
            let mut stNode3: UserStruct = Default::default();
            let mut stNode4: UserStruct = Default::default();
            let mut stNode5: UserStruct = Default::default();
            let mut stNode6: UserStruct = Default::default();
            let mut stNode7: UserStruct = Default::default();
            let mut stNode8: UserStruct = Default::default();
            let mut stTreeInfo: AVL3_TREE_INFO = Default::default();
            let mut ulVar: VOS_UINT32;
            g_ulKeyoffset = (c_ref!(stNode1.iKey).cast::<Ptr<VOS_CHAR>>()
                - c_ref!(stNode1).cast::<Ptr<VOS_CHAR>>()) as u16;
            g_ulNodeoffset = (c_ref!(stNode1.stNode).cast::<Ptr<VOS_CHAR>>()
                - c_ref!(stNode1).cast::<Ptr<VOS_CHAR>>()) as u16;
            stTreeInfo.usKeyOffset = g_ulKeyoffset;
            stTreeInfo.pfCompare = func!(AVL3_CompareFunc);
            stTreeInfo.usNodeOffset = g_ulNodeoffset;
            VOS_AVL3_INIT_TREE!(stTree, stTreeInfo);
            VOS_AVL3_INIT_NODE!(stNode1.stNode);
            stNode1.iKey = 9;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode1.stNode, stTreeInfo), true);
            ulVar = 9;
            assert!(VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) != NULL!());
            VOS_AVL3_INIT_NODE!(stNode2.stNode);
            stNode2.iKey = 8;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode2.stNode, stTreeInfo), true);
            ulVar = 8;
            assert!(VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) != NULL!());
            VOS_AVL3_INIT_NODE!(stNode3.stNode);
            stNode3.iKey = 11;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode3.stNode, stTreeInfo), true);
            ulVar = 11;
            assert!(VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) != NULL!());
            VOS_AVL3_INIT_NODE!(stNode4.stNode);
            stNode4.iKey = 7;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode4.stNode, stTreeInfo), true);
            ulVar = 7;
            assert!(VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) != NULL!());
            VOS_AVL3_INIT_NODE!(stNode5.stNode);
            stNode5.iKey = 6;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode5.stNode, stTreeInfo), true);
            ulVar = 6;
            assert!(VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) != NULL!());
            VOS_AVL3_INIT_NODE!(stNode6.stNode);
            stNode6.iKey = 3;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode6.stNode, stTreeInfo), true);
            ulVar = 3;
            assert!(VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) != NULL!());
            VOS_AVL3_INIT_NODE!(stNode7.stNode);
            stNode7.iKey = 20;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode7.stNode, stTreeInfo), true);
            ulVar = 20;
            assert!(VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) != NULL!());
            VOS_AVL3_INIT_NODE!(stNode8.stNode);
            stNode8.iKey = 12;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode8.stNode, stTreeInfo), true);
            ulVar = 12;
            assert!(VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) != NULL!());
            VOS_AVL3_DELETE!(stTree, stNode8.stNode);
            ulVar = 12;
            if VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) != NULL!() {
                assert!(false);
            }
            VOS_AVL3_DELETE!(stTree, stNode7.stNode);
            ulVar = 20;
            if VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) != NULL!() {
                assert!(false);
            }
            VOS_AVL3_INIT_NODE!(stNode7.stNode);
            stNode7.iKey = 20;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode7.stNode, stTreeInfo), true);
            ulVar = 20;
            assert!(VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) != NULL!());
            VOS_AVL3_DELETE!(stTree, stNode1.stNode);
            test_no_memory_leak!();
        }
    }

    #[test]
    fn ITest_AVL3_INSERT_008() {
        unsafe {
            let mut stTree: AVL3_TREE = Default::default();
            let mut stNode1: UserStruct = Default::default();
            let mut stNode2: UserStruct = Default::default();
            let mut stNode3: UserStruct = Default::default();
            let mut stNode4: UserStruct = Default::default();
            let mut stNode5: UserStruct = Default::default();
            let mut stNode6: UserStruct = Default::default();
            let mut stNode7: UserStruct = Default::default();
            let mut stNode8: UserStruct = Default::default();
            let mut stTreeInfo: AVL3_TREE_INFO = Default::default();
            let mut ulVar: VOS_UINT32;
            g_ulKeyoffset = (c_ref!(stNode1.iKey).cast::<Ptr<VOS_CHAR>>()
                - c_ref!(stNode1).cast::<Ptr<VOS_CHAR>>()) as u16;
            g_ulNodeoffset = (c_ref!(stNode1.stNode).cast::<Ptr<VOS_CHAR>>()
                - c_ref!(stNode1).cast::<Ptr<VOS_CHAR>>()) as u16;
            stTreeInfo.usKeyOffset = g_ulKeyoffset;
            stTreeInfo.pfCompare = func!(AVL3_CompareFunc);
            stTreeInfo.usNodeOffset = g_ulNodeoffset;
            VOS_AVL3_INIT_TREE!(stTree, stTreeInfo);
            VOS_AVL3_INIT_NODE!(stNode1.stNode);
            stNode1.iKey = 9;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode1.stNode, stTreeInfo), true);
            ulVar = 9;
            assert!(VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) != NULL!());
            VOS_AVL3_INIT_NODE!(stNode2.stNode);
            stNode2.iKey = 8;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode2.stNode, stTreeInfo), true);
            ulVar = 8;
            assert!(VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) != NULL!());
            VOS_AVL3_INIT_NODE!(stNode3.stNode);
            stNode3.iKey = 11;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode3.stNode, stTreeInfo), true);
            ulVar = 11;
            assert!(VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) != NULL!());
            VOS_AVL3_INIT_NODE!(stNode4.stNode);
            stNode4.iKey = 7;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode4.stNode, stTreeInfo), true);
            ulVar = 7;
            assert!(VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) != NULL!());
            VOS_AVL3_INIT_NODE!(stNode5.stNode);
            stNode5.iKey = 6;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode5.stNode, stTreeInfo), true);
            ulVar = 6;
            assert!(VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) != NULL!());
            VOS_AVL3_INIT_NODE!(stNode6.stNode);
            stNode6.iKey = 3;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode6.stNode, stTreeInfo), true);
            ulVar = 3;
            assert!(VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) != NULL!());
            VOS_AVL3_INIT_NODE!(stNode7.stNode);
            stNode7.iKey = 20;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode7.stNode, stTreeInfo), true);
            ulVar = 20;
            assert!(VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) != NULL!());
            VOS_AVL3_INIT_NODE!(stNode8.stNode);
            stNode8.iKey = 12;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode8.stNode, stTreeInfo), true);
            ulVar = 12;
            assert!(VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) != NULL!());
            VOS_AVL3_DELETE!(stTree, stNode8.stNode);
            ulVar = 12;
            if VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) != NULL!() {
                assert!(false);
            }
            VOS_AVL3_DELETE!(stTree, stNode7.stNode);
            ulVar = 20;
            if VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) != NULL!() {
                assert!(false);
            }
            VOS_AVL3_INIT_NODE!(stNode7.stNode);
            stNode7.iKey = 20;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode7.stNode, stTreeInfo), true);
            ulVar = 9;
            assert!(VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) != NULL!());
            VOS_AVL3_DELETE!(stTree, stNode1.stNode);
            ulVar = 9;
            if VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) != NULL!() {
                assert!(false);
            }
            VOS_AVL3_INIT_NODE!(stNode1.stNode);
            stNode1.iKey = 9;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode1.stNode, stTreeInfo), true);
            ulVar = 20;
            assert!(VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) != NULL!());
            test_no_memory_leak!();
        }
    }

    #[test]
    fn ITest_AVL3_INSERT_009() {
        unsafe {
            let mut stTree: AVL3_TREE = Default::default();
            let mut stNode1: UserStruct = Default::default();
            let mut stNode2: UserStruct = Default::default();
            let mut stNode3: UserStruct = Default::default();
            let mut stNode4: UserStruct = Default::default();
            let mut stNode5: UserStruct = Default::default();
            let mut stNode6: UserStruct = Default::default();
            let mut stNode7: UserStruct = Default::default();
            let mut stNode8: UserStruct = Default::default();
            let mut stTreeInfo: AVL3_TREE_INFO = Default::default();
            let mut ulVar: VOS_UINT32;
            g_ulKeyoffset = (c_ref!(stNode1.iKey).cast::<Ptr<VOS_CHAR>>()
                - c_ref!(stNode1).cast::<Ptr<VOS_CHAR>>()) as u16;
            g_ulNodeoffset = (c_ref!(stNode1.stNode).cast::<Ptr<VOS_CHAR>>()
                - c_ref!(stNode1).cast::<Ptr<VOS_CHAR>>()) as u16;
            stTreeInfo.usKeyOffset = g_ulKeyoffset;
            stTreeInfo.pfCompare = func!(AVL3_CompareFunc);
            stTreeInfo.usNodeOffset = g_ulNodeoffset;
            VOS_AVL3_INIT_TREE!(stTree, stTreeInfo);
            VOS_AVL3_INIT_NODE!(stNode1.stNode);
            stNode1.iKey = 9;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode1.stNode, stTreeInfo), true);
            ulVar = 9;
            assert!(VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) != NULL!());
            VOS_AVL3_INIT_NODE!(stNode2.stNode);
            stNode2.iKey = 8;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode2.stNode, stTreeInfo), true);
            ulVar = 8;
            assert!(VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) != NULL!());
            VOS_AVL3_INIT_NODE!(stNode3.stNode);
            stNode3.iKey = 11;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode3.stNode, stTreeInfo), true);
            ulVar = 11;
            assert!(VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) != NULL!());
            VOS_AVL3_INIT_NODE!(stNode4.stNode);
            stNode4.iKey = 7;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode4.stNode, stTreeInfo), true);
            ulVar = 7;
            assert!(VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) != NULL!());
            VOS_AVL3_INIT_NODE!(stNode5.stNode);
            stNode5.iKey = 91;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode5.stNode, stTreeInfo), true);
            ulVar = 91;
            assert!(VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) != NULL!());
            VOS_AVL3_INIT_NODE!(stNode6.stNode);
            stNode6.iKey = 3;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode6.stNode, stTreeInfo), true);
            ulVar = 3;
            assert!(VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) != NULL!());
            VOS_AVL3_INIT_NODE!(stNode7.stNode);
            stNode7.iKey = 20;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode7.stNode, stTreeInfo), true);
            ulVar = 20;
            assert!(VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) != NULL!());
            VOS_AVL3_INIT_NODE!(stNode8.stNode);
            stNode8.iKey = 12;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode8.stNode, stTreeInfo), true);
            ulVar = 12;
            assert!(VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) != NULL!());
            VOS_AVL3_DELETE!(stTree, stNode8.stNode);
            ulVar = 12;
            if VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) != NULL!() {
                assert!(false);
            }
            VOS_AVL3_DELETE!(stTree, stNode4.stNode);
            ulVar = 7;
            if VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) != NULL!() {
                assert!(false);
            }
            VOS_AVL3_INIT_NODE!(stNode4.stNode);
            stNode4.iKey = 7;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode4.stNode, stTreeInfo), true);
            ulVar = 7;
            assert!(VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) != NULL!());
            VOS_AVL3_DELETE!(stTree, stNode1.stNode);
            ulVar = 9;
            if VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) != NULL!() {
                assert!(false);
            }
            VOS_AVL3_INIT_NODE!(stNode1.stNode);
            stNode1.iKey = 9;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode1.stNode, stTreeInfo), true);
            ulVar = 9;
            assert!(VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) != NULL!());
            VOS_AVL3_DELETE!(stTree, stNode1.stNode);
            ulVar = 9;
            if VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) != NULL!() {
                assert!(false);
            }
            VOS_AVL3_DELETE!(stTree, stNode2.stNode);
            ulVar = 8;
            if VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) != NULL!() {
                assert!(false);
            }
            VOS_AVL3_DELETE!(stTree, stNode3.stNode);
            ulVar = 11;
            if VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) != NULL!() {
                assert!(false);
            }
            VOS_AVL3_DELETE!(stTree, stNode4.stNode);
            ulVar = 7;
            if VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) != NULL!() {
                assert!(false);
            }
            VOS_AVL3_DELETE!(stTree, stNode5.stNode);
            ulVar = 91;
            if VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) != NULL!() {
                assert!(false);
            }
            VOS_AVL3_DELETE!(stTree, stNode6.stNode);
            ulVar = 3;
            if VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) != NULL!() {
                assert!(false);
            }
            VOS_AVL3_DELETE!(stTree, stNode7.stNode);
            ulVar = 20;
            if VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) != NULL!() {
                assert!(false);
            }
            test_no_memory_leak!();
        }
    }

    #[test]
    fn ITest_AVL3_INSERT_010() {
        unsafe {
            let mut stTree: AVL3_TREE = Default::default();
            let mut stNode1: UserStruct = Default::default();
            let mut stNode2: UserStruct = Default::default();
            let mut stNode3: UserStruct = Default::default();
            let mut stNode4: UserStruct = Default::default();
            let mut stNode5: UserStruct = Default::default();
            let mut stNode6: UserStruct = Default::default();
            let mut stNode7: UserStruct = Default::default();
            let mut stNode8: UserStruct = Default::default();
            let mut stTreeInfo: AVL3_TREE_INFO = Default::default();
            g_ulKeyoffset = (c_ref!(stNode1.iKey).cast::<Ptr<VOS_CHAR>>()
                - c_ref!(stNode1).cast::<Ptr<VOS_CHAR>>()) as u16;
            g_ulNodeoffset = (c_ref!(stNode1.stNode).cast::<Ptr<VOS_CHAR>>()
                - c_ref!(stNode1).cast::<Ptr<VOS_CHAR>>()) as u16;
            stTreeInfo.usKeyOffset = g_ulKeyoffset;
            stTreeInfo.pfCompare = func!(AVL3_CompareFunc);
            stTreeInfo.usNodeOffset = g_ulNodeoffset;
            VOS_AVL3_INIT_TREE!(stTree, stTreeInfo);
            VOS_AVL3_INIT_NODE!(stNode1.stNode);
            stNode1.iKey = 9;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode1.stNode, stTreeInfo), true);
            VOS_AVL3_INIT_NODE!(stNode2.stNode);
            stNode2.iKey = 8;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode2.stNode, stTreeInfo), true);
            VOS_AVL3_INIT_NODE!(stNode3.stNode);
            stNode3.iKey = 11;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode3.stNode, stTreeInfo), true);
            VOS_AVL3_INIT_NODE!(stNode4.stNode);
            stNode4.iKey = 7;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode4.stNode, stTreeInfo), true);
            VOS_AVL3_INIT_NODE!(stNode5.stNode);
            stNode5.iKey = 92;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode5.stNode, stTreeInfo), true);
            VOS_AVL3_INIT_NODE!(stNode6.stNode);
            stNode6.iKey = 3;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode6.stNode, stTreeInfo), true);
            VOS_AVL3_INIT_NODE!(stNode7.stNode);
            stNode7.iKey = 20;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode7.stNode, stTreeInfo), true);
            VOS_AVL3_INIT_NODE!(stNode8.stNode);
            stNode8.iKey = 12;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode8.stNode, stTreeInfo), true);
            VOS_AVL3_DELETE!(stTree, stNode8.stNode);
            VOS_AVL3_DELETE!(stTree, stNode4.stNode);
            VOS_AVL3_INIT_NODE!(stNode4.stNode);
            stNode4.iKey = 7;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode4.stNode, stTreeInfo), true);
            VOS_AVL3_DELETE!(stTree, stNode1.stNode);
            VOS_AVL3_INIT_NODE!(stNode1.stNode);
            stNode1.iKey = 9;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode1.stNode, stTreeInfo), true);
            VOS_AVL3_DELETE!(stTree, stNode5.stNode);
            VOS_AVL3_DELETE!(stTree, stNode7.stNode);
            test_no_memory_leak!();
        }
    }

    #[test]
    fn ITest_AVL3_INSERT_011() {
        unsafe {
            let mut stTree: AVL3_TREE = Default::default();
            let mut stNode1: UserStruct = Default::default();
            let mut stNode2: UserStruct = Default::default();
            let mut stNode3: UserStruct = Default::default();
            let mut stNode4: UserStruct = Default::default();
            let mut stNode5: UserStruct = Default::default();
            let mut stNode6: UserStruct = Default::default();
            let mut stNode7: UserStruct = Default::default();
            let mut stNode8: UserStruct = Default::default();
            let mut stTreeInfo: AVL3_TREE_INFO = Default::default();
            g_ulKeyoffset = (c_ref!(stNode1.iKey).cast::<Ptr<VOS_CHAR>>()
                - c_ref!(stNode1).cast::<Ptr<VOS_CHAR>>()) as u16;
            g_ulNodeoffset = (c_ref!(stNode1.stNode).cast::<Ptr<VOS_CHAR>>()
                - c_ref!(stNode1).cast::<Ptr<VOS_CHAR>>()) as u16;
            stTreeInfo.usKeyOffset = g_ulKeyoffset;
            stTreeInfo.pfCompare = func!(AVL3_CompareFunc);
            stTreeInfo.usNodeOffset = g_ulNodeoffset;
            VOS_AVL3_INIT_TREE!(stTree, stTreeInfo);
            VOS_AVL3_INIT_NODE!(stNode1.stNode);
            stNode1.iKey = 9;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode1.stNode, stTreeInfo), true);
            VOS_AVL3_INIT_NODE!(stNode2.stNode);
            stNode2.iKey = 8;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode2.stNode, stTreeInfo), true);
            VOS_AVL3_INIT_NODE!(stNode3.stNode);
            stNode3.iKey = 11;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode3.stNode, stTreeInfo), true);
            VOS_AVL3_INIT_NODE!(stNode4.stNode);
            stNode4.iKey = 7;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode4.stNode, stTreeInfo), true);
            VOS_AVL3_INIT_NODE!(stNode5.stNode);
            stNode5.iKey = 6;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode5.stNode, stTreeInfo), true);
            VOS_AVL3_INIT_NODE!(stNode6.stNode);
            stNode6.iKey = 3;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode6.stNode, stTreeInfo), true);
            VOS_AVL3_INIT_NODE!(stNode7.stNode);
            stNode7.iKey = 20;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode7.stNode, stTreeInfo), true);
            VOS_AVL3_INIT_NODE!(stNode8.stNode);
            stNode8.iKey = 12;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode8.stNode, stTreeInfo), true);
            VOS_AVL3_DELETE!(stTree, stNode8.stNode);
            VOS_AVL3_DELETE!(stTree, stNode4.stNode);
            VOS_AVL3_INIT_NODE!(stNode4.stNode);
            stNode4.iKey = 7;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode4.stNode, stTreeInfo), true);
            VOS_AVL3_DELETE!(stTree, stNode1.stNode);
            VOS_AVL3_INIT_NODE!(stNode1.stNode);
            stNode1.iKey = 9;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode1.stNode, stTreeInfo), true);
            VOS_AVL3_DELETE!(stTree, stNode7.stNode);
            test_no_memory_leak!();
        }
    }

    #[test]
    fn ITest_AVL3_DELETE_001() {
        unsafe {
            let mut ulVar: VOS_UINT32;
            let mut stTree: AVL3_TREE = Default::default();
            let mut stNode1: UserStruct = Default::default();
            let mut stNode2: UserStruct = Default::default();
            let mut stNode3: UserStruct = Default::default();
            let mut stNode4: UserStruct = Default::default();
            let mut stNode5: UserStruct = Default::default();
            let mut stNode6: UserStruct = Default::default();
            let mut stNode7: UserStruct = Default::default();
            g_ulKeyoffset = (c_ref!(stNode1.iKey).cast::<Ptr<VOS_CHAR>>()
                - c_ref!(stNode1).cast::<Ptr<VOS_CHAR>>()) as u16;
            g_ulNodeoffset = (c_ref!(stNode1.stNode).cast::<Ptr<VOS_CHAR>>()
                - c_ref!(stNode1).cast::<Ptr<VOS_CHAR>>()) as u16;
            let mut stTreeInfo: AVL3_TREE_INFO = Default::default();
            stTreeInfo.usKeyOffset = g_ulKeyoffset;
            stTreeInfo.pfCompare = func!(AVL3_CompareFunc);
            stTreeInfo.usNodeOffset = g_ulNodeoffset;
            VOS_AVL3_INIT_TREE!(stTree, stTreeInfo);
            VOS_AVL3_INIT_NODE!(stNode1.stNode);
            stNode1.iKey = 9;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode1.stNode, stTreeInfo), true);
            ulVar = 9;
            assert!(VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) != NULL!());
            VOS_AVL3_INIT_NODE!(stNode2.stNode);
            stNode2.iKey = 8;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode2.stNode, stTreeInfo), true);
            ulVar = 8;
            assert!(VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) != NULL!());
            VOS_AVL3_INIT_NODE!(stNode3.stNode);
            stNode3.iKey = 11;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode3.stNode, stTreeInfo), true);
            ulVar = 11;
            assert!(VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) != NULL!());
            VOS_AVL3_INIT_NODE!(stNode4.stNode);
            stNode4.iKey = 7;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode4.stNode, stTreeInfo), true);
            ulVar = 7;
            assert!(VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) != NULL!());
            VOS_AVL3_INIT_NODE!(stNode5.stNode);
            stNode5.iKey = 6;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode5.stNode, stTreeInfo), true);
            ulVar = 6;
            assert!(VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) != NULL!());
            VOS_AVL3_INIT_NODE!(stNode6.stNode);
            stNode6.iKey = 3;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode6.stNode, stTreeInfo), true);
            ulVar = 3;
            assert!(VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) != NULL!());
            VOS_AVL3_INIT_NODE!(stNode7.stNode);
            stNode7.iKey = 20;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode7.stNode, stTreeInfo), true);
            ulVar = 20;
            assert!(VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) != NULL!());
            VOS_AVL3_DELETE!(stTree, stNode7.stNode);
            if VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) != NULL!() {
                assert!(false);
            }
            test_no_memory_leak!();
        }
    }

    #[test]
    fn ITest_AVL3_DELETE_002() {
        unsafe {
            let mut ulVar: VOS_UINT32;
            let mut stTree: AVL3_TREE = Default::default();
            let mut stNode1: UserStruct = Default::default();
            let mut stNode2: UserStruct = Default::default();
            let mut stNode3: UserStruct = Default::default();
            let mut stNode4: UserStruct = Default::default();
            let mut stNode5: UserStruct = Default::default();
            let mut stNode6: UserStruct = Default::default();
            let mut stNode7: UserStruct = Default::default();
            g_ulKeyoffset = (c_ref!(stNode1.iKey).cast::<Ptr<VOS_CHAR>>()
                - c_ref!(stNode1).cast::<Ptr<VOS_CHAR>>()) as u16;
            g_ulNodeoffset = (c_ref!(stNode1.stNode).cast::<Ptr<VOS_CHAR>>()
                - c_ref!(stNode1).cast::<Ptr<VOS_CHAR>>()) as u16;
            let mut stTreeInfo: AVL3_TREE_INFO = Default::default();
            stTreeInfo.usKeyOffset = g_ulKeyoffset;
            stTreeInfo.pfCompare = func!(AVL3_CompareFunc);
            stTreeInfo.usNodeOffset = g_ulNodeoffset;
            VOS_AVL3_INIT_TREE!(stTree, stTreeInfo);
            VOS_AVL3_INIT_NODE!(stNode1.stNode);
            stNode1.iKey = 9;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode1.stNode, stTreeInfo), true);
            ulVar = 9;
            assert!(VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) != NULL!());
            VOS_AVL3_INIT_NODE!(stNode2.stNode);
            stNode2.iKey = 8;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode2.stNode, stTreeInfo), true);
            ulVar = 8;
            assert!(VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) != NULL!());
            VOS_AVL3_INIT_NODE!(stNode3.stNode);
            stNode3.iKey = 11;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode3.stNode, stTreeInfo), true);
            ulVar = 11;
            assert!(VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) != NULL!());
            VOS_AVL3_INIT_NODE!(stNode4.stNode);
            stNode4.iKey = 7;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode4.stNode, stTreeInfo), true);
            ulVar = 7;
            assert!(VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) != NULL!());
            VOS_AVL3_INIT_NODE!(stNode5.stNode);
            stNode5.iKey = 6;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode5.stNode, stTreeInfo), true);
            ulVar = 6;
            assert!(VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) != NULL!());
            VOS_AVL3_INIT_NODE!(stNode6.stNode);
            stNode6.iKey = 3;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode6.stNode, stTreeInfo), true);
            ulVar = 3;
            assert!(VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) != NULL!());
            VOS_AVL3_INIT_NODE!(stNode7.stNode);
            stNode7.iKey = 20;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode7.stNode, stTreeInfo), true);
            ulVar = 20;
            assert!(VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) != NULL!());
            VOS_AVL3_DELETE!(stTree, stNode7.stNode);
            ulVar = 20;
            if VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) != NULL!() {
                assert!(false);
            }
            test_no_memory_leak!();
        }
    }

    #[test]
    fn ITest_AVL3_DELETE_003() {
        unsafe {
            let mut ulVar: VOS_UINT32;
            let mut stTree: AVL3_TREE = Default::default();
            let mut stNode1: UserStruct = Default::default();
            let mut stNode2: UserStruct = Default::default();
            let mut stNode3: UserStruct = Default::default();
            let mut stNode4: UserStruct = Default::default();
            let mut stNode5: UserStruct = Default::default();
            let mut stNode6: UserStruct = Default::default();
            let mut stNode7: UserStruct = Default::default();
            g_ulKeyoffset = (c_ref!(stNode1.iKey).cast::<Ptr<VOS_CHAR>>()
                - c_ref!(stNode1).cast::<Ptr<VOS_CHAR>>()) as u16;
            g_ulNodeoffset = (c_ref!(stNode1.stNode).cast::<Ptr<VOS_CHAR>>()
                - c_ref!(stNode1).cast::<Ptr<VOS_CHAR>>()) as u16;
            let mut stTreeInfo: AVL3_TREE_INFO = Default::default();
            stTreeInfo.usKeyOffset = g_ulKeyoffset;
            stTreeInfo.pfCompare = func!(AVL3_CompareFunc);
            stTreeInfo.usNodeOffset = g_ulNodeoffset;
            VOS_AVL3_INIT_TREE!(stTree, stTreeInfo);
            VOS_AVL3_INIT_NODE!(stNode1.stNode);
            stNode1.iKey = 9;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode1.stNode, stTreeInfo), true);
            ulVar = 9;
            assert!(VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) != NULL!());
            VOS_AVL3_INIT_NODE!(stNode2.stNode);
            stNode2.iKey = 8;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode2.stNode, stTreeInfo), true);
            ulVar = 8;
            assert!(VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) != NULL!());
            VOS_AVL3_INIT_NODE!(stNode3.stNode);
            stNode3.iKey = 11;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode3.stNode, stTreeInfo), true);
            ulVar = 11;
            assert!(VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) != NULL!());
            VOS_AVL3_INIT_NODE!(stNode4.stNode);
            stNode4.iKey = 7;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode4.stNode, stTreeInfo), true);
            ulVar = 7;
            assert!(VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) != NULL!());
            VOS_AVL3_INIT_NODE!(stNode5.stNode);
            stNode5.iKey = 6;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode5.stNode, stTreeInfo), true);
            ulVar = 6;
            assert!(VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) != NULL!());
            VOS_AVL3_INIT_NODE!(stNode6.stNode);
            stNode6.iKey = 3;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode6.stNode, stTreeInfo), true);
            ulVar = 3;
            assert!(VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) != NULL!());
            VOS_AVL3_INIT_NODE!(stNode7.stNode);
            stNode7.iKey = 20;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode7.stNode, stTreeInfo), true);
            ulVar = 20;
            assert!(VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) != NULL!());
            test_no_memory_leak!();
        }
    }

    #[test]
    fn ITest_AVL3_DELETE_004() {
        unsafe {
            let mut ulVar: VOS_UINT32;
            let mut stTree: AVL3_TREE = Default::default();
            let mut stNode1: UserStruct = Default::default();
            let mut stNode2: UserStruct = Default::default();
            let mut stNode3: UserStruct = Default::default();
            g_ulKeyoffset = (c_ref!(stNode1.iKey).cast::<Ptr<VOS_CHAR>>()
                - c_ref!(stNode1).cast::<Ptr<VOS_CHAR>>()) as u16;
            g_ulNodeoffset = (c_ref!(stNode1.stNode).cast::<Ptr<VOS_CHAR>>()
                - c_ref!(stNode1).cast::<Ptr<VOS_CHAR>>()) as u16;
            let mut stTreeInfo: AVL3_TREE_INFO = Default::default();
            stTreeInfo.usKeyOffset = g_ulKeyoffset;
            stTreeInfo.pfCompare = func!(AVL3_CompareFunc);
            stTreeInfo.usNodeOffset = g_ulNodeoffset;
            VOS_AVL3_INIT_TREE!(stTree, stTreeInfo);
            VOS_AVL3_INIT_NODE!(stNode1.stNode);
            stNode1.iKey = 9;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode1.stNode, stTreeInfo), true);
            ulVar = 9;
            assert!(VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) != NULL!());
            VOS_AVL3_INIT_NODE!(stNode2.stNode);
            stNode2.iKey = 8;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode2.stNode, stTreeInfo), true);
            ulVar = 8;
            assert!(VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) != NULL!());
            VOS_AVL3_INIT_NODE!(stNode3.stNode);
            stNode3.iKey = 11;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode3.stNode, stTreeInfo), true);
            ulVar = 11;
            assert!(VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) != NULL!());
            VOS_AVL3_DELETE!(stTree, stNode2.stNode);
            ulVar = 8;
            if VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) != NULL!() {
                assert!(false);
            }
            test_no_memory_leak!();
        }
    }

    #[test]
    fn ITest_AVL3_DELETE_006() {
        unsafe {
            let mut ulVar: VOS_UINT32;
            let mut stTree: AVL3_TREE = Default::default();
            let mut stNode1: UserStruct = Default::default();
            let mut stNode2: UserStruct = Default::default();
            let mut stNode3: UserStruct = Default::default();
            let mut stNode4: UserStruct = Default::default();
            let mut stNode5: UserStruct = Default::default();
            let mut stNode6: UserStruct = Default::default();
            let mut stNode7: UserStruct = Default::default();
            let mut stNode8: UserStruct = Default::default();
            g_ulKeyoffset = (c_ref!(stNode1.iKey).cast::<Ptr<VOS_CHAR>>()
                - c_ref!(stNode1).cast::<Ptr<VOS_CHAR>>()) as u16;
            g_ulNodeoffset = (c_ref!(stNode1.stNode).cast::<Ptr<VOS_CHAR>>()
                - c_ref!(stNode1).cast::<Ptr<VOS_CHAR>>()) as u16;
            let mut stTreeInfo: AVL3_TREE_INFO = Default::default();
            stTreeInfo.usKeyOffset = g_ulKeyoffset;
            stTreeInfo.pfCompare = func!(AVL3_CompareFunc);
            stTreeInfo.usNodeOffset = g_ulNodeoffset;
            VOS_AVL3_INIT_TREE!(stTree, stTreeInfo);
            VOS_AVL3_INIT_NODE!(stNode1.stNode);
            stNode1.iKey = 9;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode1.stNode, stTreeInfo), true);
            ulVar = 9;
            assert!(VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) != NULL!());
            VOS_AVL3_INIT_NODE!(stNode2.stNode);
            stNode2.iKey = 8;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode2.stNode, stTreeInfo), true);
            ulVar = 8;
            assert!(VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) != NULL!());
            VOS_AVL3_INIT_NODE!(stNode3.stNode);
            stNode3.iKey = 11;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode3.stNode, stTreeInfo), true);
            ulVar = 11;
            assert!(VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) != NULL!());
            VOS_AVL3_INIT_NODE!(stNode4.stNode);
            stNode4.iKey = 7;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode4.stNode, stTreeInfo), true);
            ulVar = 7;
            assert!(VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) != NULL!());
            VOS_AVL3_INIT_NODE!(stNode5.stNode);
            stNode5.iKey = 92;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode5.stNode, stTreeInfo), true);
            ulVar = 92;
            assert!(VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) != NULL!());
            VOS_AVL3_INIT_NODE!(stNode6.stNode);
            stNode6.iKey = 3;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode6.stNode, stTreeInfo), true);
            ulVar = 3;
            assert!(VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) != NULL!());
            VOS_AVL3_INIT_NODE!(stNode7.stNode);
            stNode7.iKey = 20;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode7.stNode, stTreeInfo), true);
            ulVar = 20;
            assert!(VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) != NULL!());
            VOS_AVL3_INIT_NODE!(stNode8.stNode);
            stNode8.iKey = 200;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode8.stNode, stTreeInfo), true);
            ulVar = 200;
            assert!(VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) != NULL!());
            VOS_AVL3_DELETE!(stTree, stNode1.stNode);
            ulVar = 9;
            if VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) != NULL!() {
                assert!(false);
            }
            VOS_AVL3_DELETE!(stTree, stNode2.stNode);
            ulVar = 8;
            if VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) != NULL!() {
                assert!(false);
            }
            VOS_AVL3_DELETE!(stTree, stNode3.stNode);
            ulVar = 11;
            if VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) != NULL!() {
                assert!(false);
            }
            VOS_AVL3_DELETE!(stTree, stNode4.stNode);
            ulVar = 7;
            if VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) != NULL!() {
                assert!(false);
            }
            VOS_AVL3_DELETE!(stTree, stNode5.stNode);
            ulVar = 92;
            if VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) != NULL!() {
                assert!(false);
            }
            VOS_AVL3_DELETE!(stTree, stNode6.stNode);
            ulVar = 3;
            if VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) != NULL!() {
                assert!(false);
            }
            VOS_AVL3_DELETE!(stTree, stNode7.stNode);
            ulVar = 20;
            if VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) != NULL!() {
                assert!(false);
            }
            VOS_AVL3_DELETE!(stTree, stNode8.stNode);
            ulVar = 200;
            if VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) != NULL!() {
                assert!(false);
            }
            test_no_memory_leak!();
        }
    }

    #[test]
    fn ITest_AVL3_DELETE_007() {
        unsafe {
            let mut ulVar: VOS_UINT32;
            let mut stTree: AVL3_TREE = Default::default();
            let mut stNode1: UserStruct = Default::default();
            let mut stNode2: UserStruct = Default::default();
            let mut stNode3: UserStruct = Default::default();
            g_ulKeyoffset = (c_ref!(stNode1.iKey).cast::<Ptr<VOS_CHAR>>()
                - c_ref!(stNode1).cast::<Ptr<VOS_CHAR>>()) as u16;
            g_ulNodeoffset = (c_ref!(stNode1.stNode).cast::<Ptr<VOS_CHAR>>()
                - c_ref!(stNode1).cast::<Ptr<VOS_CHAR>>()) as u16;
            let mut stTreeInfo: AVL3_TREE_INFO = Default::default();
            stTreeInfo.usKeyOffset = g_ulKeyoffset;
            stTreeInfo.pfCompare = func!(AVL3_CompareFunc);
            stTreeInfo.usNodeOffset = g_ulNodeoffset;
            VOS_AVL3_INIT_TREE!(stTree, stTreeInfo);
            VOS_AVL3_INIT_NODE!(stNode1.stNode);
            stNode1.iKey = 9;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode1.stNode, stTreeInfo), true);
            ulVar = 9;
            assert!(VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) != NULL!());
            VOS_AVL3_INIT_NODE!(stNode2.stNode);
            stNode2.iKey = 8;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode2.stNode, stTreeInfo), true);
            ulVar = 8;
            assert!(VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) != NULL!());
            VOS_AVL3_INIT_NODE!(stNode3.stNode);
            stNode3.iKey = 11;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode3.stNode, stTreeInfo), true);
            ulVar = 11;
            assert!(VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) != NULL!());
            VOS_AVL3_DELETE!(stTree, stNode2.stNode);
            ulVar = 8;
            if VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) != NULL!() {
                assert!(false);
            }
            test_no_memory_leak!();
        }
    }

    #[test]
    fn ITest_AVL3_DELETE_008() {
        unsafe {
            let mut ulVar: VOS_UINT32;
            let mut stTree: AVL3_TREE = Default::default();
            let mut stNode1: UserStruct = Default::default();
            let mut stNode2: UserStruct = Default::default();
            let mut stNode3: UserStruct = Default::default();
            g_ulKeyoffset = (c_ref!(stNode1.iKey).cast::<Ptr<VOS_CHAR>>()
                - c_ref!(stNode1).cast::<Ptr<VOS_CHAR>>()) as u16;
            g_ulNodeoffset = (c_ref!(stNode1.stNode).cast::<Ptr<VOS_CHAR>>()
                - c_ref!(stNode1).cast::<Ptr<VOS_CHAR>>()) as u16;
            let mut stTreeInfo: AVL3_TREE_INFO = Default::default();
            stTreeInfo.usKeyOffset = g_ulKeyoffset;
            stTreeInfo.pfCompare = func!(AVL3_CompareFunc);
            stTreeInfo.usNodeOffset = g_ulNodeoffset;
            VOS_AVL3_INIT_TREE!(stTree, stTreeInfo);
            VOS_AVL3_INIT_NODE!(stNode1.stNode);
            stNode1.iKey = 9;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode1.stNode, stTreeInfo), true);
            ulVar = 9;
            assert!(VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) != NULL!());
            VOS_AVL3_INIT_NODE!(stNode2.stNode);
            stNode2.iKey = 8;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode2.stNode, stTreeInfo), true);
            ulVar = 8;
            assert!(VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) != NULL!());
            VOS_AVL3_INIT_NODE!(stNode3.stNode);
            stNode3.iKey = 11;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode3.stNode, stTreeInfo), true);
            ulVar = 11;
            assert!(VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) != NULL!());
            VOS_AVL3_DELETE!(stTree, stNode1.stNode);
            ulVar = 9;
            if VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) != NULL!() {
                assert!(false);
            }
            test_no_memory_leak!();
        }
    }

    #[test]
    fn ITest_AVL3_DELETE_009() {
        unsafe {
            let mut ulVar: VOS_UINT32;
            let mut stTree: AVL3_TREE = Default::default();
            let mut stNode1: UserStruct = Default::default();
            let mut stNode2: UserStruct = Default::default();
            let mut stNode3: UserStruct = Default::default();
            g_ulKeyoffset = (c_ref!(stNode1.iKey).cast::<Ptr<VOS_CHAR>>()
                - c_ref!(stNode1).cast::<Ptr<VOS_CHAR>>()) as u16;
            g_ulNodeoffset = (c_ref!(stNode1.stNode).cast::<Ptr<VOS_CHAR>>()
                - c_ref!(stNode1).cast::<Ptr<VOS_CHAR>>()) as u16;
            let mut stTreeInfo: AVL3_TREE_INFO = Default::default();
            stTreeInfo.usKeyOffset = g_ulKeyoffset;
            stTreeInfo.pfCompare = func!(AVL3_CompareFunc);
            stTreeInfo.usNodeOffset = g_ulNodeoffset;
            VOS_AVL3_INIT_TREE!(stTree, stTreeInfo);
            VOS_AVL3_INIT_NODE!(stNode1.stNode);
            stNode1.iKey = 9;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode1.stNode, stTreeInfo), true);
            ulVar = 9;
            assert!(VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) != NULL!());
            VOS_AVL3_INIT_NODE!(stNode2.stNode);
            stNode2.iKey = 8;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode2.stNode, stTreeInfo), true);
            ulVar = 8;
            assert!(VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) != NULL!());
            VOS_AVL3_INIT_NODE!(stNode3.stNode);
            stNode3.iKey = 11;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode3.stNode, stTreeInfo), true);
            ulVar = 11;
            assert!(VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) != NULL!());
            VOS_AVL3_DELETE!(stTree, stNode1.stNode);
            VOS_AVL3_INIT_NODE!(stNode1.stNode);
            stNode1.iKey = 9;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode1.stNode, stTreeInfo), true);
            VOS_AVL3_DELETE!(stTree, stNode1.stNode);
            ulVar = 9;
            if VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) != NULL!() {
                assert!(false);
            }
            test_no_memory_leak!();
        }
    }

    #[test]
    fn ITest_AVL3_FIND_001() {
        unsafe {
            let mut ulVar: VOS_UINT32 = 0;
            let mut ulVar2: VOS_UINT32 = 0;
            let mut stTree: AVL3_TREE = Default::default();
            let mut stNode1: UserStruct = Default::default();
            let mut stNode2: UserStruct = Default::default();
            let mut stNode3: UserStruct = Default::default();
            let mut stNode4: UserStruct = Default::default();
            g_ulKeyoffset = (c_ref!(stNode1.iKey).cast::<Ptr<VOS_CHAR>>()
                - c_ref!(stNode1).cast::<Ptr<VOS_CHAR>>()) as u16;
            g_ulNodeoffset = (c_ref!(stNode1.stNode).cast::<Ptr<VOS_CHAR>>()
                - c_ref!(stNode1).cast::<Ptr<VOS_CHAR>>()) as u16;
            let mut stTreeInfo: AVL3_TREE_INFO = Default::default();
            stTreeInfo.usKeyOffset = g_ulKeyoffset;
            stTreeInfo.pfCompare = func!(AVL3_CompareFunc);
            stTreeInfo.usNodeOffset = g_ulNodeoffset;
            VOS_AVL3_INIT_TREE!(stTree, stTreeInfo);
            VOS_AVL3_INIT_NODE!(stNode1.stNode);
            stNode1.iKey = 9;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode1.stNode, stTreeInfo), true);
            ulVar = 9;
            assert!(VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) != NULL!());
            VOS_AVL3_INIT_NODE!(stNode2.stNode);
            stNode2.iKey = 8;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode2.stNode, stTreeInfo), true);
            ulVar = 8;
            assert!(VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) != NULL!());
            VOS_AVL3_INIT_NODE!(stNode3.stNode);
            stNode3.iKey = 11;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode3.stNode, stTreeInfo), true);
            ulVar = 11;
            assert!(VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) != NULL!());
            ulVar = 8;
            stNode4 = *(VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo)
                .cast::<Ptr<UserStruct>>());
            VOS_GetAVL3Node(c_ref!(stNode4.stNode), c_ref!(ulVar2));
            assert_eq!(8, ulVar2);
            test_no_memory_leak!();
        }
    }

    #[test]
    fn ITest_AVL3_FIND_002() {
        unsafe {
            let mut ulVar: VOS_UINT32 = 0;
            let mut ulVar2: VOS_UINT32 = 0;
            let mut stTree: AVL3_TREE = Default::default();
            let mut stNode1: UserStruct = Default::default();
            let mut stNode2: UserStruct = Default::default();
            let mut stNode3: UserStruct = Default::default();
            let mut stNode4: UserStruct = Default::default();
            g_ulKeyoffset = (c_ref!(stNode1.iKey).cast::<Ptr<VOS_CHAR>>()
                - c_ref!(stNode1).cast::<Ptr<VOS_CHAR>>()) as u16;
            g_ulNodeoffset = (c_ref!(stNode1.stNode).cast::<Ptr<VOS_CHAR>>()
                - c_ref!(stNode1).cast::<Ptr<VOS_CHAR>>()) as u16;
            let mut stTreeInfo: AVL3_TREE_INFO = Default::default();
            stTreeInfo.usKeyOffset = g_ulKeyoffset;
            stTreeInfo.pfCompare = func!(AVL3_CompareFunc);
            stTreeInfo.usNodeOffset = g_ulNodeoffset;
            VOS_AVL3_INIT_TREE!(stTree, stTreeInfo);
            VOS_AVL3_INIT_NODE!(stNode1.stNode);
            stNode1.iKey = 9;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode1.stNode, stTreeInfo), true);
            ulVar = 9;
            if VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) == NULL!() {
                assert!(false);
            }
            VOS_AVL3_INIT_NODE!(stNode2.stNode);
            stNode2.iKey = 8;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode2.stNode, stTreeInfo), true);
            ulVar = 8;
            if VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) == NULL!() {
                assert!(false);
            }
            VOS_AVL3_INIT_NODE!(stNode3.stNode);
            stNode3.iKey = 11;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode3.stNode, stTreeInfo), true);
            ulVar = 11;
            if VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) == NULL!() {
                assert!(false);
            }
            ulVar = 9;
            stNode4 = *(VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo)
                .cast::<Ptr<UserStruct>>());
            VOS_GetAVL3Node(c_ref!(stNode4.stNode), c_ref!(ulVar2));
            assert_eq!(9, ulVar2);
            test_no_memory_leak!();
        }
    }

    #[test]
    fn ITest_AVL3_FIND_003() {
        unsafe {
            let mut ulVar: VOS_UINT32 = 0;
            let mut stTree: AVL3_TREE = Default::default();
            let mut stNode1: UserStruct = Default::default();
            let mut stNode2: UserStruct = Default::default();
            let mut stNode3: UserStruct = Default::default();
            g_ulKeyoffset = (c_ref!(stNode1.iKey).cast::<Ptr<VOS_CHAR>>()
                - c_ref!(stNode1).cast::<Ptr<VOS_CHAR>>()) as u16;
            g_ulNodeoffset = (c_ref!(stNode1.stNode).cast::<Ptr<VOS_CHAR>>()
                - c_ref!(stNode1).cast::<Ptr<VOS_CHAR>>()) as u16;
            let mut stTreeInfo: AVL3_TREE_INFO = Default::default();
            stTreeInfo.usKeyOffset = g_ulKeyoffset;
            stTreeInfo.pfCompare = func!(AVL3_CompareFunc);
            stTreeInfo.usNodeOffset = g_ulNodeoffset;
            VOS_AVL3_INIT_TREE!(stTree, stTreeInfo);
            VOS_AVL3_INIT_NODE!(stNode1.stNode);
            stNode1.iKey = 9;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode1.stNode, stTreeInfo), true);
            ulVar = 9;
            if VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) == NULL!() {
                assert!(false);
            }
            VOS_AVL3_INIT_NODE!(stNode2.stNode);
            stNode2.iKey = 8;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode2.stNode, stTreeInfo), true);
            ulVar = 8;
            if VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) == NULL!() {
                assert!(false);
            }
            VOS_AVL3_INIT_NODE!(stNode3.stNode);
            stNode3.iKey = 11;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode3.stNode, stTreeInfo), true);
            ulVar = 11;
            if VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) == NULL!() {
                assert!(false);
            }
            ulVar = 12;
            if VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) != NULL!() {
                VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo);
                assert!(false);
            }
            test_no_memory_leak!();
        }
    }

    #[test]
    fn ITest_AVL3_FIND_004() {
        unsafe {
            let mut ulVar: VOS_UINT32 = 0;
            let mut ulVar2: VOS_UINT32 = 0;
            let mut stTree: AVL3_TREE = Default::default();
            let mut stNode1: UserStruct = Default::default();
            let mut stNode2: UserStruct = Default::default();
            let mut stNode3: UserStruct = Default::default();
            let mut stNode4: UserStruct = Default::default();
            g_ulKeyoffset = (c_ref!(stNode1.iKey).cast::<Ptr<VOS_CHAR>>()
                - c_ref!(stNode1).cast::<Ptr<VOS_CHAR>>()) as u16;
            g_ulNodeoffset = (c_ref!(stNode1.stNode).cast::<Ptr<VOS_CHAR>>()
                - c_ref!(stNode1).cast::<Ptr<VOS_CHAR>>()) as u16;
            let mut stTreeInfo: AVL3_TREE_INFO = Default::default();
            stTreeInfo.usKeyOffset = g_ulKeyoffset;
            stTreeInfo.pfCompare = func!(AVL3_CompareFunc);
            stTreeInfo.usNodeOffset = g_ulNodeoffset;
            VOS_AVL3_INIT_TREE!(stTree, stTreeInfo);
            VOS_AVL3_INIT_NODE!(stNode1.stNode);
            stNode1.iKey = 9;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode1.stNode, stTreeInfo), true);
            ulVar = 9;
            if VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) == NULL!() {
                assert!(false);
            }
            VOS_AVL3_INIT_NODE!(stNode2.stNode);
            stNode2.iKey = 8;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode2.stNode, stTreeInfo), true);
            ulVar = 8;
            if VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) == NULL!() {
                assert!(false);
            }
            VOS_AVL3_INIT_NODE!(stNode3.stNode);
            stNode3.iKey = 11;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode3.stNode, stTreeInfo), true);
            ulVar = 11;
            if VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) == NULL!() {
                assert!(false);
            }
            VOS_AVL3_DELETE!(stTree, stNode2.stNode);
            VOS_AVL3_INIT_NODE!(stNode2.stNode);
            stNode2.iKey = 12;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode2.stNode, stTreeInfo), true);
            ulVar = 12;
            if VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) == NULL!() {
                assert!(false);
            }
            ulVar = 12;
            stNode4 = *(VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo)
                .cast::<Ptr<UserStruct>>());
            VOS_GetAVL3Node(c_ref!(stNode4.stNode), c_ref!(ulVar2));
            assert_eq!(12, ulVar2);
            test_no_memory_leak!();
        }
    }

    #[test]
    fn ITest_AVL3_FIND_005() {
        unsafe {
            let mut ulVar: VOS_UINT32 = 0;
            let mut ulVar2: VOS_UINT32 = 0;
            let mut stTree: AVL3_TREE = Default::default();
            let mut stNode1: UserStruct = Default::default();
            let mut stNode2: UserStruct = Default::default();
            let mut stNode3: UserStruct = Default::default();
            let mut stNode4: UserStruct = Default::default();
            g_ulKeyoffset = (c_ref!(stNode1.iKey).cast::<Ptr<VOS_CHAR>>()
                - c_ref!(stNode1).cast::<Ptr<VOS_CHAR>>()) as u16;
            g_ulNodeoffset = (c_ref!(stNode1.stNode).cast::<Ptr<VOS_CHAR>>()
                - c_ref!(stNode1).cast::<Ptr<VOS_CHAR>>()) as u16;
            let mut stTreeInfo: AVL3_TREE_INFO = Default::default();
            stTreeInfo.usKeyOffset = g_ulKeyoffset;
            stTreeInfo.pfCompare = func!(AVL3_CompareFunc);
            stTreeInfo.usNodeOffset = g_ulNodeoffset;
            VOS_AVL3_INIT_TREE!(stTree, stTreeInfo);
            VOS_AVL3_INIT_NODE!(stNode1.stNode);
            stNode1.iKey = 9;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode1.stNode, stTreeInfo), true);
            ulVar = 9;
            if VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) == NULL!() {
                assert!(false);
            }
            VOS_AVL3_INIT_NODE!(stNode2.stNode);
            stNode2.iKey = 8;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode2.stNode, stTreeInfo), true);
            ulVar = 8;
            if VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) == NULL!() {
                assert!(false);
            }
            VOS_AVL3_INIT_NODE!(stNode3.stNode);
            stNode3.iKey = 11;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode3.stNode, stTreeInfo), true);
            ulVar = 11;
            if VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) == NULL!() {
                assert!(false);
            }
            VOS_AVL3_DELETE!(stTree, stNode2.stNode);
            ulVar = 8;
            if VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) != NULL!() {
                stNode4 = *(VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo)
                    .cast::<Ptr<UserStruct>>());
                VOS_GetAVL3Node(c_ref!(stNode4.stNode), c_ref!(ulVar2));
                assert!(false);
            }
            test_no_memory_leak!();
        }
    }

    #[test]
    fn ITest_AVL3_FIND_006() {
        unsafe {
            let mut ulVar: VOS_UINT32 = 0;
            let mut stTree: AVL3_TREE = Default::default();
            let mut stNode1: UserStruct = Default::default();
            g_ulKeyoffset = (c_ref!(stNode1.iKey).cast::<Ptr<VOS_CHAR>>()
                - c_ref!(stNode1).cast::<Ptr<VOS_CHAR>>()) as u16;
            g_ulNodeoffset = (c_ref!(stNode1.stNode).cast::<Ptr<VOS_CHAR>>()
                - c_ref!(stNode1).cast::<Ptr<VOS_CHAR>>()) as u16;
            let mut stTreeInfo: AVL3_TREE_INFO = Default::default();
            stTreeInfo.usKeyOffset = g_ulKeyoffset;
            stTreeInfo.pfCompare = func!(AVL3_CompareFunc);
            stTreeInfo.usNodeOffset = g_ulNodeoffset;
            VOS_AVL3_INIT_TREE!(stTree, stTreeInfo);
            ulVar = 8;
            if VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) != NULL!() {
                VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo);
                assert!(false);
            }
            test_no_memory_leak!();
        }
    }

    #[test]
    fn ITest_AVL3_FIND_NEXT_001() {
        unsafe {
            let mut ulVar: VOS_UINT32 = 0;
            let mut ulVar2: VOS_UINT32 = 0;
            let mut stTree: AVL3_TREE = Default::default();
            let mut stNode1: UserStruct = Default::default();
            let mut stNode2: UserStruct = Default::default();
            let mut stNode3: UserStruct = Default::default();
            let mut stNode4: UserStruct = Default::default();
            g_ulKeyoffset = (c_ref!(stNode1.iKey).cast::<Ptr<VOS_CHAR>>()
                - c_ref!(stNode1).cast::<Ptr<VOS_CHAR>>()) as u16;
            g_ulNodeoffset = (c_ref!(stNode1.stNode).cast::<Ptr<VOS_CHAR>>()
                - c_ref!(stNode1).cast::<Ptr<VOS_CHAR>>()) as u16;
            let mut stTreeInfo: AVL3_TREE_INFO = Default::default();
            stTreeInfo.usKeyOffset = g_ulKeyoffset;
            stTreeInfo.pfCompare = func!(AVL3_CompareFunc);
            stTreeInfo.usNodeOffset = g_ulNodeoffset;
            VOS_AVL3_INIT_TREE!(stTree, stTreeInfo);
            VOS_AVL3_INIT_NODE!(stNode1.stNode);
            stNode1.iKey = 9;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode1.stNode, stTreeInfo), true);
            ulVar = 9;
            if VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) == NULL!() {
                assert!(false);
            }
            VOS_AVL3_INIT_NODE!(stNode2.stNode);
            stNode2.iKey = 8;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode2.stNode, stTreeInfo), true);
            ulVar = 8;
            if VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) == NULL!() {
                assert!(false);
            }
            VOS_AVL3_INIT_NODE!(stNode3.stNode);
            stNode3.iKey = 11;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode3.stNode, stTreeInfo), true);
            ulVar = 11;
            if VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) == NULL!() {
                assert!(false);
            }
            ulVar = 8;
            stNode4 = *(VOS_AVL3_FIND_NEXT!(stTree, c_ref!(ulVar).cast(), stTreeInfo)
                .cast::<Ptr<UserStruct>>());
            VOS_GetAVL3Node(c_ref!(stNode4.stNode), c_ref!(ulVar2));
            assert_eq!(9, ulVar2);
            test_no_memory_leak!();
        }
    }

    #[test]
    fn ITest_AVL3_FIND_NEXT_002() {
        unsafe {
            let mut ulVar: VOS_UINT32 = 0;
            let mut ulVar2: VOS_UINT32 = 0;
            let mut stTree: AVL3_TREE = Default::default();
            let mut stNode1: UserStruct = Default::default();
            let mut stNode2: UserStruct = Default::default();
            let mut stNode3: UserStruct = Default::default();
            let mut stNode4: UserStruct = Default::default();
            g_ulKeyoffset = (c_ref!(stNode1.iKey).cast::<Ptr<VOS_CHAR>>()
                - c_ref!(stNode1).cast::<Ptr<VOS_CHAR>>()) as u16;
            g_ulNodeoffset = (c_ref!(stNode1.stNode).cast::<Ptr<VOS_CHAR>>()
                - c_ref!(stNode1).cast::<Ptr<VOS_CHAR>>()) as u16;
            let mut stTreeInfo: AVL3_TREE_INFO = Default::default();
            stTreeInfo.usKeyOffset = g_ulKeyoffset;
            stTreeInfo.pfCompare = func!(AVL3_CompareFunc);
            stTreeInfo.usNodeOffset = g_ulNodeoffset;
            VOS_AVL3_INIT_TREE!(stTree, stTreeInfo);
            VOS_AVL3_INIT_NODE!(stNode1.stNode);
            stNode1.iKey = 9;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode1.stNode, stTreeInfo), true);
            ulVar = 9;
            if VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) == NULL!() {
                assert!(false);
            }
            VOS_AVL3_INIT_NODE!(stNode2.stNode);
            stNode2.iKey = 8;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode2.stNode, stTreeInfo), true);
            ulVar = 8;
            if VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) == NULL!() {
                assert!(false);
            }
            VOS_AVL3_INIT_NODE!(stNode3.stNode);
            stNode3.iKey = 11;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode3.stNode, stTreeInfo), true);
            ulVar = 11;
            if VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) == NULL!() {
                assert!(false);
            }
            ulVar = 9;
            stNode4 = *(VOS_AVL3_FIND_NEXT!(stTree, c_ref!(ulVar).cast(), stTreeInfo)
                .cast::<Ptr<UserStruct>>());
            VOS_GetAVL3Node(c_ref!(stNode4.stNode), c_ref!(ulVar2));
            assert_eq!(11, ulVar2);
            test_no_memory_leak!();
        }
    }

    #[test]
    fn ITest_AVL3_FIND_NEXT_003() {
        unsafe {
            let mut ulVar: VOS_UINT32 = 0;
            let mut stTree: AVL3_TREE = Default::default();
            let mut stNode1: UserStruct = Default::default();
            let mut stNode2: UserStruct = Default::default();
            let mut stNode3: UserStruct = Default::default();
            g_ulKeyoffset = (c_ref!(stNode1.iKey).cast::<Ptr<VOS_CHAR>>()
                - c_ref!(stNode1).cast::<Ptr<VOS_CHAR>>()) as u16;
            g_ulNodeoffset = (c_ref!(stNode1.stNode).cast::<Ptr<VOS_CHAR>>()
                - c_ref!(stNode1).cast::<Ptr<VOS_CHAR>>()) as u16;
            let mut stTreeInfo: AVL3_TREE_INFO = Default::default();
            stTreeInfo.usKeyOffset = g_ulKeyoffset;
            stTreeInfo.pfCompare = func!(AVL3_CompareFunc);
            stTreeInfo.usNodeOffset = g_ulNodeoffset;
            VOS_AVL3_INIT_TREE!(stTree, stTreeInfo);
            VOS_AVL3_INIT_NODE!(stNode1.stNode);
            stNode1.iKey = 9;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode1.stNode, stTreeInfo), true);
            ulVar = 9;
            if VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) == NULL!() {
                assert!(false);
            }
            VOS_AVL3_INIT_NODE!(stNode2.stNode);
            stNode2.iKey = 8;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode2.stNode, stTreeInfo), true);
            ulVar = 8;
            if VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) == NULL!() {
                assert!(false);
            }
            VOS_AVL3_INIT_NODE!(stNode3.stNode);
            stNode3.iKey = 11;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode3.stNode, stTreeInfo), true);
            ulVar = 11;
            if VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) == NULL!() {
                assert!(false);
            }
            ulVar = 12;
            if VOS_AVL3_FIND_NEXT!(stTree, c_ref!(ulVar).cast(), stTreeInfo) != NULL!() {
                VOS_AVL3_FIND_NEXT!(stTree, c_ref!(ulVar).cast(), stTreeInfo);
                assert!(false);
            }
            test_no_memory_leak!();
        }
    }

    #[test]
    fn ITest_AVL3_FIND_NEXT_004() {
        unsafe {
            let mut ulVar: VOS_UINT32 = 0;
            let mut stTree: AVL3_TREE = Default::default();
            let mut stNode1: UserStruct = Default::default();
            let mut stNode2: UserStruct = Default::default();
            let mut stNode3: UserStruct = Default::default();
            g_ulKeyoffset = (c_ref!(stNode1.iKey).cast::<Ptr<VOS_CHAR>>()
                - c_ref!(stNode1).cast::<Ptr<VOS_CHAR>>()) as u16;
            g_ulNodeoffset = (c_ref!(stNode1.stNode).cast::<Ptr<VOS_CHAR>>()
                - c_ref!(stNode1).cast::<Ptr<VOS_CHAR>>()) as u16;
            let mut stTreeInfo: AVL3_TREE_INFO = Default::default();
            stTreeInfo.usKeyOffset = g_ulKeyoffset;
            stTreeInfo.pfCompare = func!(AVL3_CompareFunc);
            stTreeInfo.usNodeOffset = g_ulNodeoffset;
            VOS_AVL3_INIT_TREE!(stTree, stTreeInfo);
            VOS_AVL3_INIT_NODE!(stNode1.stNode);
            stNode1.iKey = 9;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode1.stNode, stTreeInfo), true);
            ulVar = 9;
            if VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) == NULL!() {
                assert!(false);
            }
            VOS_AVL3_INIT_NODE!(stNode2.stNode);
            stNode2.iKey = 8;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode2.stNode, stTreeInfo), true);
            ulVar = 8;
            if VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) == NULL!() {
                assert!(false);
            }
            VOS_AVL3_INIT_NODE!(stNode3.stNode);
            stNode3.iKey = 11;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode3.stNode, stTreeInfo), true);
            VOS_AVL3_DELETE!(stTree, stNode2.stNode);
            VOS_AVL3_INIT_NODE!(stNode2.stNode);
            stNode2.iKey = 12;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode2.stNode, stTreeInfo), true);
            ulVar = 12;
            if VOS_AVL3_FIND_NEXT!(stTree, c_ref!(ulVar).cast(), stTreeInfo) != NULL!() {
                VOS_AVL3_FIND_NEXT!(stTree, c_ref!(ulVar).cast(), stTreeInfo);
                assert!(false);
            }
            test_no_memory_leak!();
        }
    }

    #[test]
    fn ITest_AVL3_FIND_NEXT_005() {
        unsafe {
            let mut ulVar: VOS_UINT32 = 0;
            let mut ulVar2: VOS_UINT32 = 0;
            let mut stTree: AVL3_TREE = Default::default();
            let mut stNode1: UserStruct = Default::default();
            let mut stNode2: UserStruct = Default::default();
            let mut stNode3: UserStruct = Default::default();
            let mut stNode4: UserStruct = Default::default();
            g_ulKeyoffset = (c_ref!(stNode1.iKey).cast::<Ptr<VOS_CHAR>>()
                - c_ref!(stNode1).cast::<Ptr<VOS_CHAR>>()) as u16;
            g_ulNodeoffset = (c_ref!(stNode1.stNode).cast::<Ptr<VOS_CHAR>>()
                - c_ref!(stNode1).cast::<Ptr<VOS_CHAR>>()) as u16;
            let mut stTreeInfo: AVL3_TREE_INFO = Default::default();
            stTreeInfo.usKeyOffset = g_ulKeyoffset;
            stTreeInfo.pfCompare = func!(AVL3_CompareFunc);
            stTreeInfo.usNodeOffset = g_ulNodeoffset;
            VOS_AVL3_INIT_TREE!(stTree, stTreeInfo);
            VOS_AVL3_INIT_NODE!(stNode1.stNode);
            stNode1.iKey = 9;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode1.stNode, stTreeInfo), true);
            ulVar = 9;
            if VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) == NULL!() {
                assert!(false);
            }
            VOS_AVL3_INIT_NODE!(stNode2.stNode);
            stNode2.iKey = 8;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode2.stNode, stTreeInfo), true);
            ulVar = 8;
            if VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) == NULL!() {
                assert!(false);
            }
            VOS_AVL3_INIT_NODE!(stNode3.stNode);
            stNode3.iKey = 11;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode3.stNode, stTreeInfo), true);
            ulVar = 11;
            if VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) == NULL!() {
                assert!(false);
            }
            VOS_AVL3_DELETE!(stTree, stNode2.stNode);
            ulVar = 8;
            if VOS_AVL3_FIND_NEXT!(stTree, c_ref!(ulVar).cast(), stTreeInfo) != NULL!() {
                stNode4 = *(VOS_AVL3_FIND_NEXT!(stTree, c_ref!(ulVar).cast(), stTreeInfo)
                    .cast::<Ptr<UserStruct>>());
                VOS_GetAVL3Node(c_ref!(stNode4.stNode), c_ref!(ulVar2));
                if 9 != ulVar2 {
                    assert!(false);
                }
            } else {
                assert!(false);
            }
            test_no_memory_leak!();
        }
    }

    #[test]
    fn ITest_AVL3_FIND_NEXT_006() {
        unsafe {
            let mut ulVar: VOS_UINT32 = 0;
            let mut stTree: AVL3_TREE = Default::default();
            let mut stNode1: UserStruct = Default::default();
            g_ulKeyoffset = (c_ref!(stNode1.iKey).cast::<Ptr<VOS_CHAR>>()
                - c_ref!(stNode1).cast::<Ptr<VOS_CHAR>>()) as u16;
            g_ulNodeoffset = (c_ref!(stNode1.stNode).cast::<Ptr<VOS_CHAR>>()
                - c_ref!(stNode1).cast::<Ptr<VOS_CHAR>>()) as u16;
            let mut stTreeInfo: AVL3_TREE_INFO = Default::default();
            stTreeInfo.usKeyOffset = g_ulKeyoffset;
            stTreeInfo.pfCompare = func!(AVL3_CompareFunc);
            stTreeInfo.usNodeOffset = g_ulNodeoffset;
            VOS_AVL3_INIT_TREE!(stTree, stTreeInfo);
            ulVar = 8;
            if VOS_AVL3_FIND_NEXT!(stTree, c_ref!(ulVar).cast(), stTreeInfo) != NULL!() {
                VOS_AVL3_FIND_NEXT!(stTree, c_ref!(ulVar).cast(), stTreeInfo);
                assert!(false);
            }
            test_no_memory_leak!();
        }
    }

    #[test]
    fn ITest_AVL3_FIND_NEXT_007() {
        unsafe {
            let mut ulVar: VOS_UINT32 = 0;
            let mut stTree: AVL3_TREE = Default::default();
            let mut stNode1: UserStruct = Default::default();
            let mut stNode2: UserStruct = Default::default();
            let mut stNode3: UserStruct = Default::default();
            g_ulKeyoffset = (c_ref!(stNode1.iKey).cast::<Ptr<VOS_CHAR>>()
                - c_ref!(stNode1).cast::<Ptr<VOS_CHAR>>()) as u16;
            g_ulNodeoffset = (c_ref!(stNode1.stNode).cast::<Ptr<VOS_CHAR>>()
                - c_ref!(stNode1).cast::<Ptr<VOS_CHAR>>()) as u16;
            let mut stTreeInfo: AVL3_TREE_INFO = Default::default();
            stTreeInfo.usKeyOffset = g_ulKeyoffset;
            stTreeInfo.pfCompare = func!(AVL3_CompareFunc);
            stTreeInfo.usNodeOffset = g_ulNodeoffset;
            VOS_AVL3_INIT_TREE!(stTree, stTreeInfo);
            VOS_AVL3_INIT_NODE!(stNode1.stNode);
            stNode1.iKey = 9;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode1.stNode, stTreeInfo), true);
            ulVar = 9;
            if VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) == NULL!() {
                assert!(false);
            }
            VOS_AVL3_INIT_NODE!(stNode2.stNode);
            stNode2.iKey = 8;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode2.stNode, stTreeInfo), true);
            ulVar = 8;
            if VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) == NULL!() {
                assert!(false);
            }
            VOS_AVL3_INIT_NODE!(stNode3.stNode);
            stNode3.iKey = 11;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode3.stNode, stTreeInfo), true);
            ulVar = 11;
            if VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) == NULL!() {
                assert!(false);
            }
            ulVar = 11;
            if VOS_AVL3_FIND_NEXT!(stTree, c_ref!(ulVar).cast(), stTreeInfo) != NULL!() {
                VOS_AVL3_FIND_NEXT!(stTree, c_ref!(ulVar).cast(), stTreeInfo);
                assert!(false);
            }
            test_no_memory_leak!();
        }
    }

    #[test]
    fn ITest_AVL3_FIND_OR_FIND_NEXT_001() {
        unsafe {
            let mut ulVar: VOS_UINT32 = 0;
            let mut pulVar: Ptr<VOS_UINT32>;
            let mut stTree: AVL3_TREE = Default::default();
            let mut pstNode: Ptr<AVL3_NODE>;
            let mut stNode1: UserStruct = Default::default();
            let mut stNode2: UserStruct = Default::default();
            let mut stNode3: UserStruct = Default::default();
            let mut stNode4: UserStruct = Default::default();
            g_ulKeyoffset = (c_ref!(stNode1.iKey).cast::<Ptr<VOS_CHAR>>()
                - c_ref!(stNode1).cast::<Ptr<VOS_CHAR>>()) as u16;
            g_ulNodeoffset = (c_ref!(stNode1.stNode).cast::<Ptr<VOS_CHAR>>()
                - c_ref!(stNode1).cast::<Ptr<VOS_CHAR>>()) as u16;
            let mut stTreeInfo: AVL3_TREE_INFO = Default::default();
            stTreeInfo.usKeyOffset = g_ulKeyoffset;
            stTreeInfo.pfCompare = func!(AVL3_CompareFunc);
            stTreeInfo.usNodeOffset = g_ulNodeoffset;
            VOS_AVL3_INIT_TREE!(stTree, stTreeInfo);
            VOS_AVL3_INIT_NODE!(stNode1.stNode);
            stNode1.iKey = 9;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode1.stNode, stTreeInfo), true);
            ulVar = 9;
            if VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) == NULL!() {
                assert!(false);
            }
            VOS_AVL3_INIT_NODE!(stNode2.stNode);
            stNode2.iKey = 8;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode2.stNode, stTreeInfo), true);
            ulVar = 8;
            if VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) == NULL!() {
                assert!(false);
            }
            VOS_AVL3_INIT_NODE!(stNode3.stNode);
            stNode3.iKey = 11;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode3.stNode, stTreeInfo), true);
            ulVar = 8;
            stNode4 = *(VOS_AVL3_FIND_OR_FIND_NEXT!(stTree, c_ref!(ulVar).cast(), stTreeInfo)
                .cast::<Ptr<UserStruct>>());
            pstNode = c_ref!(stNode4.stNode);
            pulVar = (pstNode.cast::<Ptr<VOS_CHAR>>() - g_ulNodeoffset + g_ulKeyoffset)
                .cast::<Ptr<VOS_UINT32>>();
            assert_eq!(8, *pulVar);
            test_no_memory_leak!();
        }
    }

    #[test]
    fn ITest_AVL3_FIND_OR_FIND_NEXT_002() {
        unsafe {
            let mut ulVar: VOS_UINT32 = 0;
            let mut ulVar2: VOS_UINT32 = 0;
            let mut stTree: AVL3_TREE = Default::default();
            let mut stNode1: UserStruct = Default::default();
            let mut stNode2: UserStruct = Default::default();
            let mut stNode3: UserStruct = Default::default();
            let mut stNode4: UserStruct = Default::default();
            g_ulKeyoffset = (c_ref!(stNode1.iKey).cast::<Ptr<VOS_CHAR>>()
                - c_ref!(stNode1).cast::<Ptr<VOS_CHAR>>()) as u16;
            g_ulNodeoffset = (c_ref!(stNode1.stNode).cast::<Ptr<VOS_CHAR>>()
                - c_ref!(stNode1).cast::<Ptr<VOS_CHAR>>()) as u16;
            let mut stTreeInfo: AVL3_TREE_INFO = Default::default();
            stTreeInfo.usKeyOffset = g_ulKeyoffset;
            stTreeInfo.pfCompare = func!(AVL3_CompareFunc);
            stTreeInfo.usNodeOffset = g_ulNodeoffset;
            VOS_AVL3_INIT_TREE!(stTree, stTreeInfo);
            VOS_AVL3_INIT_NODE!(stNode1.stNode);
            stNode1.iKey = 9;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode1.stNode, stTreeInfo), true);
            ulVar = 9;
            if VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) == NULL!() {
                assert!(false);
            }
            VOS_AVL3_INIT_NODE!(stNode2.stNode);
            stNode2.iKey = 8;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode2.stNode, stTreeInfo), true);
            ulVar = 8;
            if VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) == NULL!() {
                assert!(false);
            }
            VOS_AVL3_INIT_NODE!(stNode3.stNode);
            stNode3.iKey = 11;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode3.stNode, stTreeInfo), true);
            ulVar = 9;
            stNode4 = *(VOS_AVL3_FIND_OR_FIND_NEXT!(stTree, c_ref!(ulVar).cast(), stTreeInfo)
                .cast::<Ptr<UserStruct>>());
            VOS_GetAVL3Node(c_ref!(stNode4.stNode), c_ref!(ulVar2));
            assert_eq!(9, ulVar2);
            test_no_memory_leak!();
        }
    }

    #[test]
    fn ITest_AVL3_FIND_OR_FIND_NEXT_003() {
        unsafe {
            let mut ulVar: VOS_UINT32 = 0;
            let mut ulVar2: VOS_UINT32 = 0;
            let mut stTree: AVL3_TREE = Default::default();
            let mut stNode1: UserStruct = Default::default();
            let mut stNode2: UserStruct = Default::default();
            let mut stNode3: UserStruct = Default::default();
            let mut stNode4: UserStruct = Default::default();
            g_ulKeyoffset = (c_ref!(stNode1.iKey).cast::<Ptr<VOS_CHAR>>()
                - c_ref!(stNode1).cast::<Ptr<VOS_CHAR>>()) as u16;
            g_ulNodeoffset = (c_ref!(stNode1.stNode).cast::<Ptr<VOS_CHAR>>()
                - c_ref!(stNode1).cast::<Ptr<VOS_CHAR>>()) as u16;
            let mut stTreeInfo: AVL3_TREE_INFO = Default::default();
            stTreeInfo.usKeyOffset = g_ulKeyoffset;
            stTreeInfo.pfCompare = func!(AVL3_CompareFunc);
            stTreeInfo.usNodeOffset = g_ulNodeoffset;
            VOS_AVL3_INIT_TREE!(stTree, stTreeInfo);
            VOS_AVL3_INIT_NODE!(stNode1.stNode);
            stNode1.iKey = 9;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode1.stNode, stTreeInfo), true);
            ulVar = 9;
            if VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) == NULL!() {
                assert!(false);
            }
            VOS_AVL3_INIT_NODE!(stNode2.stNode);
            stNode2.iKey = 8;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode2.stNode, stTreeInfo), true);
            ulVar = 8;
            if VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) == NULL!() {
                assert!(false);
            }
            VOS_AVL3_INIT_NODE!(stNode3.stNode);
            stNode3.iKey = 11;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode3.stNode, stTreeInfo), true);
            ulVar = 12;
            if VOS_AVL3_FIND_OR_FIND_NEXT!(stTree, c_ref!(ulVar).cast(), stTreeInfo) != NULL!() {
                stNode4 = *(VOS_AVL3_FIND_OR_FIND_NEXT!(stTree, c_ref!(ulVar).cast(), stTreeInfo)
                    .cast::<Ptr<UserStruct>>());
                VOS_GetAVL3Node(c_ref!(stNode4.stNode), c_ref!(ulVar2));
                assert!(false);
            }
            test_no_memory_leak!();
        }
    }

    #[test]
    fn ITest_AVL3_FIND_OR_FIND_NEXT_004() {
        unsafe {
            let mut ulVar: VOS_UINT32 = 0;
            let mut ulVar2: VOS_UINT32 = 0;
            let mut stTree: AVL3_TREE = Default::default();
            let mut stNode1: UserStruct = Default::default();
            let mut stNode2: UserStruct = Default::default();
            let mut stNode3: UserStruct = Default::default();
            let mut stNode4: UserStruct = Default::default();
            g_ulKeyoffset = (c_ref!(stNode1.iKey).cast::<Ptr<VOS_CHAR>>()
                - c_ref!(stNode1).cast::<Ptr<VOS_CHAR>>()) as u16;
            g_ulNodeoffset = (c_ref!(stNode1.stNode).cast::<Ptr<VOS_CHAR>>()
                - c_ref!(stNode1).cast::<Ptr<VOS_CHAR>>()) as u16;
            let mut stTreeInfo: AVL3_TREE_INFO = Default::default();
            stTreeInfo.usKeyOffset = g_ulKeyoffset;
            stTreeInfo.pfCompare = func!(AVL3_CompareFunc);
            stTreeInfo.usNodeOffset = g_ulNodeoffset;
            VOS_AVL3_INIT_TREE!(stTree, stTreeInfo);
            VOS_AVL3_INIT_NODE!(stNode1.stNode);
            stNode1.iKey = 9;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode1.stNode, stTreeInfo), true);
            ulVar = 9;
            if VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) == NULL!() {
                assert!(false);
            }
            VOS_AVL3_INIT_NODE!(stNode2.stNode);
            stNode2.iKey = 8;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode2.stNode, stTreeInfo), true);
            ulVar = 8;
            if VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) == NULL!() {
                assert!(false);
            }
            VOS_AVL3_INIT_NODE!(stNode3.stNode);
            stNode3.iKey = 11;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode3.stNode, stTreeInfo), true);
            ulVar = 11;
            if VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) == NULL!() {
                assert!(false);
            }
            VOS_AVL3_DELETE!(stTree, stNode2.stNode);
            ulVar = 8;
            if VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) != NULL!() {
                assert!(false);
            }
            VOS_AVL3_INIT_NODE!(stNode2.stNode);
            stNode2.iKey = 12;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode2.stNode, stTreeInfo), true);
            ulVar = 12;
            if VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) == NULL!() {
                assert!(false);
            }
            ulVar = 12;
            if VOS_AVL3_FIND_OR_FIND_NEXT!(stTree, c_ref!(ulVar).cast(), stTreeInfo) != NULL!() {
                stNode4 = *(VOS_AVL3_FIND_OR_FIND_NEXT!(stTree, c_ref!(ulVar).cast(), stTreeInfo)
                    .cast::<Ptr<UserStruct>>());
                VOS_GetAVL3Node(c_ref!(stNode4.stNode), c_ref!(ulVar2));
                assert_eq!(12, ulVar2);
            } else {
                assert!(false);
            }
            test_no_memory_leak!();
        }
    }

    #[test]
    fn ITest_AVL3_FIND_OR_FIND_NEXT_005() {
        unsafe {
            let mut ulVar: VOS_UINT32 = 0;
            let mut ulVar2: VOS_UINT32 = 0;
            let mut stTree: AVL3_TREE = Default::default();
            let mut stNode1: UserStruct = Default::default();
            let mut stNode2: UserStruct = Default::default();
            let mut stNode3: UserStruct = Default::default();
            let mut stNode4: UserStruct = Default::default();
            g_ulKeyoffset = (c_ref!(stNode1.iKey).cast::<Ptr<VOS_CHAR>>()
                - c_ref!(stNode1).cast::<Ptr<VOS_CHAR>>()) as u16;
            g_ulNodeoffset = (c_ref!(stNode1.stNode).cast::<Ptr<VOS_CHAR>>()
                - c_ref!(stNode1).cast::<Ptr<VOS_CHAR>>()) as u16;
            let mut stTreeInfo: AVL3_TREE_INFO = Default::default();
            stTreeInfo.usKeyOffset = g_ulKeyoffset;
            stTreeInfo.pfCompare = func!(AVL3_CompareFunc);
            stTreeInfo.usNodeOffset = g_ulNodeoffset;
            VOS_AVL3_INIT_TREE!(stTree, stTreeInfo);
            VOS_AVL3_INIT_NODE!(stNode1.stNode);
            stNode1.iKey = 9;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode1.stNode, stTreeInfo), true);
            ulVar = 9;
            if VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) == NULL!() {
                assert!(false);
            }
            VOS_AVL3_INIT_NODE!(stNode2.stNode);
            stNode2.iKey = 8;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode2.stNode, stTreeInfo), true);
            ulVar = 8;
            if VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) == NULL!() {
                assert!(false);
            }
            VOS_AVL3_INIT_NODE!(stNode3.stNode);
            stNode3.iKey = 11;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode3.stNode, stTreeInfo), true);
            ulVar = 11;
            if VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) == NULL!() {
                assert!(false);
            }
            VOS_AVL3_DELETE!(stTree, stNode2.stNode);
            ulVar = 8;
            if VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) != NULL!() {
                assert!(false);
            }
            if VOS_AVL3_FIND_OR_FIND_NEXT!(stTree, c_ref!(ulVar).cast(), stTreeInfo) != NULL!() {
                stNode4 = *(VOS_AVL3_FIND_OR_FIND_NEXT!(stTree, c_ref!(ulVar).cast(), stTreeInfo)
                    .cast::<Ptr<UserStruct>>());
                VOS_GetAVL3Node(c_ref!(stNode4.stNode), c_ref!(ulVar2));
                assert_eq!(9, ulVar2);
            } else {
                assert!(false);
            }
            test_no_memory_leak!();
        }
    }

    #[test]
    fn ITest_AVL3_FIND_OR_FIND_NEXT_006() {
        unsafe {
            let mut ulVar: VOS_UINT32 = 8;
            let mut stTree: AVL3_TREE = Default::default();
            let mut stNode1: UserStruct = Default::default();
            g_ulKeyoffset = (c_ref!(stNode1.iKey).cast::<Ptr<VOS_CHAR>>()
                - c_ref!(stNode1).cast::<Ptr<VOS_CHAR>>()) as u16;
            g_ulNodeoffset = (c_ref!(stNode1.stNode).cast::<Ptr<VOS_CHAR>>()
                - c_ref!(stNode1).cast::<Ptr<VOS_CHAR>>()) as u16;
            let mut stTreeInfo: AVL3_TREE_INFO = Default::default();
            stTreeInfo.usKeyOffset = g_ulKeyoffset;
            stTreeInfo.pfCompare = func!(AVL3_CompareFunc);
            stTreeInfo.usNodeOffset = g_ulNodeoffset;
            VOS_AVL3_INIT_TREE!(stTree, stTreeInfo);
            if VOS_AVL3_FIND_OR_FIND_NEXT!(stTree, c_ref!(ulVar).cast(), stTreeInfo) != NULL!() {
                VOS_AVL3_FIND_OR_FIND_NEXT!(stTree, c_ref!(ulVar).cast(), stTreeInfo);
                assert!(false);
            }
            test_no_memory_leak!();
        }
    }

    #[test]
    fn ITest_AVL3_FIND_OR_FIND_NEXT_007() {
        unsafe {
            let mut ulVar: VOS_UINT32 = 0;
            let mut ulVar2: VOS_UINT32 = 0;
            let mut stTree: AVL3_TREE = Default::default();
            let mut stNode1: UserStruct = Default::default();
            let mut stNode2: UserStruct = Default::default();
            let mut stNode3: UserStruct = Default::default();
            let mut stNode4: UserStruct = Default::default();
            g_ulKeyoffset = (c_ref!(stNode1.iKey).cast::<Ptr<VOS_CHAR>>()
                - c_ref!(stNode1).cast::<Ptr<VOS_CHAR>>()) as u16;
            g_ulNodeoffset = (c_ref!(stNode1.stNode).cast::<Ptr<VOS_CHAR>>()
                - c_ref!(stNode1).cast::<Ptr<VOS_CHAR>>()) as u16;
            let mut stTreeInfo: AVL3_TREE_INFO = Default::default();
            stTreeInfo.usKeyOffset = g_ulKeyoffset;
            stTreeInfo.pfCompare = func!(AVL3_CompareFunc);
            stTreeInfo.usNodeOffset = g_ulNodeoffset;
            VOS_AVL3_INIT_TREE!(stTree, stTreeInfo);
            VOS_AVL3_INIT_NODE!(stNode1.stNode);
            stNode1.iKey = 9;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode1.stNode, stTreeInfo), true);
            ulVar = 9;
            if VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) == NULL!() {
                assert!(false);
            }
            VOS_AVL3_INIT_NODE!(stNode2.stNode);
            stNode2.iKey = 8;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode2.stNode, stTreeInfo), true);
            ulVar = 8;
            if VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) == NULL!() {
                assert!(false);
            }
            VOS_AVL3_INIT_NODE!(stNode3.stNode);
            stNode3.iKey = 11;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode3.stNode, stTreeInfo), true);
            ulVar = 11;
            if VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) == NULL!() {
                assert!(false);
            }
            ulVar = 11;
            if VOS_AVL3_FIND_OR_FIND_NEXT!(stTree, c_ref!(ulVar).cast(), stTreeInfo) != NULL!() {
                stNode4 = *(VOS_AVL3_FIND_OR_FIND_NEXT!(stTree, c_ref!(ulVar).cast(), stTreeInfo)
                    .cast::<Ptr<UserStruct>>());
                VOS_GetAVL3Node(c_ref!(stNode4.stNode), c_ref!(ulVar2));
                assert_eq!(11, ulVar2);
            } else {
                assert!(false);
            }
            test_no_memory_leak!();
        }
    }

    #[test]
    fn ITest_AVL3_FIRST_001() {
        unsafe {
            let mut ulVar: VOS_UINT32 = 0;
            let mut ulVar2: VOS_UINT32 = 0;
            let mut stTree: AVL3_TREE = Default::default();
            let mut stNode1: UserStruct = Default::default();
            let mut stNode2: UserStruct = Default::default();
            let mut stNode3: UserStruct = Default::default();
            let mut stNode4: UserStruct = Default::default();
            let mut stNode5: UserStruct = Default::default();
            g_ulKeyoffset = (c_ref!(stNode1.iKey).cast::<Ptr<VOS_CHAR>>()
                - c_ref!(stNode1).cast::<Ptr<VOS_CHAR>>()) as u16;
            g_ulNodeoffset = (c_ref!(stNode1.stNode).cast::<Ptr<VOS_CHAR>>()
                - c_ref!(stNode1).cast::<Ptr<VOS_CHAR>>()) as u16;
            let mut stTreeInfo: AVL3_TREE_INFO = Default::default();
            stTreeInfo.usKeyOffset = g_ulKeyoffset;
            stTreeInfo.pfCompare = func!(AVL3_CompareFunc);
            stTreeInfo.usNodeOffset = g_ulNodeoffset;
            VOS_AVL3_INIT_TREE!(stTree, stTreeInfo);
            VOS_AVL3_INIT_NODE!(stNode1.stNode);
            stNode1.iKey = 9;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode1.stNode, stTreeInfo), true);
            ulVar = 9;
            if VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) == NULL!() {
                assert!(false);
            }
            VOS_AVL3_INIT_NODE!(stNode2.stNode);
            stNode2.iKey = 8;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode2.stNode, stTreeInfo), true);
            ulVar = 8;
            if VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) == NULL!() {
                assert!(false);
            }
            VOS_AVL3_INIT_NODE!(stNode3.stNode);
            stNode3.iKey = 11;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode3.stNode, stTreeInfo), true);
            ulVar = 11;
            if VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) == NULL!() {
                assert!(false);
            }
            VOS_AVL3_INIT_NODE!(stNode4.stNode);
            stNode4.iKey = 12;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode4.stNode, stTreeInfo), true);
            ulVar = 12;
            if VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) == NULL!() {
                assert!(false);
            }
            ulVar = 8;
            if VOS_AVL3_FIRST!(stTree, stTreeInfo) != NULL!() {
                stNode5 = *(VOS_AVL3_FIRST!(stTree, stTreeInfo).cast::<Ptr<UserStruct>>());
                VOS_GetAVL3Node(c_ref!(stNode5.stNode), c_ref!(ulVar2));
                assert_eq!(8, ulVar2);
            } else {
                assert!(false);
            }
            test_no_memory_leak!();
        }
    }

    #[test]
    fn ITest_AVL3_FIRST_002() {
        unsafe {
            let mut stTree: AVL3_TREE = Default::default();
            let mut stNode1: UserStruct = Default::default();
            g_ulKeyoffset = (c_ref!(stNode1.iKey).cast::<Ptr<VOS_CHAR>>()
                - c_ref!(stNode1).cast::<Ptr<VOS_CHAR>>()) as u16;
            g_ulNodeoffset = (c_ref!(stNode1.stNode).cast::<Ptr<VOS_CHAR>>()
                - c_ref!(stNode1).cast::<Ptr<VOS_CHAR>>()) as u16;
            let mut stTreeInfo: AVL3_TREE_INFO = Default::default();
            stTreeInfo.usKeyOffset = g_ulKeyoffset;
            stTreeInfo.pfCompare = func!(AVL3_CompareFunc);
            stTreeInfo.usNodeOffset = g_ulNodeoffset;
            VOS_AVL3_INIT_TREE!(stTree, stTreeInfo);
            if VOS_AVL3_FIRST!(stTree, stTreeInfo) != NULL!() {
                VOS_AVL3_FIRST!(stTree, stTreeInfo);
                assert!(false);
            }
            test_no_memory_leak!();
        }
    }

    #[test]
    fn ITest_AVL3_FIRST_003() {
        unsafe {
            let mut ulVar: VOS_UINT32 = 0;
            let mut stTree: AVL3_TREE = Default::default();
            let mut stNode1: UserStruct = Default::default();
            g_ulKeyoffset = (c_ref!(stNode1.iKey).cast::<Ptr<VOS_CHAR>>()
                - c_ref!(stNode1).cast::<Ptr<VOS_CHAR>>()) as u16;
            g_ulNodeoffset = (c_ref!(stNode1.stNode).cast::<Ptr<VOS_CHAR>>()
                - c_ref!(stNode1).cast::<Ptr<VOS_CHAR>>()) as u16;
            let mut stTreeInfo: AVL3_TREE_INFO = Default::default();
            stTreeInfo.usKeyOffset = g_ulKeyoffset;
            stTreeInfo.pfCompare = func!(AVL3_CompareFunc);
            stTreeInfo.usNodeOffset = g_ulNodeoffset;
            VOS_AVL3_INIT_TREE!(stTree, stTreeInfo);
            VOS_AVL3_INIT_NODE!(stNode1.stNode);
            stNode1.iKey = 9;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode1.stNode, stTreeInfo), true);
            ulVar = 9;
            if VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) == NULL!() {
                assert!(false);
            }
            VOS_AVL3_DELETE!(stTree, stNode1.stNode);
            if VOS_AVL3_FIRST!(stTree, stTreeInfo) != NULL!() {
                VOS_AVL3_FIRST!(stTree, stTreeInfo);
                assert!(false);
            }
            test_no_memory_leak!();
        }
    }

    #[test]
    fn ITest_AVL3_IN_TREE_001() {
        unsafe {
            let mut ulVar: VOS_UINT32 = 0;
            let mut stTree: AVL3_TREE = Default::default();
            let mut stNode1: UserStruct = Default::default();
            g_ulKeyoffset = (c_ref!(stNode1.iKey).cast::<Ptr<VOS_CHAR>>()
                - c_ref!(stNode1).cast::<Ptr<VOS_CHAR>>()) as u16;
            g_ulNodeoffset = (c_ref!(stNode1.stNode).cast::<Ptr<VOS_CHAR>>()
                - c_ref!(stNode1).cast::<Ptr<VOS_CHAR>>()) as u16;
            let mut stTreeInfo: AVL3_TREE_INFO = Default::default();
            stTreeInfo.usKeyOffset = g_ulKeyoffset;
            stTreeInfo.pfCompare = func!(AVL3_CompareFunc);
            stTreeInfo.usNodeOffset = g_ulNodeoffset;
            VOS_AVL3_INIT_TREE!(stTree, stTreeInfo);
            VOS_AVL3_INIT_NODE!(stNode1.stNode);
            stNode1.iKey = 9;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode1.stNode, stTreeInfo), true);
            ulVar = 9;
            if VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) == NULL!() {
                assert!(false);
            }
            if VOS_AVL3_IN_TREE!(stNode1.stNode) as i32 == 0 {
                assert!(false);
            }
            test_no_memory_leak!();
        }
    }

    #[test]
    fn ITest_AVL3_IN_TREE_002() {
        unsafe {
            let mut ulVar: VOS_UINT32 = 0;
            let mut stTree: AVL3_TREE = Default::default();
            let mut stNode1: UserStruct = Default::default();
            let mut stNode2: UserStruct = Default::default();
            let mut stNode3: UserStruct = Default::default();
            g_ulKeyoffset = (c_ref!(stNode1.iKey).cast::<Ptr<VOS_CHAR>>()
                - c_ref!(stNode1).cast::<Ptr<VOS_CHAR>>()) as u16;
            g_ulNodeoffset = (c_ref!(stNode1.stNode).cast::<Ptr<VOS_CHAR>>()
                - c_ref!(stNode1).cast::<Ptr<VOS_CHAR>>()) as u16;
            let mut stTreeInfo: AVL3_TREE_INFO = Default::default();
            stTreeInfo.usKeyOffset = g_ulKeyoffset;
            stTreeInfo.pfCompare = func!(AVL3_CompareFunc);
            stTreeInfo.usNodeOffset = g_ulNodeoffset;
            VOS_AVL3_INIT_TREE!(stTree, stTreeInfo);
            VOS_AVL3_INIT_NODE!(stNode1.stNode);
            stNode1.iKey = 9;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode1.stNode, stTreeInfo), true);
            ulVar = 9;
            if VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) == NULL!() {
                assert!(false);
            }
            VOS_AVL3_INIT_NODE!(stNode2.stNode);
            stNode2.iKey = 45;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode2.stNode, stTreeInfo), true);
            ulVar = 45;
            if VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) == NULL!() {
                assert!(false);
            }
            VOS_AVL3_INIT_NODE!(stNode3.stNode);
            if VOS_AVL3_IN_TREE!(stNode3.stNode) as i32 == VOS_TRUE!() {
                assert!(false);
            }
            test_no_memory_leak!();
        }
    }

    #[test]
    fn ITest_AVL3_IN_TREE_003() {
        unsafe {
            let mut ulVar: VOS_UINT32 = 0;
            let mut stTree: AVL3_TREE = Default::default();
            let mut stNode1: UserStruct = Default::default();
            let mut stNode2: UserStruct = Default::default();
            g_ulKeyoffset = (c_ref!(stNode1.iKey).cast::<Ptr<VOS_CHAR>>()
                - c_ref!(stNode1).cast::<Ptr<VOS_CHAR>>()) as u16;
            g_ulNodeoffset = (c_ref!(stNode1.stNode).cast::<Ptr<VOS_CHAR>>()
                - c_ref!(stNode1).cast::<Ptr<VOS_CHAR>>()) as u16;
            let mut stTreeInfo: AVL3_TREE_INFO = Default::default();
            stTreeInfo.usKeyOffset = g_ulKeyoffset;
            stTreeInfo.pfCompare = func!(AVL3_CompareFunc);
            stTreeInfo.usNodeOffset = g_ulNodeoffset;
            VOS_AVL3_INIT_TREE!(stTree, stTreeInfo);
            VOS_AVL3_INIT_NODE!(stNode1.stNode);
            stNode1.iKey = 9;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode1.stNode, stTreeInfo), true);
            ulVar = 9;
            if VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) == NULL!() {
                assert!(false);
            }
            VOS_AVL3_INIT_NODE!(stNode2.stNode);
            stNode2.iKey = 45;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode2.stNode, stTreeInfo), true);
            ulVar = 45;
            if VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) == NULL!() {
                assert!(false);
            }
            VOS_AVL3_DELETE!(stTree, stNode2.stNode);
            ulVar = 45;
            if VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) != NULL!() {
                assert!(false);
            }
            if VOS_AVL3_IN_TREE!(stNode2.stNode) as i32 == VOS_TRUE!() {
                assert!(false);
            }
            test_no_memory_leak!();
        }
    }

    #[test]
    fn ITest_AVL3_LAST_001() {
        unsafe {
            let mut ulVar: VOS_UINT32 = 0;
            let mut ulVar2: VOS_UINT32 = 0;
            let mut stTree: AVL3_TREE = Default::default();
            let mut stNode1: UserStruct = Default::default();
            let mut stNode2: UserStruct = Default::default();
            let mut stNode3: UserStruct = Default::default();
            let mut stNode4: UserStruct = Default::default();
            let mut stNode5: UserStruct = Default::default();
            g_ulKeyoffset = (c_ref!(stNode1.iKey).cast::<Ptr<VOS_CHAR>>()
                - c_ref!(stNode1).cast::<Ptr<VOS_CHAR>>()) as u16;
            g_ulNodeoffset = (c_ref!(stNode1.stNode).cast::<Ptr<VOS_CHAR>>()
                - c_ref!(stNode1).cast::<Ptr<VOS_CHAR>>()) as u16;
            let mut stTreeInfo: AVL3_TREE_INFO = Default::default();
            stTreeInfo.usKeyOffset = g_ulKeyoffset;
            stTreeInfo.pfCompare = func!(AVL3_CompareFunc);
            stTreeInfo.usNodeOffset = g_ulNodeoffset;
            VOS_AVL3_INIT_TREE!(stTree, stTreeInfo);
            VOS_AVL3_INIT_NODE!(stNode1.stNode);
            stNode1.iKey = 9;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode1.stNode, stTreeInfo), true);
            ulVar = 9;
            if VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) == NULL!() {
                assert!(false);
            }
            VOS_AVL3_INIT_NODE!(stNode2.stNode);
            stNode2.iKey = 8;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode2.stNode, stTreeInfo), true);
            ulVar = 8;
            if VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) == NULL!() {
                assert!(false);
            }
            VOS_AVL3_INIT_NODE!(stNode3.stNode);
            stNode3.iKey = 11;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode3.stNode, stTreeInfo), true);
            ulVar = 11;
            if VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) == NULL!() {
                assert!(false);
            }
            VOS_AVL3_INIT_NODE!(stNode4.stNode);
            stNode4.iKey = 12;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode4.stNode, stTreeInfo), true);
            ulVar = 12;
            if VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) == NULL!() {
                assert!(false);
            }
            if VOS_AVL3_LAST!(stTree, stTreeInfo) != NULL!() {
                stNode5 = *(VOS_AVL3_LAST!(stTree, stTreeInfo).cast::<Ptr<UserStruct>>());
                VOS_GetAVL3Node(c_ref!(stNode5.stNode), c_ref!(ulVar2));
                assert_eq!(12, ulVar2);
            } else {
                assert!(false);
            }
            test_no_memory_leak!();
        }
    }

    #[test]
    fn ITest_AVL3_LAST_002() {
        unsafe {
            let mut stTree: AVL3_TREE = Default::default();
            let mut stNode1: UserStruct = Default::default();
            g_ulKeyoffset = (c_ref!(stNode1.iKey).cast::<Ptr<VOS_CHAR>>()
                - c_ref!(stNode1).cast::<Ptr<VOS_CHAR>>()) as u16;
            g_ulNodeoffset = (c_ref!(stNode1.stNode).cast::<Ptr<VOS_CHAR>>()
                - c_ref!(stNode1).cast::<Ptr<VOS_CHAR>>()) as u16;
            let mut stTreeInfo: AVL3_TREE_INFO = Default::default();
            stTreeInfo.usKeyOffset = g_ulKeyoffset;
            stTreeInfo.pfCompare = func!(AVL3_CompareFunc);
            stTreeInfo.usNodeOffset = g_ulNodeoffset;
            VOS_AVL3_INIT_TREE!(stTree, stTreeInfo);
            if VOS_AVL3_LAST!(stTree, stTreeInfo) != NULL!() {
                VOS_AVL3_LAST!(stTree, stTreeInfo);
                assert!(false);
            }
            test_no_memory_leak!();
        }
    }

    #[test]
    fn ITest_AVL3_NEXT_001() {
        unsafe {
            let mut stTreeInfo: AVL3_TREE_INFO = Default::default();
            let mut stNode1: UserStruct = Default::default();
            g_ulKeyoffset = (c_ref!(stNode1.iKey).cast::<Ptr<VOS_CHAR>>()
                - c_ref!(stNode1).cast::<Ptr<VOS_CHAR>>()) as u16;
            g_ulNodeoffset = (c_ref!(stNode1.stNode).cast::<Ptr<VOS_CHAR>>()
                - c_ref!(stNode1).cast::<Ptr<VOS_CHAR>>()) as u16;
            stTreeInfo.usKeyOffset = g_ulKeyoffset;
            stTreeInfo.pfCompare = func!(AVL3_CompareFunc);
            stTreeInfo.usNodeOffset = g_ulNodeoffset;
            VOS_AVL3_INIT_NODE!(stNode1.stNode);
            if VOS_AVL3_NEXT!(stNode1.stNode, stTreeInfo) != NULL!() {
                VOS_AVL3_NEXT!(stNode1.stNode, stTreeInfo);
                assert!(false);
            }
            test_no_memory_leak!();
        }
    }

    #[test]
    fn ITest_AVL3_NEXT_002() {
        unsafe {
            let mut ulVar: VOS_UINT32 = 0;
            let mut stTree: AVL3_TREE = Default::default();
            let mut stNode1: UserStruct = Default::default();
            g_ulKeyoffset = (c_ref!(stNode1.iKey).cast::<Ptr<VOS_CHAR>>()
                - c_ref!(stNode1).cast::<Ptr<VOS_CHAR>>()) as u16;
            g_ulNodeoffset = (c_ref!(stNode1.stNode).cast::<Ptr<VOS_CHAR>>()
                - c_ref!(stNode1).cast::<Ptr<VOS_CHAR>>()) as u16;
            let mut stTreeInfo: AVL3_TREE_INFO = Default::default();
            stTreeInfo.usKeyOffset = g_ulKeyoffset;
            stTreeInfo.pfCompare = func!(AVL3_CompareFunc);
            stTreeInfo.usNodeOffset = g_ulNodeoffset;
            VOS_AVL3_INIT_TREE!(stTree, stTreeInfo);
            VOS_AVL3_INIT_NODE!(stNode1.stNode);
            stNode1.iKey = 9;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode1.stNode, stTreeInfo), true);
            ulVar = 9;
            if VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) == NULL!() {
                assert!(false);
            }
            if VOS_AVL3_NEXT!(stNode1.stNode, stTreeInfo) != NULL!() {
                VOS_AVL3_NEXT!(stNode1.stNode, stTreeInfo);
                assert!(false);
            }
            test_no_memory_leak!();
        }
    }

    #[test]
    fn ITest_AVL3_NEXT_003() {
        unsafe {
            let mut ulVar: VOS_UINT32 = 0;
            let mut ulVar2: VOS_UINT32 = 0;
            let mut stTree: AVL3_TREE = Default::default();
            let mut stNode1: UserStruct = Default::default();
            let mut stNode2: UserStruct = Default::default();
            let mut stNode3: UserStruct = Default::default();
            let mut stNode5: UserStruct = Default::default();
            g_ulKeyoffset = (c_ref!(stNode1.iKey).cast::<Ptr<VOS_CHAR>>()
                - c_ref!(stNode1).cast::<Ptr<VOS_CHAR>>()) as u16;
            g_ulNodeoffset = (c_ref!(stNode1.stNode).cast::<Ptr<VOS_CHAR>>()
                - c_ref!(stNode1).cast::<Ptr<VOS_CHAR>>()) as u16;
            let mut stTreeInfo: AVL3_TREE_INFO = Default::default();
            stTreeInfo.usKeyOffset = g_ulKeyoffset;
            stTreeInfo.pfCompare = func!(AVL3_CompareFunc);
            stTreeInfo.usNodeOffset = g_ulNodeoffset;
            VOS_AVL3_INIT_TREE!(stTree, stTreeInfo);
            VOS_AVL3_INIT_NODE!(stNode1.stNode);
            stNode1.iKey = 9;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode1.stNode, stTreeInfo), true);
            ulVar = 9;
            if VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) == NULL!() {
                assert!(false);
            }
            VOS_AVL3_INIT_NODE!(stNode2.stNode);
            stNode2.iKey = 19;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode2.stNode, stTreeInfo), true);
            ulVar = 19;
            if VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) == NULL!() {
                assert!(false);
            }
            VOS_AVL3_INIT_NODE!(stNode3.stNode);
            stNode3.iKey = 10;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode3.stNode, stTreeInfo), true);
            ulVar = 10;
            if VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) == NULL!() {
                assert!(false);
            }
            if VOS_AVL3_NEXT!(stNode3.stNode, stTreeInfo) != NULL!() {
                stNode5 = *(VOS_AVL3_NEXT!(stNode3.stNode, stTreeInfo).cast::<Ptr<UserStruct>>());
                VOS_GetAVL3Node(c_ref!(stNode5.stNode), c_ref!(ulVar2));
                assert_eq!(19, ulVar2);
            } else {
                assert!(false);
            }
            test_no_memory_leak!();
        }
    }

    #[test]
    fn ITest_AVL3_NEXT_004() {
        unsafe {
            let mut ulVar: VOS_UINT32 = 0;
            let mut stTree: AVL3_TREE = Default::default();
            let mut stNode1: UserStruct = Default::default();
            let mut stNode2: UserStruct = Default::default();
            let mut stNode3: UserStruct = Default::default();
            g_ulKeyoffset = (c_ref!(stNode1.iKey).cast::<Ptr<VOS_CHAR>>()
                - c_ref!(stNode1).cast::<Ptr<VOS_CHAR>>()) as u16;
            g_ulNodeoffset = (c_ref!(stNode1.stNode).cast::<Ptr<VOS_CHAR>>()
                - c_ref!(stNode1).cast::<Ptr<VOS_CHAR>>()) as u16;
            let mut stTreeInfo: AVL3_TREE_INFO = Default::default();
            stTreeInfo.usKeyOffset = g_ulKeyoffset;
            stTreeInfo.pfCompare = func!(AVL3_CompareFunc);
            stTreeInfo.usNodeOffset = g_ulNodeoffset;
            VOS_AVL3_INIT_TREE!(stTree, stTreeInfo);
            VOS_AVL3_INIT_NODE!(stNode1.stNode);
            stNode1.iKey = 9;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode1.stNode, stTreeInfo), true);
            ulVar = 9;
            if VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) == NULL!() {
                assert!(false);
            }
            VOS_AVL3_INIT_NODE!(stNode2.stNode);
            stNode2.iKey = 19;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode2.stNode, stTreeInfo), true);
            ulVar = 19;
            if VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) == NULL!() {
                assert!(false);
            }
            VOS_AVL3_INIT_NODE!(stNode3.stNode);
            stNode3.iKey = 10;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode3.stNode, stTreeInfo), true);
            ulVar = 10;
            if VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) == NULL!() {
                assert!(false);
            }
            if VOS_AVL3_NEXT!(stNode2.stNode, stTreeInfo) != NULL!() {
                VOS_AVL3_NEXT!(stNode2.stNode, stTreeInfo);
                assert!(false);
            }
            test_no_memory_leak!();
        }
    }

    #[test]
    fn ITest_AVL3_PREV_001() {
        unsafe {
            let mut ulVar: VOS_UINT32 = 0;
            let mut stTree: AVL3_TREE = Default::default();
            let mut stNode1: UserStruct = Default::default();
            g_ulKeyoffset = (c_ref!(stNode1.iKey).cast::<Ptr<VOS_CHAR>>()
                - c_ref!(stNode1).cast::<Ptr<VOS_CHAR>>()) as u16;
            g_ulNodeoffset = (c_ref!(stNode1.stNode).cast::<Ptr<VOS_CHAR>>()
                - c_ref!(stNode1).cast::<Ptr<VOS_CHAR>>()) as u16;
            let mut stTreeInfo: AVL3_TREE_INFO = Default::default();
            stTreeInfo.usKeyOffset = g_ulKeyoffset;
            stTreeInfo.pfCompare = func!(AVL3_CompareFunc);
            stTreeInfo.usNodeOffset = g_ulNodeoffset;
            VOS_AVL3_INIT_TREE!(stTree, stTreeInfo);
            VOS_AVL3_INIT_NODE!(stNode1.stNode);
            stNode1.iKey = 9;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode1.stNode, stTreeInfo), true);
            ulVar = 9;
            if VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) == NULL!() {
                assert!(false);
            }
            if VOS_AVL3_PREV!(stNode1.stNode, stTreeInfo) != NULL!() {
                VOS_AVL3_PREV!(stNode1.stNode, stTreeInfo);
                assert!(false);
            }
            test_no_memory_leak!();
        }
    }

    #[test]
    fn ITest_AVL3_PREV_002() {
        unsafe {
            let mut ulVar: VOS_UINT32 = 0;
            let mut ulVar2: VOS_UINT32 = 0;
            let mut stTree: AVL3_TREE = Default::default();
            let mut stNode1: UserStruct = Default::default();
            let mut stNode2: UserStruct = Default::default();
            let mut stNode3: UserStruct = Default::default();
            let mut stNode5: UserStruct = Default::default();
            g_ulKeyoffset = (c_ref!(stNode1.iKey).cast::<Ptr<VOS_CHAR>>()
                - c_ref!(stNode1).cast::<Ptr<VOS_CHAR>>()) as u16;
            g_ulNodeoffset = (c_ref!(stNode1.stNode).cast::<Ptr<VOS_CHAR>>()
                - c_ref!(stNode1).cast::<Ptr<VOS_CHAR>>()) as u16;
            let mut stTreeInfo: AVL3_TREE_INFO = Default::default();
            stTreeInfo.usKeyOffset = g_ulKeyoffset;
            stTreeInfo.pfCompare = func!(AVL3_CompareFunc);
            stTreeInfo.usNodeOffset = g_ulNodeoffset;
            VOS_AVL3_INIT_TREE!(stTree, stTreeInfo);
            VOS_AVL3_INIT_NODE!(stNode1.stNode);
            stNode1.iKey = 9;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode1.stNode, stTreeInfo), true);
            ulVar = 9;
            if VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) == NULL!() {
                assert!(false);
            }
            VOS_AVL3_INIT_NODE!(stNode2.stNode);
            stNode2.iKey = 19;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode2.stNode, stTreeInfo), true);
            ulVar = 19;
            if VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) == NULL!() {
                assert!(false);
            }
            VOS_AVL3_INIT_NODE!(stNode3.stNode);
            stNode3.iKey = 10;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode3.stNode, stTreeInfo), true);
            ulVar = 10;
            if VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) == NULL!() {
                assert!(false);
            }
            if VOS_AVL3_PREV!(stNode3.stNode, stTreeInfo) != NULL!() {
                stNode5 = *(VOS_AVL3_PREV!(stNode3.stNode, stTreeInfo).cast::<Ptr<UserStruct>>());
                VOS_GetAVL3Node(c_ref!(stNode5.stNode), c_ref!(ulVar2));
                assert_eq!(9, ulVar2);
            } else {
                assert!(false);
            }
            test_no_memory_leak!();
        }
    }

    #[test]
    fn ITest_AVL3_PREV_003() {
        unsafe {
            let mut ulVar: VOS_UINT32 = 0;
            let mut stTree: AVL3_TREE = Default::default();
            let mut stNode1: UserStruct = Default::default();
            let mut stNode2: UserStruct = Default::default();
            let mut stNode3: UserStruct = Default::default();
            g_ulKeyoffset = (c_ref!(stNode1.iKey).cast::<Ptr<VOS_CHAR>>()
                - c_ref!(stNode1).cast::<Ptr<VOS_CHAR>>()) as u16;
            g_ulNodeoffset = (c_ref!(stNode1.stNode).cast::<Ptr<VOS_CHAR>>()
                - c_ref!(stNode1).cast::<Ptr<VOS_CHAR>>()) as u16;
            let mut stTreeInfo: AVL3_TREE_INFO = Default::default();
            stTreeInfo.usKeyOffset = g_ulKeyoffset;
            stTreeInfo.pfCompare = func!(AVL3_CompareFunc);
            stTreeInfo.usNodeOffset = g_ulNodeoffset;
            VOS_AVL3_INIT_TREE!(stTree, stTreeInfo);
            VOS_AVL3_INIT_NODE!(stNode1.stNode);
            stNode1.iKey = 9;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode1.stNode, stTreeInfo), true);
            ulVar = 9;
            if VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) == NULL!() {
                assert!(false);
            }
            VOS_AVL3_INIT_NODE!(stNode2.stNode);
            stNode2.iKey = 19;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode2.stNode, stTreeInfo), true);
            ulVar = 19;
            if VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) == NULL!() {
                assert!(false);
            }
            VOS_AVL3_INIT_NODE!(stNode3.stNode);
            stNode3.iKey = 10;
            assert_eq!(VOS_AVL3_INSERT!(stTree, stNode3.stNode, stTreeInfo), true);
            ulVar = 10;
            if VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) == NULL!() {
                assert!(false);
            }
            if VOS_AVL3_PREV!(stNode1.stNode, stTreeInfo) != NULL!() {
                VOS_AVL3_PREV!(stNode1.stNode, stTreeInfo);
                assert!(false);
            }
            test_no_memory_leak!();
        }
    }

    #[test]
    fn ITest_AVL3_INSERT_OR_FIND_001() {
        unsafe {
            let mut ulVar: VOS_UINT32 = 0;
            let mut stTree: AVL3_TREE = Default::default();
            let mut stNode1: UserStruct = Default::default();
            g_ulKeyoffset = (c_ref!(stNode1.iKey).cast::<Ptr<VOS_CHAR>>()
                - c_ref!(stNode1).cast::<Ptr<VOS_CHAR>>()) as u16;
            g_ulNodeoffset = (c_ref!(stNode1.stNode).cast::<Ptr<VOS_CHAR>>()
                - c_ref!(stNode1).cast::<Ptr<VOS_CHAR>>()) as u16;
            let mut stTreeInfo: AVL3_TREE_INFO = Default::default();
            stTreeInfo.usKeyOffset = g_ulKeyoffset;
            stTreeInfo.pfCompare = func!(AVL3_CompareFunc);
            stTreeInfo.usNodeOffset = g_ulNodeoffset;
            VOS_AVL3_INIT_TREE!(stTree, stTreeInfo);
            VOS_AVL3_INIT_NODE!(stNode1.stNode);
            stNode1.iKey = 2;
            VOS_AVL3_INSERT_OR_FIND!(stTree, stNode1.stNode, stTreeInfo);
            ulVar = 2;
            if VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) == NULL!() {
                assert!(false);
            }
            test_no_memory_leak!();
        }
    }

    #[test]
    fn ITest_AVL3_INSERT_OR_FIND_002() {
        unsafe {
            let mut ulVar: VOS_UINT32 = 0;
            let mut stTree: AVL3_TREE = Default::default();
            let mut stNode1: UserStruct = Default::default();
            let mut stNode2: UserStruct = Default::default();
            let mut stNode3: UserStruct = Default::default();
            let mut stNode4: UserStruct = Default::default();
            g_ulKeyoffset = (c_ref!(stNode1.iKey).cast::<Ptr<VOS_CHAR>>()
                - c_ref!(stNode1).cast::<Ptr<VOS_CHAR>>()) as u16;
            g_ulNodeoffset = (c_ref!(stNode1.stNode).cast::<Ptr<VOS_CHAR>>()
                - c_ref!(stNode1).cast::<Ptr<VOS_CHAR>>()) as u16;
            let mut stTreeInfo: AVL3_TREE_INFO = Default::default();
            stTreeInfo.usKeyOffset = g_ulKeyoffset;
            stTreeInfo.pfCompare = func!(AVL3_CompareFunc);
            stTreeInfo.usNodeOffset = g_ulNodeoffset;
            VOS_AVL3_INIT_TREE!(stTree, stTreeInfo);
            VOS_AVL3_INIT_NODE!(stNode1.stNode);
            stNode1.iKey = 2;
            VOS_AVL3_INSERT_OR_FIND!(stTree, stNode1.stNode, stTreeInfo);
            ulVar = 2;
            if VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) == NULL!() {
                assert!(false);
            }
            VOS_AVL3_INIT_NODE!(stNode2.stNode);
            stNode2.iKey = 12;
            VOS_AVL3_INSERT_OR_FIND!(stTree, stNode2.stNode, stTreeInfo);
            ulVar = 12;
            if VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) == NULL!() {
                assert!(false);
            }
            VOS_AVL3_INIT_NODE!(stNode3.stNode);
            stNode3.iKey = 23;
            VOS_AVL3_INSERT_OR_FIND!(stTree, stNode3.stNode, stTreeInfo);
            ulVar = 23;
            if VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) == NULL!() {
                assert!(false);
            }
            VOS_AVL3_INIT_NODE!(stNode4.stNode);
            stNode4.iKey = 21;
            VOS_AVL3_INSERT_OR_FIND!(stTree, stNode4.stNode, stTreeInfo);
            ulVar = 21;
            if VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) == NULL!() {
                assert!(false);
            }
            test_no_memory_leak!();
        }
    }

    #[test]
    fn ITest_AVL3_INSERT_OR_FIND_003() {
        unsafe {
            let mut ulVar: VOS_UINT32 = 0;
            let mut stTree: AVL3_TREE = Default::default();
            let mut stNode1: UserStruct = Default::default();
            let mut stNode2: UserStruct = Default::default();
            let mut stNode3: UserStruct = Default::default();
            let mut stNode4: UserStruct = Default::default();
            g_ulKeyoffset = (c_ref!(stNode1.iKey).cast::<Ptr<VOS_CHAR>>()
                - c_ref!(stNode1).cast::<Ptr<VOS_CHAR>>()) as u16;
            g_ulNodeoffset = (c_ref!(stNode1.stNode).cast::<Ptr<VOS_CHAR>>()
                - c_ref!(stNode1).cast::<Ptr<VOS_CHAR>>()) as u16;
            let mut stTreeInfo: AVL3_TREE_INFO = Default::default();
            stTreeInfo.usKeyOffset = g_ulKeyoffset;
            stTreeInfo.pfCompare = func!(AVL3_CompareFunc);
            stTreeInfo.usNodeOffset = g_ulNodeoffset;
            VOS_AVL3_INIT_TREE!(stTree, stTreeInfo);
            VOS_AVL3_INIT_NODE!(stNode1.stNode);
            stNode1.iKey = 2;
            VOS_AVL3_INSERT_OR_FIND!(stTree, stNode1.stNode, stTreeInfo);
            ulVar = 2;
            if VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) == NULL!() {
                assert!(false);
            }
            VOS_AVL3_INIT_NODE!(stNode2.stNode);
            stNode2.iKey = 12;
            VOS_AVL3_INSERT_OR_FIND!(stTree, stNode2.stNode, stTreeInfo);
            ulVar = 12;
            if VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) == NULL!() {
                assert!(false);
            }
            VOS_AVL3_INIT_NODE!(stNode3.stNode);
            stNode3.iKey = 12;
            VOS_AVL3_INSERT_OR_FIND!(stTree, stNode3.stNode, stTreeInfo);
            ulVar = 12;
            if VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) == NULL!() {
                assert!(false);
            }
            VOS_AVL3_INIT_NODE!(stNode4.stNode);
            stNode4.iKey = 21;
            VOS_AVL3_INSERT_OR_FIND!(stTree, stNode4.stNode, stTreeInfo);
            ulVar = 21;
            if VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) == NULL!() {
                assert!(false);
            }
            test_no_memory_leak!();
        }
    }

    #[test]
    fn ITest_AVL3_INSERT_OR_FIND_004() {
        unsafe {
            let mut ulVar: VOS_UINT32 = 0;
            let mut stTree: AVL3_TREE = Default::default();
            let mut stNode1: UserStruct = Default::default();
            let mut stNode2: UserStruct = Default::default();
            let mut stNode3: UserStruct = Default::default();
            let mut stNode4: UserStruct = Default::default();
            g_ulKeyoffset = (c_ref!(stNode1.iKey).cast::<Ptr<VOS_CHAR>>()
                - c_ref!(stNode1).cast::<Ptr<VOS_CHAR>>()) as u16;
            g_ulNodeoffset = (c_ref!(stNode1.stNode).cast::<Ptr<VOS_CHAR>>()
                - c_ref!(stNode1).cast::<Ptr<VOS_CHAR>>()) as u16;
            let mut stTreeInfo: AVL3_TREE_INFO = Default::default();
            stTreeInfo.usKeyOffset = g_ulKeyoffset;
            stTreeInfo.pfCompare = func!(AVL3_CompareFunc);
            stTreeInfo.usNodeOffset = g_ulNodeoffset;
            VOS_AVL3_INIT_TREE!(stTree, stTreeInfo);
            VOS_AVL3_INIT_NODE!(stNode1.stNode);
            stNode1.iKey = 2;
            VOS_AVL3_INSERT_OR_FIND!(stTree, stNode1.stNode, stTreeInfo);
            ulVar = 2;
            ulVar = 2;
            if VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) == NULL!() {
                assert!(false);
            }
            VOS_AVL3_INIT_NODE!(stNode2.stNode);
            stNode2.iKey = 10;
            VOS_AVL3_INSERT_OR_FIND!(stTree, stNode2.stNode, stTreeInfo);
            ulVar = 10;
            if VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) == NULL!() {
                assert!(false);
            }
            VOS_AVL3_INIT_NODE!(stNode3.stNode);
            stNode3.iKey = 12;
            VOS_AVL3_INSERT_OR_FIND!(stTree, stNode3.stNode, stTreeInfo);
            ulVar = 12;
            if VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) == NULL!() {
                assert!(false);
            }
            VOS_AVL3_INIT_NODE!(stNode4.stNode);
            stNode4.iKey = 21;
            VOS_AVL3_INSERT_OR_FIND!(stTree, stNode4.stNode, stTreeInfo);
            ulVar = 21;
            if VOS_AVL3_FIND!(stTree, c_ref!(ulVar).cast(), stTreeInfo) == NULL!() {
                assert!(false);
            }
            test_no_memory_leak!();
        }
    }
}
