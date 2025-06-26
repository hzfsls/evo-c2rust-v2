pub fn slist_length(mut list: Ptr<SListEntry>) -> u32 {
    let mut entry: Ptr<SListEntry>;
    let mut length: u32;

    length = 0;
    entry = list.cast();

    while (entry != NULL!()).as_bool() {
        length.prefix_plus_plus();
        entry = entry.next.cast();
    }

    return length.cast();
}
