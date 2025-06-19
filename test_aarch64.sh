#!/bin/bash
rm -rf target/aarch64-unknown-none-softfloat/debug/build/pie-boot-*
# cargo test --target aarch64-unknown-none -p test-some-rt  --test test -- --show-output
cargo test --target aarch64-unknown-none-softfloat -p test-some-rt --test test -- --show-output
