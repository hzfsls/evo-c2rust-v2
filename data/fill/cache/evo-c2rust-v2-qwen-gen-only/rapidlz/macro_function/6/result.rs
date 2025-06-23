macro_rules! RAPIDLZ_WRITE64BIT { ($ptr:expr, $val:expr) => {
    {
        let mut unaligned_ptr = $ptr.cast::<Ptr<RapidlzUnalignU64>>();
        unaligned_ptr.v = $val;
    }
} }
pub(crate) use RAPIDLZ_WRITE64BIT;
```

### Explanation:
- `RAPIDLZ_WRITE64BIT` is a macro that dereferences a pointer as a `RapidlzUnalignU64` type and assigns a value to its `v` field.
- In Rust, we use `.cast::<Ptr<T>>()` to cast the pointer to the appropriate type.
- We wrap the logic in a block to avoid macro expansion issues and ensure proper scoping.
- The macro is marked `pub(crate)` and re-exported with `pub(crate) use` to ensure visibility within the crate.