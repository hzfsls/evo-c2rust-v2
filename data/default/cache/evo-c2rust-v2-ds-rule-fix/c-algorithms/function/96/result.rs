pub fn set_iter_next(mut iterator: Ptr<SetIterator>) -> SetValue {
    let mut set: Ptr<Set>;
    let mut result: SetValue = Default::default();
    let mut current_entry: Ptr<SetEntry>;
    let mut chain: u32 = Default::default();

    set = iterator.set.cast();

    if (iterator.next_entry == NULL!()).as_bool() {
        return SET_NULL!();
    }

    current_entry = iterator.next_entry.cast();
    result = current_entry.data.cast();

    if (current_entry.next != NULL!()).as_bool() {
        iterator.next_entry = current_entry.next.cast();
    } else {
        iterator.next_entry = NULL!();

        chain = (iterator.next_chain + 1).cast();

        while (chain < set.table_size).as_bool() {
            if (set.table[chain] != NULL!()).as_bool() {
                iterator.next_entry = set.table[chain].cast();

                break;
            }

            chain.prefix_plus_plus();
        }

        iterator.next_chain = chain.cast();
    }

    return result.cast();
}
