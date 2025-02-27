# wheel-reimplement
Learn CS foundations by reimplementing wheels
## Repo overview
- `simple-dns` A synchronous dual-stack DNS client.
- `rsos` A simple x86_64 operating system built with rust `no_std`

## Additional Notes
### rsos
`rsos` requires Rust's nightly features. It can be built with `rustc 1.86.0-nightly (a567209da 2025-02-13)`, any nightly version different from this may fail to compile.  
The cargo config is well set, when executing `cargo run` or `cargo test`, it will run the system with `qemu-system-x86_64` automatically. So make sure you installed `qemu` correctly, if you want to run it yourself.
