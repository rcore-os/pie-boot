#!/bin/bash
cargo test --target aarch64-unknown-none -p test-some-rt  --test test -- --show-output
# cargo test --target aarch64-unknown-none-softfloat -p test-some-rt --features pie-boot/dev-local-code --test test -- --show-output
