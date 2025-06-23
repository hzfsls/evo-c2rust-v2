pub fn RapidlzCopyMatchFast(mut dst: Ptr<u8>, mut r#match: Ptr<u8>, mut offset: u16, mut length: u32) {
    let mut dstCurr: Ptr<u8> = dst.cast();
    let mut matchPtr: Ptr<u8> = r#match.cast();

    if (offset >= RAPIDLZ_SIXTEEN_BYTE!()).as_bool() {
        RapidlzCopyLiteralsFast(matchPtr.cast(), dstCurr.cast(), length.cast());
        return;
    }

    c_for!(let mut i: i32 = 0; i < RAPIDLZ_EIGHT_BYTE!().cast(); i.prefix_plus_plus(); {
        dstCurr[i] = matchPtr[i].cast();
    });

    if (length <= RAPIDLZ_EIGHT_BYTE!()).as_bool() {
        return;
    }

    let mut dstEnd: Ptr<u8> = (dstCurr + length).cast();
    if (offset < RAPIDLZ_EIGHT_BYTE!()).as_bool() {
        matchPtr += (*g_overlapOffAddVal.lock())[offset];
        dstCurr += RAPIDLZ_EIGHT_BYTE!();
    }

    c_do!({
        RapidlzCopy8Byte(dstCurr.cast(), matchPtr.cast());
        dstCurr += RAPIDLZ_EIGHT_BYTE!();
        matchPtr += RAPIDLZ_EIGHT_BYTE!();
    } while dstCurr < dstEnd);
}
