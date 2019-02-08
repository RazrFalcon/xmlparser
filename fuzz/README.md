## Run

```
cd ..
RUSTFLAGS="-C relocation-model=dynamic-no-pic" cargo +nightly fuzz run fuzz_xml
```
