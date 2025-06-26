pub fn bloom_filter_union(mut filter1: Ptr<BloomFilter>, mut filter2: Ptr<BloomFilter>) -> Ptr<BloomFilter> {
    let mut result: Ptr<BloomFilter>;
    let mut i: u32;
    let mut array_size: u32;

    if (filter1.table_size != filter2.table_size).as_bool() || (filter1.num_functions != filter2.num_functions).as_bool() ||
        (filter1.hash_func != filter2.hash_func).as_bool()
    {
        return NULL!();
    }

    result = bloom_filter_new(filter1.table_size.cast(), filter1.hash_func.cast(), filter1.num_functions.cast());

    if (result == NULL!()).as_bool() {
        return NULL!();
    }

    array_size = (filter1.table_size + 7) / 8;

    c_for!(i = 0; i < array_size; i.prefix_plus_plus(); {
        result.table[i] = (filter1.table[i] | filter2.table[i]).cast();
    });

    return result.cast();
}
