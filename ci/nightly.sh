#!/bin/bash

set -o errexit -o nounset

cargo build
cargo test --features std
