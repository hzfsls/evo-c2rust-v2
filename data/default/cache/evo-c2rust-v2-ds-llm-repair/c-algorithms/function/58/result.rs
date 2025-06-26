pub fn bloom_filter_query(mut bloomfilter: Ptr<BloomFilter>, mut value: BloomFilterValue) -> i32 {
    let mut hash: u32;
    let mut subhash: u32;
    let mut index: u32;
    let mut i: u32;
    let mut b: u8;
    let mut bit: u8;

    hash = (bloomfilter.hash_func)(value);

    c_for!(i = 0; i < bloomfilter.num_functions; i.prefix_plus_plus(); {
        subhash = (hash ^ salts[i]);

        index = (subhash % bloomfilter.table_size);

        b = bloomfilter.table[index / 8];
        bit = (1 << (index % 8)) as u8;

        if ((b & bit) == 0) {
            return 0;
        }
    });

    return 1;
}
