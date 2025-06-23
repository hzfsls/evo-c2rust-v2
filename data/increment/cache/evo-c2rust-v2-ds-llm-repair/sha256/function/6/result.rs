pub fn vosSha256CompressBlock(mut state: Ptr<u32>, mut block: Ptr<u8>) {
    let mut W: Array<u32, 64> = Default::default();
    let mut i: u32 = Default::default();
    let mut j: u32 = Default::default();
    let mut a: u32 = Default::default();
    let mut b: u32 = Default::default();
    let mut c: u32 = Default::default();
    let mut d: u32 = Default::default();
    let mut e: u32 = Default::default();
    let mut f: u32 = Default::default();
    let mut g: u32 = Default::default();
    let mut h: u32 = Default::default();

    c_for!(i = 0; i < 16; i.suffix_plus_plus(); {
        W[i] = GET_UINT32_BE!(block, 4 * i);
    });

    c_for!(i = 16; i < 64; i.suffix_plus_plus(); {
        W[i] = W[i - 16] + W[i - 7] + (VOS_ROTR32!(W[i - 15], 7) ^ VOS_ROTR32!(W[i - 15], 18) ^ (W[i - 15] >> 3)) +
               (VOS_ROTR32!(W[i - 2], 17) ^ VOS_ROTR32!(W[i - 2], 19) ^ (W[i - 2] >> 10));
    });

    j = 0;
    a = state[j];
    j += 1;
    b = state[j];
    j += 1;
    c = state[j];
    j += 1;
    d = state[j];
    j += 1;
    e = state[j];
    j += 1;
    f = state[j];
    j += 1;
    g = state[j];
    j += 1;
    h = state[j];

    c_for!(i = 0; i < 64; i += 8; {
        j = 0;
        VOS_ROUND!(a, b, c, d, e, f, g, h, i + j, K256[i + 0], W);
        j += 1;
        VOS_ROUND!(h, a, b, c, d, e, f, g, i + j, K256[i + 1], W);
        j += 1;
        VOS_ROUND!(g, h, a, b, c, d, e, f, i + j, K256[i + 2], W);
        j += 1;
        VOS_ROUND!(f, g, h, a, b, c, d, e, i + j, K256[i + 3], W);
        j += 1;
        VOS_ROUND!(e, f, g, h, a, b, c, d, i + j, K256[i + 4], W);
        j += 1;
        VOS_ROUND!(d, e, f, g, h, a, b, c, i + j, K256[i + 5], W);
        j += 1;
        VOS_ROUND!(c, d, e, f, g, h, a, b, i + j, K256[i + 6], W);
        j += 1;
        VOS_ROUND!(b, c, d, e, f, g, h, a, i + j, K256[i + 7], W);
    });

    j = 0;
    state[j] += a;
    j += 1;
    state[j] += b;
    j += 1;
    state[j] += c;
    j += 1;
    state[j] += d;
    j += 1;
    state[j] += e;
    j += 1;
    state[j] += f;
    j += 1;
    state[j] += g;
    j += 1;
    state[j] += h;
}
