macro_rules! MD5_FUNC_G {
    ($value:expr, $md5State:expr, $text:expr, $addEnd:expr, $moveBit:expr) => {
        {
            let temp_value = MD5_LINEAR_FUNC_G!($md5State[1], $md5State[2], $md5State[3]) + $md5State[0] + $text + $addEnd;
            MD5_CYCLE_MOVE!(temp_value, $moveBit);
            MD5_CHANGE_STATE_IN_TURN!($md5State, temp_value);
            $value = temp_value;
        }
    }
}
pub(crate) use MD5_FUNC_G;
```

### Explanation:
- The macro `MD5_FUNC_G` is a compound macro that assigns a computed value to `value`, performs a cycle move, and updates the `md5State`.
- In Rust, we use a block `{}` to simulate the `do { ... } while (0)` pattern.
- The intermediate result is stored in `temp_value` to avoid potential issues with side effects.
- All nested macros like `MD5_LINEAR_FUNC_G`, `MD5_CYCLE_MOVE`, and `MD5_CHANGE_STATE_IN_TURN` are assumed to be already translated and available in the Rust code.
- The assignment to `$value` is done at the end of the block to match the C behavior.