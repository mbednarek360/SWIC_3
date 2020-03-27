rm -r web/pkg
wasm-pack build --target web --release
mv pkg web
find src/pkg -type f ! -name '*.js' ! -name '*.wasm' -delete
find src/pkg -type f ! -name '*.wasm' -exec minify {} -o {} \;
find src -name "*.html" -o -name "*.js" -o -name "*.wasm" -o -name "*.css" | tar -cjf SLIC_3.tar.bz2 -T -