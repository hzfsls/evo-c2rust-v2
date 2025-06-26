pub fn bloom_filter_new(mut table_size: u32, mut hash_func: BloomFilterHashFunc, mut num_functions: u32) -> Ptr<BloomFilter> {
    let mut filter: Ptr<BloomFilter>;

    if (num_functions > (c_sizeofval!(salts) / c_sizeofval!(salts[0])).cast::<u32>()) {
        return NULL!();
    }

    filter = c_malloc!(c_sizeof!(BloomFilter));

    if (filter == NULL!()) {
        return NULL!();
    }

    filter.table = c_calloc!((table_size + 7) / 8, 1);

    if (filter.table == NULL!()) {
        c_free!(filter);
        return NULL!();
    }

    filter.hash_func = hash_func;
    filter.num_functions = num_functions;
    filter.table_size = table_size;

    return filter;
}
