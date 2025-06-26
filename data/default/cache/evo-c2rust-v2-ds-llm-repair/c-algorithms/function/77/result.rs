pub fn hash_table_remove(mut hash_table: Ptr<HashTable>, mut key: HashTableKey) -> i32 {
    let mut rover: Ptr<Ptr<HashTableEntry>>;
    let mut entry: Ptr<HashTableEntry>;
    let mut pair: Ptr<HashTablePair>;
    let mut index: u32;
    let mut result: i32;

    index = (hash_table.hash_func)(key) % hash_table.table_size;

    result = 0;
    rover = c_ref!(hash_table.table[index]).cast();

    while (*rover != NULL!()).as_bool() {
        pair = c_ref!((*rover).pair).cast();

        if ((hash_table.equal_func)(key, pair.key) != 0).as_bool() {
            entry = *rover;

            *rover = entry.next.cast();

            hash_table_free_entry(hash_table.cast(), entry.cast());

            hash_table.entries.prefix_minus_minus();

            result = 1;

            break;
        }

        rover = c_ref!((*rover).next).cast();
    }

    return result.cast();
}
