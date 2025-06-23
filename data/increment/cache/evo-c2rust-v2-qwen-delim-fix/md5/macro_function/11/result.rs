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
- The macro `MD5_FUNC_I` is translated using `macro_rules!` in Rust.
- The body of the macro is wrapped in a block `{}` to mimic the `do { ... } while (0)` pattern in C.
- The `temp_value` is used to capture the result of the expression to assign to `$value` at the end.
- All macro calls like `MD5_LINEAR_FUNC_I`, `MD5_CYCLE_MOVE`, and `MD5_CHANGE_STATE_IN_TURN` are translated as macro invocations with `!()` in Rust.
- The assignment to `$value` is done at the end to match the C behavior.