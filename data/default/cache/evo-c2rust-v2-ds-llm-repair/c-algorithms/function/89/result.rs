pub fn set_remove(mut set: Ptr<Set>, mut data: SetValue) -> i32 {
    let mut rover: Ptr<Ptr<SetEntry>>;
    let mut entry: Ptr<SetEntry>;
    let mut index: u32;

    index = (set.hash_func)(data) % set.table_size;

    rover = c_ref!(set.table[index]);

    while (*rover != NULL!()) {
        if ((set.equal_func)(data, (*rover).data) != 0) {
            entry = *rover;

            *rover = entry.next;

            set.entries -= 1;

            set_free_entry(set, entry);

            return 1;
        }

        rover = c_ref!((*rover).next);
    }

    return 0;
}
