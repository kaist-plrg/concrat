# Concrat

**Con**current-**C** to **R**ust **A**utomatic **T**ranslator

```bash
git clone https://github.com/kaist-plrg/concrat
cd concrat
rustup component add rust-src rustc-dev llvm-tools-preview
cargo build --release
cargo build --manifest-path deps_crate/Cargo.toml
cargo test --release --lib
./test_dataflow.sh
./test_examples.sh
./test_bench.sh
```
