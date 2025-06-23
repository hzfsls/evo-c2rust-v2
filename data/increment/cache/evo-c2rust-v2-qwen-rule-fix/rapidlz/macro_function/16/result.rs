macro_rules! RAPIDLZ_WRITE64BIT { ($ptr:expr, $val:expr) => {
    {
        let mut unaligned_ptr = $ptr.cast::<Ptr<RapidlzUnalignU64>>();
        unaligned_ptr.v = $val;
    }
} }
pub(crate) use RAPIDLZ_WRITE64BIT;
```

### Explanation:
- `ptr` is a pointer in C, so in Rust it's translated to `Ptr<T>`.
- `(RapidlzUnalignU64 *)ptr` is a cast to a pointer of type `RapidlzUnalignU64`, so we use `.cast::<Ptr<RapidlzUnalignU64>>()` in Rust.
- The assignment `->v = (val)` is translated directly to `.v = $val`.
- The macro is wrapped in a block `{ ... }` to avoid macro hygiene issues and to ensure it behaves like the original C macro.