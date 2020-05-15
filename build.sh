rm -r web/pkg
wasm-pack build --target web --release
mv pkg web
find web/pkg -type f ! -name '*.js' ! -name '*.wasm' -delete
find web/pkg -type f ! -name '*.wasm' -exec minify {} -o {} \;
find web -name "*.html" -o -name "*.js" -o -name "*.wasm" -o -name "*.css" | tar -cjf SWIC_3.tar.bz2 -T -