pub fn BzpFileEOF(mut f: FilePtr) -> bool {
    let mut c: i32 = c_fgetc!(f);
    if c == BZP_EOF!() {
        return true;
    }
    c_ungetc!(c, f).cast::<Void>();
    return false;
}
