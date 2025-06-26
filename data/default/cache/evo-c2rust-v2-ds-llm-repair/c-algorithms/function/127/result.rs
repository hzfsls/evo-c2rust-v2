pub fn trie_remove(mut trie: Ptr<Trie>, mut key: Ptr<u8>) -> i32 {
    let mut node: Ptr<TrieNode> = Default::default();
    let mut next: Ptr<TrieNode> = Default::default();
    let mut last_next_ptr: Ptr<Ptr<TrieNode>> = Default::default();
    let mut p: Ptr<u8> = Default::default();
    let mut c: i32 = Default::default();

    node = trie_find_end(trie, key);

    if (node != NULL!()) && (node.data != TRIE_NULL!()) {
        node.data = TRIE_NULL!();
    } else {
        return 0;
    }

    node = trie.root_node;
    last_next_ptr = c_ref!(trie.root_node);
    p = key;

    loop {
        c = (*p).cast::<u8>().cast::<i32>();
        next = node.next[c];

        node.use_count -= 1;

        if (node.use_count <= 0) {
            c_free!(node);

            if (last_next_ptr != NULL!()) {
                *last_next_ptr = NULL!();
                last_next_ptr = NULL!();
            }
        }

        if (c == '\0' as i32) {
            break;
        } else {
            p += 1;
        }

        if (last_next_ptr != NULL!()) {
            last_next_ptr = c_ref!(node.next[c]);
        }

        node = next;
    }

    return 1;
}
