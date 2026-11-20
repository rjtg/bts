# Shared paper index verification

The Research binder and Studio workbook now reuse one paper index for
showing a section, updating the corresponding button's pressed state and
moving focus. Research still preserves the selected group and unsubmitted
Insight amount on render. The Studio workbook still retains its underlying
form elements and opens a selected details section.

The focused Chromium test activates both workbooks by keyboard, checks focus
and the active paper, switches away and back to an unsubmitted selection,
then returns to the office. The same spec opens the newsroom at a narrow
width. The [journey review](../../docs/mockups/player-journeys/README.md)
keeps the separate design proposals and current screenshots.

## Checks

- Production browser build with TypeScript and Wasm.
- 54 Chromium tests, including native/Wasm parity. The suite used a temporary
  copy of the Playwright config on port 4191 because 4186 was occupied.
- Rust workspace tests and doctests with the locked dependency set.
- Rust formatting and `git diff --check`.
- The local TypeScript language server was unavailable. TypeScript was checked
  through the production build.

From `web/`, build with `npm run build` and run
`npm run test:e2e -- office-papers` for the focused keyboard and draft regression.
The [browser setup](../../docs/browser-foundation.md) lists toolchain
requirements and Chromium setup.
