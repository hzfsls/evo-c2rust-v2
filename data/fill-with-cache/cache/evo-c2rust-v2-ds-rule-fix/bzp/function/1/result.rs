pub fn BzpStreamFinish(mut stream: Ptr<BzpStream>) {
    if (stream != NULL!()).as_bool() {
        c_free!(stream);
        stream = NULL!();
    }
}
