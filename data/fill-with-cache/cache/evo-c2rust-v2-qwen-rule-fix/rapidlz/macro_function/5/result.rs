macro_rules! RAPIDLZ_READ64BIT { ($ptr:expr) => { 
    ((const RapidlzUnalignU64Ptr::new($ptr)).v) 
} }
pub(crate) use RAPIDLZ_READ64BIT;
```

### Explanation:
- `const RapidlzUnalignU64 *` is translated to `const RapidlzUnalignU64Ptr::new(...)`, assuming `RapidlzUnalignU64Ptr` is a wrapper type for a pointer to `RapidlzUnalignU64`.
- The dereference `->v` is translated to `.v` in Rust.
- The macro is wrapped in `macro_rules!` and marked `pub(crate)` for visibility.
- The macro is used as `RAPIDLZ_READ64BIT!(ptr)` in Rust code.