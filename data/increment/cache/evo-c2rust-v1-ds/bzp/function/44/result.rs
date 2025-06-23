pub fn BzpStreamInit() -> Ptr<BzpStream> {
    let mut stream: Ptr<BzpStream> = c_malloc!(c_sizeof!(BzpStream));
    if stream == NULL!() {
        return NULL!();
    }
    stream.filePtr = NULL!();
    stream.pos = 0;
    stream.nBuf = 0;
    return stream.cast();
}
