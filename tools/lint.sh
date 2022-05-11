#!/bin/bash

set -xe

cargo clippy --all-targets --all-features -- \
  --deny warnings \
  --deny clippy::all \
  --deny clippy::cargo \
  --deny clippy::nursery \
  --deny clippy::pedantic
cargo fmt --check

