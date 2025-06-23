macro_rules! CMPTLZ_WRITE32BIT { ($ptr:expr, $val:expr) => {
    {
        let mut unaligned_ptr = $ptr.cast::<Ptr<CmptlzUnalignU32>>();
        unaligned_ptr.v = $val;
    }
} }
pub(crate) use CMPTLZ_WRITE32BIT;
```

### Explanation:
- `CmptlzUnalignU32` is assumed to be a struct or type defined elsewhere.
- `ptr` is translated to a pointer using `.cast::<Ptr<CmptlzUnalignU32>>()` to match the C cast `(CmptlzUnalignU32 *)`.
- The assignment `->v = (val)` is directly translated to `.v = $val`.
- The macro is wrapped in a block `{ ... }` to avoid macro expansion issues and to ensure safe usage.