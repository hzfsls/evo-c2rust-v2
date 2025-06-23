use std::ptr;

pub unsafe fn vos_avl3_last<T>(tree: *const AVL3Tree, tree_info: *const AVL3TreeInfo) -> *mut T {
    if tree.is_null() || tree_info.is_null() {
        return ptr::null_mut();
    }

    let node = (*tree).pstLast;
    
    if node.is_null() {
        return ptr::null_mut();
    }

    let node_start = (node as *mut u8).offset((*tree_info).usNodeOffset as isize) as *mut T;
    node_start
}

// Assuming the following types are defined elsewhere in your Rust code:
// #[repr(C)]
// pub struct AVL3Tree {
//     pstLast: *mut AVL3Node,
//     // other fields...
// }
//
// #[repr(C)]
// pub struct AVL3TreeInfo {
//     usNodeOffset: u16,
//     // other fields...
// }
//
// #[repr(C)]
// pub struct AVL3Node {
//     // node fields...
// }
