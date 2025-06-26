pub fn set_iter_has_more(mut iterator: Ptr<SetIterator>) -> i32 {
    return (iterator.next_entry != NULL!()).cast();
}
