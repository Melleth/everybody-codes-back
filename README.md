# everybody-codes-back

This is the backend repository for my submission.
Both parts are written in rust. I wanted to try to write a small server
with [Rocket](https://rocket.rs/). Unfortunately I encountered some struggles
that requires the build to use the nightly toolchain. Rocket doesn't seem to be
fully stable yet. ;)

## Build instructions
1. install rust through [rustup](https://rustup.rs/)
2. install nightly toolchain via rustup
3. clone repo
4. `cargo build --bin search`
5. `cargo run --bin backend`
