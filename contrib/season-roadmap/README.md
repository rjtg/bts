# Season roadmap review

## Starting point

The goal started at `f179eb3` with an empty index and working-tree changes
for exact-minute airtime, the Station Office and its design/capture evidence.
These are prerequisites for the new work. Their baseline is tracked separately
from the nine milestone commits. Baseline: `e178404` (workspace Rust tests/doctests, production Wasm/browser build, 40 Chromium tests).

## Review

From the repository root, enter `web/` before running these commands.
These paths describe the local tool installation used for the captures.

```sh
RUSTUP_HOME=/tmp/bts-rust-1100 RUSTUP_TOOLCHAIN=1.100.0 WASM_BINDGEN=/tmp/bts-web-tools/bin/wasm-bindgen npm run build
RUSTUP_HOME=/tmp/bts-rust-1100 RUSTUP_TOOLCHAIN=1.100.0 PLAYWRIGHT_BROWSERS_PATH=/tmp/bts-playwright npm run test:e2e
npm run preview -- --host 0.0.0.0
```

Open port 4173 on the preview host. The existing network address is
`http://100.71.132.116:4173/`. For historical review, use
`git worktree add --detach <directory> <commit>` and build there.

## Progress checkpoint

- Active slice: longer station loop and scoped autosell.
- Verified milestone: Add playable weekly discussion runs.
- Next: extend the scenario and connect per-run autosell to the minute market.
- Runtime token refresh schedule: unavailable.

## 1. Recurring discussion editions

Find this slice with `git log --oneline --grep='Add playable weekly discussion runs'`.
The [implemented rules](../../docs/recurring-discussion.md) describe the timing,
booking commitments and missed editions.

1. Open Production and expand **Make this a weekly show**. Name the show
   Studio Roundtable and choose 18:01. Open the three-edition binder.
2. Book edition one. Finish it, open its Airing file, run it and review its
   completed result.
3. Return to Production. Edit edition two and replace Nora Vale with Rafi
   Linden. Choose Entertainment & culture and Lively. Edition one's
   displayed brief remains unchanged.
4. Book edition two. Replace Nora with Rafi on edition three as well, since
   Nora's ordinary availability covers only the start of the scenario.
   Book, finish and air both episodes.

The native fixture checks three distinct episodes and airings, protection of
the reserved interval, no charge for missed editions, 3,000 Funds in total
production costs, money conservation and exact replay of the later editions.
The browser walkthrough checks that the edited second brief leaves the first
intact. It does not establish that the new brief improves reception.

[Chromium capture of the binder](discussion-binder.png). Workspace tests and
doctests, strict Clippy and rustdoc, formatting, isolated fuzzy tests,
Wasm/browser build and all 41 Chromium tests passed.
