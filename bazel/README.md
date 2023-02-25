# nix build
```shell
nix develop github:cargo2nix/cargo2nix#bootstrap --extra-experimental-features "nix-command flakes"
# need source code compile,so slowly
cargo2nix
# Or skip the shell and run it directly
# nix run github:cargo2nix/cargo2nix --extra-experimental-features "nix-command flakes"
cd /crates/basic-demo
nix build --extra-experimental-features "nix-command flakes"
./result-bin/bin/basic-demo
```

```shell
bazel run //:bazel
```