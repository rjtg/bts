# Browser foundation

Status: implemented. The browser is the intended primary player interface;
the CLI remains a developer tool. The first fourteen-slot integration harness
has been replaced by the [programme-planning workspace](programme-planning.md).
That specification documents the current seven-day scenario, explicit runtimes,
15-minute ticks, processing cadence, protocol v2, observation boundary and tests.
The original protocol v1 is no longer supported.

## Architecture

`bts-fuzzy` remains independent, dependency-free and extractable. `bts-sim`
owns the typed-clock engine, planner, fuzzy models, exact integer accounts,
quotes, audience-specific commitments, deadlines, off-air handling and causal
evidence. The old whole-hour CLI experiments still run on the same engine.
No filesystem, terminal, browser, wall-clock or external-service dependencies
enter the simulation.

`bts-web::Session` owns one Rust planner/simulation per game session. It exposes
`new`, `execute(Command)`, `dispatch_json(&str)` and `report()`. The wasm-bindgen
ABI provides `new WebSession()`, `session.dispatch(json)`, `session.free()` and
`api_version()` (2). Clone/apply/observe/commit keeps rejected commands atomic.
TypeScript owns controls and formatting, not simulation or accounting.

Player observations are constructed from a whitelist, separately from developer
reports. They exclude hidden execution, tastes, hype, repetition and fuzzy traces.
Quoted target delivery and past start-snapshot counts remain explicitly provisional
exact expectations, not measured ratings. Research and measurement-based
settlement remain future work. No developer inspector is exported. All world
data runs locally; the observation boundary is not a security guarantee.

Financial amounts, contract IDs and integer delivery units are decimal strings.
Small IDs and tick boundaries are safe integer JSON numbers. There is no backend,
account service, external AI, save protocol or runtime network dependency.

## Local development and static build

Prerequisites: stable Rust with `wasm32-unknown-unknown`, Node 24+ and npm,
wasm-bindgen CLI exactly matching the pinned crate version 0.2.130.
From the repository root:

```sh
rustup target add wasm32-unknown-unknown
cargo install wasm-bindgen-cli --version 0.2.130 --locked
cargo build --locked --target wasm32-unknown-unknown -p bts-fuzzy -p bts-sim -p bts-web
npm --prefix web ci
npm --prefix web run wasm
npm --prefix web run dev
```

Use the printed localhost HTTP URL. `file://` is not supported. After Rust changes,
rerun `npm --prefix web run wasm`; Vite handles TypeScript/CSS changes. A custom
CLI path may be supplied with `WASM_BINDGEN=/path/to/wasm-bindgen`.

```sh
npm --prefix web run build
# Once, in web/:
cd web
npx playwright install --with-deps chromium
npm run test:e2e
npm run preview
# Or from the repository root:
# npm --prefix web run preview -- --host 0.0.0.0
```

The build compiles release Wasm, generates bindings, checks TypeScript and writes
the static **`web/dist/`** bundle. Serve that directory as a unit over HTTP with
`.wasm` as `application/wasm`; relative URLs support hosting under a subdirectory.
Preview normally uses port 4173. Playwright starts/stops its own preview on
**4174**, so a manual preview can stay open. `PLAYWRIGHT_BROWSERS_PATH` may select
a browser cache directory. Generated bindings, node_modules, dist and test
artifacts are ignored; Cargo.lock and package-lock.json are committed.

CI retains native tests/doctests, formatting, Clippy, docs, original examples,
all CLI exports and the copied standalone fuzzy checks. It builds all three
libraries for Wasm, runs the planning example, and builds/tests/uploads a static
browser bundle. Uploading a bundle is not deploying it. The committed
[planning evidence](../contrib/programme-planning/README.md) records actual
local results; the [planning specification](programme-planning.md) distinguishes
implemented behaviour from remaining proposals and limitations.
