pub fn BzpStreamFinish(mut stream: Ptr<BzpStream>) {
    if stream != NULL!() {
        c_free!(stream);
        stream = NULL!();
    }
}
