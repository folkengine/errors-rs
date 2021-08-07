# errors-rs

This is the code for [Benjamin Kay's](https://github.com/benkay86)
[Rust Error
Tutorial](https://benkay86.github.io/rust-error-tutorial.html), with a
little cleanup.

Under `code` is functioning code for each of the sections of the
tutorial.


* [Section 1](https://benkay86.github.io/rust-error-tutorial.html#result)
  Result?
* [Section 2](https://benkay86.github.io/rust-error-tutorial.html#dont_panic)
  Don't Panic!
  * `02a-panic_desugared` has the desugared version of the code.

# run script

This repository includes a shell script that makes it easy to run a
specific version of the error code.

```bash
❯ ./run 2
❯ cd /home/gaoler/src/github.com/folkengine/errors-rs/code/02-panic

    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/errors`
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: 
Os { code: 13, kind: PermissionDenied, message: "Permission denied" }', 
src/main.rs:4:48 
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

