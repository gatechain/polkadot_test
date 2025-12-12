# polkadot

```sh
# build wasm with benchmarks
cargo build --features runtime-benchmarks --release

# run benchmark
frame-omni-bencher v1 benchmark pallet --runtime ./target/release/wbuild/polkadot_test/polkadot_test.wasm --pallet polkadot_test --extrinsic test_block

# expand macro
RUSTFLAGS="--cfg substrate_runtime" cargo expand --target wasm32v1-none --no-default-features --features runtime-benchmarks
```