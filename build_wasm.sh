#!/usr/bin/env bash
set -e

echo "Building WebAssembly package..."
nix develop --command wasm-pack build --target web --out-name timex_datalink_wasm --out-dir pkg --no-default-features --features "wasm"

echo "Making the web application executable..."
# Make sure we're copying the latest version of the HTML file
if [ -f "index.html" ]; then
    cp index.html pkg/
else
    echo "Warning: index.html not found in root directory"
fi
cp wasm_timex.js pkg/

echo "WebAssembly build complete. The output is in the pkg/ directory."
echo "You can now serve the web application by running:"
echo "cd pkg && python3 -m http.server"
echo "Then open your browser to http://localhost:8000"