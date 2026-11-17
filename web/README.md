# Browser integration harness

Functional TypeScript/HTML/CSS controls over the Rust `bts-web` adapter.
No simulation, pricing or accounting logic is implemented in JavaScript.

See [browser foundation](../docs/browser-foundation.md) for prerequisites,
protocol, player-reporting limits and validation evidence.

```sh
# From the repository root, once:
rustup target add wasm32-unknown-unknown
cargo install wasm-bindgen-cli --version 0.2.130 --locked
cd web
npm ci
npm run wasm
npm run dev

# Static build and real Chromium tests:
npm run build
npx playwright install --with-deps chromium
npm run test:e2e
npm run preview
```

Serve the generated `dist/` directory over HTTP. No deployment is performed.
Rebuild Wasm after Rust changes. `WASM_BINDGEN` can select a non-PATH CLI.
Playwright starts/stops the preview server itself on 127.0.0.1:4173; stop any
manual preview before running tests. `PLAYWRIGHT_BROWSERS_PATH` may select a
custom browser cache directory.
