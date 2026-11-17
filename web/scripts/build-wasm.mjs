import { execFileSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';
const root = fileURLToPath(new URL('../../', import.meta.url));
const bindgen = process.env.WASM_BINDGEN ?? 'wasm-bindgen';
const version = execFileSync(bindgen, ['--version'], { encoding: 'utf8' }).trim();
if (version !== 'wasm-bindgen 0.2.130') throw new Error(`Expected wasm-bindgen 0.2.130, got ${version}`);
execFileSync('cargo', ['build', '--locked', '--release', '--target', 'wasm32-unknown-unknown', '-p', 'bts-web'], { cwd: root, stdio: 'inherit' });
execFileSync(bindgen, [`${root}target/wasm32-unknown-unknown/release/bts_web.wasm`, '--target', 'web', '--out-dir', `${root}web/pkg`], { stdio: 'inherit' });
