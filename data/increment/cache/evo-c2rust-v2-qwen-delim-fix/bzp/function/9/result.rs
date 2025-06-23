pub fn BzpFileEOF(mut f: FilePtr) -> bool {
    let mut c: i32 = c_fgetc!(f).cast();
    if (c == BZP_EOF!()).as_bool() {
        return true;
    }
    c_ungetc!(c.cast(), f).cast::<Void>();
    return false;
}