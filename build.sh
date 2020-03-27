rm -r web/pkg
wasm-pack build --target web --release
mv pkg web