Problem
```shell
dyld[12295]: Library not loaded: @rpath/libpython3.9.dylib
  Referenced from: <5D1CCF3C-0EFB-3921-8D1B-6A2F7F48B3A0> /Users/lux/src/github.com/xunfeng1980/rust-study/target/debug/pyo3-demo
  Reason: tried: '/Users/lux/.wasmedge/lib/libpython3.9.dylib' (no such file), '/System/Volumes/Preboot/Cryptexes/OS@rpath/libpython3.9.dylib' (no such file), '/Users/lux/src/github.com/xunfeng1980/rust-study/target/debug/deps/libpython3.9.dylib' (no such file), '/Users/lux/src/github.com/xunfeng1980/rust-study/target/debug/libpython3.9.dylib' (no such file), '/Users/lux/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/aarch64-apple-darwin/lib/libpython3.9.dylib' (no such file), '/Users/lux/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/libpython3.9.dylib' (no such file), '/Users/lux/lib/libpython3.9.dylib' (no such file), '/usr/local/lib/libpython3.9.dylib' (no such file), '/usr/lib/libpython3.9.dylib' (no such file, not in dyld cache)
```
Solved
```shell
DYLD_FALLBACK_LIBRARY_PATH=/Users/lux/opt/miniconda3/lib
```