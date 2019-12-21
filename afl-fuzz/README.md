## Prepare

```
cargo install afl
```

## Run

```
cargo afl build
cargo afl fuzz -i in -o out target/debug/afl-fuzz
```
