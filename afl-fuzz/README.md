## Dependencies

```
cargo install afl
```

## Run

```
env RUSTFLAGS="-Clink-arg=-fuse-ld=gold" cargo afl build
cargo afl fuzz -i in -o out target/debug/afl-fuzz
```
