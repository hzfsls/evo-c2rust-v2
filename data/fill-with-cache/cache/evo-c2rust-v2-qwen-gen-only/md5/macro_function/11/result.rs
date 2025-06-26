macro_rules! MD5_FUNC_I {
    ($value:expr, $md5State:expr, $text:expr, $addEnd:expr, $moveBit:expr) => {
        {
            let temp_value = MD5_LINEAR_FUNC_I!($md5State[1], $md5State[2], $md5State[3]) + $md5State[0] + $text + $addEnd;
            MD5_CYCLE_MOVE!(temp_value, $moveBit);
            MD5_CHANGE_STATE_IN_TURN!($md5State, temp_value);
            $value = temp_value;
        }
    }
}
pub(crate) use MD5_FUNC_I;
```

### Explanation:
- The macro `MD5_FUNC_I` is translated using `macro_rules!` with appropriate parameters.
- The body of the macro is wrapped in a block `{}` to mimic the `do { ... } while (0)` pattern in C.
- The `MD5_LINEAR_FUNC_I`, `MD5_CYCLE_MOVE`, and `MD5_CHANGE_STATE_IN_TURN` macros are used as-is with the `!()` syntax.
- The result of the computation is assigned to `$value` at the end to match the C behavior.