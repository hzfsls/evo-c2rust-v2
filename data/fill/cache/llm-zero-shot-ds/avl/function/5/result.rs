use std::ptr;

pub unsafe fn vos_avl3_last<T>(tree: *const AVL3Tree<T>, tree_info: *const AVL3TreeInfo<T>) -> *mut T {
    if tree.is_null() || tree_info.is_null() {
        return ptr::null_mut();
    }

    let last_node = (*tree).pstLast;
    if last_node.is_null() {
        return ptr::null_mut();
    }

    // Assuming usNodeOffset is the offset of the node within the containing struct
    let node_offset = (*tree_info).usNodeOffset;
    (last_node as *mut u8).sub(node_offset as usize) as *mut T
}

// Assuming the following types are defined somewhere in your codebase:
// struct AVL3Tree<T> {
//     pstLast: *mut AVL3Node<T>,
//     // other fields...
// }
//
// struct AVL3TreeInfo<T> {
//     usNodeOffset: u16,
//     // other fields...
// }
//
// struct AVL3Node<T> {
//     // node fields...
// }
