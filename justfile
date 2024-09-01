# build the package
build:
    wasm-pack build --target=deno

# clean package and build outputs
clean:
    cargo clean
    rm -rf pkg
