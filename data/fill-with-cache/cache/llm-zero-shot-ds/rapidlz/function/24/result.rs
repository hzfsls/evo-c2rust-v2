use std::alloc::{dealloc, Layout};
use std::ptr::null_mut;

// Assuming RapidlzCCtx is a struct with a hashTable field that is a raw pointer
struct RapidlzCCtx {
    hashTable: *mut u8,
    // other fields...
}

unsafe fn RapidlzCCtxFree(cCtx: *mut RapidlzCCtx) {
    if !cCtx.is_null() {
        let cCtx_ref = &mut *cCtx;
        if !cCtx_ref.hashTable.is_null() {
            // Deallocate the hashTable
            // Note: We need to know the layout (size and alignment) of the hashTable allocation
            // This is just a placeholder - you'd need the actual layout used when allocating
            dealloc(cCtx_ref.hashTable, Layout::new::<u8>());
            cCtx_ref.hashTable = null_mut();
        }
        // Deallocate the RapidlzCCtx itself
        // Again, we need the actual layout used when allocating
        dealloc(cCtx as *mut u8, Layout::new::<RapidlzCCtx>());
    }
}
