# ALT Packages Difference

### How to build .so

```
cd apd_core
cargo build --release
```

### How to run CLI
```
cd ..
cd apd
cp ../apd_core/target/release/libapd_core.so target/debug/

cargo run -- --repo-1 p9 --repo-2 p10
```
