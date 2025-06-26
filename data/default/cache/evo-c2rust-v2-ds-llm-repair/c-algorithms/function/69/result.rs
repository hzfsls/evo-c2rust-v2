pub fn hash_table_allocate_table(mut hash_table: Ptr<HashTable>) -> i32 {
    let mut new_table_size: u32;
    if (hash_table.prime_index < hash_table_num_primes) {
        let tmp0 = hash_table.prime_index;
        new_table_size = hash_table_primes[tmp0];
    } else {
        new_table_size = (hash_table.entries * 10);
    }
    hash_table.table_size = new_table_size;
    hash_table.table = c_calloc!(hash_table.table_size, c_sizeof!(Ptr<HashTableEntry>));
    return (hash_table.table != NULL!()).cast::<i32>();
}
