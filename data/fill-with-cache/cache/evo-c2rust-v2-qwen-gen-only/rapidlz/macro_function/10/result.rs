macro_rules! SAFE_COPY_MATCH { ($dstCurr:expr, $matchSrc:expr, $matchLength:expr) => 
    {
        while (c_prefix_minus_minus!($matchLength) > 0)
        {
            *$dstCurr = *$matchSrc;
            $dstCurr.plus_plus();
            $matchSrc.plus_plus();
        }
    }
}
pub(crate) use SAFE_COPY_MATCH;
```

### Explanation:
- `matchLength-- > 0` is translated to `c_prefix_minus_minus!($matchLength) > 0` to mimic the prefix decrement behavior.
- `*(dstCurr)++ = *(matchSrc)++` is translated to:
  - `*$dstCurr = *$matchSrc` for the assignment.
  - `$dstCurr.plus_plus()` and `$matchSrc.plus_plus()` for the increment.
- The macro is wrapped in a `do { ... } while (0)` block in C, but in Rust macros, this is not needed as the block is already a block expression.