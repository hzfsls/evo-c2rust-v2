pub fn list_length(mut list: Ptr<ListEntry>) -> u32 {
    let mut entry: Ptr<ListEntry>;
    let mut length: u32;

    length = 0;
    entry = list.cast();

    while (entry != NULL!()).as_bool() {
        length.prefix_plus_plus();
        entry = entry.next.cast();
    }

    return length.cast();
}
