# build the package
build:
    wasm-pack build --target=deno
    ./scripts/add-ts-header.ts pkg/streaming_lz4.js

# clean package and build outputs
clean:
    cargo clean
    rm -rf pkg

check-publish: build
    deno publish --dry-run
