import { defineConfig } from '@playwright/test';
export default defineConfig({
  testDir: './tests',
  fullyParallel: false,
  use: { baseURL: 'http://127.0.0.1:4176', headless: true },
  webServer: { command: 'npm run preview -- --port 4176', url: 'http://127.0.0.1:4176', reuseExistingServer: false },
});
