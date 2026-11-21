import { defineConfig } from '@playwright/test';

const port = Number(process.env.BTS_E2E_PORT ?? 4186);
if (!Number.isInteger(port) || port < 1 || port > 65535) throw new Error('BTS_E2E_PORT must be a valid TCP port.');
const url = `http://127.0.0.1:${port}`;
export default defineConfig({
  testDir: './tests',
  fullyParallel: false,
  use: { baseURL: url, headless: true },
  webServer: { command: `npm run preview -- --port ${port}`, url, reuseExistingServer: false },
});
