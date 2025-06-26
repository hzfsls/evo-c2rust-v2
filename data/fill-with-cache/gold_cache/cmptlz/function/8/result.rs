pub fn CmptLzDecMemFree(mut memHook: Ptr<CmptLzMemHook>, mut memHandle: i32, mut freeAddress: VoidPtr){
    (memHook.CmptLzFree)(memHandle, freeAddress);
}