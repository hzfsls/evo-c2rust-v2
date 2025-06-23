macro_rules! RAPIDLZ_READ32BIT { ($ptr:expr) => { 
    ((const RapidlzUnalignU32::cast($ptr)).v) 
} }
pub(crate) use RAPIDLZ_READ32BIT;
```

### Explanation:
- `const RapidlzUnalignU32::cast($ptr)` is used to cast the pointer to a `const` pointer of type `RapidlzUnalignU32`, similar to the C cast `(const RapidlzUnalignU32 *)ptr`.
- `.v` accesses the field `v` of the struct, just like in C.
- The macro is defined with `macro_rules!` and wrapped with `pub(crate) use` to make it visible within the crate.