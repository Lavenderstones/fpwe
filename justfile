cargo +args:
    cd rust && cargo {{args}}

web profile='debug':
    @#!/usr/bin/env bash
    if [ "{{profile}}" = "debug" ]; then PROFILE="dev"; else PROFILE="{{profile}}"; fi
    cd rust
    # multithreaded
    RUSTFLAGS="-C link-args=-pthread -C target-feature=+atomics -C link-args=-sSIDE_MODULE=2 -Zlink-native-libraries=no -Cllvm-args=-enable-emscripten-cxx-exceptions=0" cargo +nightly build -Zbuild-std --target wasm32-unknown-emscripten --profile $PROFILE
    mv target/wasm32-unknown-emscripten/{{profile}}/fpwe.wasm target/wasm32-unknown-emscripten/{{profile}}/fpwe.threads.wasm
    # singlethreaded
    PROFILE="{{profile}}" cargo +nightly build --features nothreads -Zbuild-std --target wasm32-unknown-emscripten --profile $PROFILE
