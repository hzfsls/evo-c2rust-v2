pub unsafe extern "C" fn CmptLzDecMemFree(
    memHook: *mut CmptLzMemHook,
    memHandle: i32,
    freeAddress: *mut c_void,
) {
    ((*memHook).CmptLzFree)(memHandle, freeAddress);
}
