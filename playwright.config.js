import {defineConfig} from '@playwright/test';
export default defineConfig({
 testDir:'tests/browser',timeout:30000,workers:1,
 use:{baseURL:'http://127.0.0.1:8777',launchOptions:process.env.SATELLITE_BROWSER_PATH?{executablePath:process.env.SATELLITE_BROWSER_PATH}:{}},
 webServer:{command:'python -m http.server 8777 --bind 127.0.0.1 --directory dist',url:'http://127.0.0.1:8777/HARBOR/',reuseExistingServer:!process.env.CI},
});
