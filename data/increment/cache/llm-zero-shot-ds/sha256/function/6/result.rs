fn vos_sha256_compress_block(state: &mut [u32; VOS_SHA256_CTX_HASH_LEN], block: &[u8; SHA256_BLOCK_SIZE]) {
    let mut w = [0u32; 64];
    
    // Process first 16 words
    for i in 0..16 {
        w[i] = u32::from_be_bytes([
            block[4 * i],
            block[4 * i + 1],
            block[4 * i + 2],
            block[4 * i + 3],
        ]);
    }
    
    // Process remaining words
    for i in 16..64 {
        let s0 = w[i-15].rotate_right(7) ^ w[i-15].rotate_right(18) ^ (w[i-15] >> 3);
        let s1 = w[i-2].rotate_right(17) ^ w[i-2].rotate_right(19) ^ (w[i-2] >> 10);
        w[i] = w[i-16].wrapping_add(s0).wrapping_add(w[i-7]).wrapping_add(s1);
    }
    
    // Initialize working variables
    let mut a = state[0];
    let mut b = state[1];
    let mut c = state[2];
    let mut d = state[3];
    let mut e = state[4];
    let mut f = state[5];
    let mut g = state[6];
    let mut h = state[7];
    
    // Compression function main loop
    for i in (0..64).step_by(8) {
        vos_round(&mut a, &mut b, &mut c, &mut d, &mut e, &mut f, &mut g, &mut h, i+0, K256[i+0], &w);
        vos_round(&mut h, &mut a, &mut b, &mut c, &mut d, &mut e, &mut f, &mut g, i+1, K256[i+1], &w);
        vos_round(&mut g, &mut h, &mut a, &mut b, &mut c, &mut d, &mut e, &mut f, i+2, K256[i+2], &w);
        vos_round(&mut f, &mut g, &mut h, &mut a, &mut b, &mut c, &mut d, &mut e, i+3, K256[i+3], &w);
        vos_round(&mut e, &mut f, &mut g, &mut h, &mut a, &mut b, &mut c, &mut d, i+4, K256[i+4], &w);
        vos_round(&mut d, &mut e, &mut f, &mut g, &mut h, &mut a, &mut b, &mut c, i+5, K256[i+5], &w);
        vos_round(&mut c, &mut d, &mut e, &mut f, &mut g, &mut h, &mut a, &mut b, i+6, K256[i+6], &w);
        vos_round(&mut b, &mut c, &mut d, &mut e, &mut f, &mut g, &mut h, &mut a, i+7, K256[i+7], &w);
    }
    
    // Update state
    state[0] = state[0].wrapping_add(a);
    state[1] = state[1].wrapping_add(b);
    state[2] = state[2].wrapping_add(c);
    state[3] = state[3].wrapping_add(d);
    state[4] = state[4].wrapping_add(e);
    state[5] = state[5].wrapping_add(f);
    state[6] = state[6].wrapping_add(g);
    state[7] = state[7].wrapping_add(h);
}
