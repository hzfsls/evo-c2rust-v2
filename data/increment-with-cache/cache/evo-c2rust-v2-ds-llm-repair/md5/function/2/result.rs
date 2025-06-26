pub fn VOS_MD5Init(mut context: Ptr<MD5_CTX>) {
    if (context == NULL!()).as_bool() {
        return;
    }

    c_memset_s!(context, c_sizeof!(MD5_CTX), 0, c_sizeof!(MD5_CTX)).cast::<Void>();

    context.aulState[0] = 0x67452301;
    context.aulState[1] = 0xefcdab89;
    context.aulState[2] = 0x98badcfe;
    context.aulState[3] = 0x10325476;
}
