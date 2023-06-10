#!/usr/bin/env bash

set -Eeuox pipefail

FEATURES_SETS=(
    ""
    "--no-default-features"
    '--no-default-features --features "alloc"'
    '--no-default-features --features "std"'
    '--no-default-features --features "alloc std"'
)

TOOLCHAINS=(
    stable
    beta
    nightly
    "1.60.0"
)

cargo +stable fmt --all -- --check

for TOOLCHAIN in "${TOOLCHAINS[@]}"; do
    for FEATURES_SET in "${FEATURES_SETS[@]}"; do
        cargo +$TOOLCHAIN clippy --all $FEATURES_SETS -- -D warnings
        cargo +$TOOLCHAIN clippy --all --tests $FEATURES_SETS -- -D warnings
        cargo +$TOOLCHAIN test --verbose --all
    done
    (
        cd ./tests/no-alloc
        cargo +$TOOLCHAIN clippy --all -- -D warnings
        cargo +$TOOLCHAIN build --verbose --all
    )
    (
        if [[ $TOOLCHAIN != "1.60.0" ]]; then
            cd ./tests/no-std
            cargo +$TOOLCHAIN clippy --all -- -D warnings
            cargo +$TOOLCHAIN build --verbose --all
        fi
    )
    (
        cd ./tests/re-export
        cargo +$TOOLCHAIN clippy --all -- -D warnings
        cargo +$TOOLCHAIN clippy --all --tests -- -D warnings
        cargo +$TOOLCHAIN test --verbose --all
    )
    (
        cd ./tests/use-re-exported
        cargo +$TOOLCHAIN clippy --all -- -D warnings
        cargo +$TOOLCHAIN clippy --all --tests -- -D warnings
        cargo +$TOOLCHAIN test --verbose --all
    )
    (
        cd ./examples/wasm_bindgen
        wasm-pack test --node | grep -q succeed
    )
done
