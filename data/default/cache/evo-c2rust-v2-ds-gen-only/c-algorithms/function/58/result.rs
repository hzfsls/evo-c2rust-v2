pub fn bloom_filter_query(mut bloomfilter: Ptr<BloomFilter>, mut value: BloomFilterValue) -> i32 {
    let mut hash: u32;
    let mut subhash: u32;
    let mut index: u32;
    let mut i: u32;
    let mut b: u8;
    let mut bit: i32;

    hash = bloomfilter.hash_func(value).cast();

    c_for!(i = 0; i < bloomfilter.num_functions; i.prefix_plus_plus(); {
        subhash = (hash ^ salts[i]).cast();

        index = (subhash % bloomfilter.table_size).cast();

        b = bloomfilter.table[index / 8].cast();
        bit = (1 << (index % 8)).cast();

        if ((b & bit) == 0).as_bool() {
            return 0;
        }
    });

    return 1;
}
