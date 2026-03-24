import { test, expect } from '@playwright/test';

const BASE_URL = 'http://localhost:5173';
const API_URL = 'http://localhost:3001';

const testUser = {
  email: `test-${Date.now()}@example.com`,
  username: `testuser${Date.now()}`,
  password: 'TestPassword123!',
};

test.describe('Critical User Journeys', () => {
  test('should complete full user flow: register → login → search → add to watchlist → view profile', async ({
    page,
  }) => {
    await page.goto(`${BASE_URL}/login`);
    await expect(page).toHaveTitle(/ANIME/i);

    await page.click('text=Need an account? Register');
    await page.fill('input[placeholder*="Username"]', testUser.username);
    await page.fill('input[type="email"]', testUser.email);
    await page.fill('input[type="password"]', testUser.password);
    await page.click('button:has-text("Register")');

    await page.waitForURL(`${BASE_URL}/recommendations`, { timeout: 10000 });
    await expect(page).toContainText(testUser.username);
  });

  test('should login and view recommendations', async ({ page }) => {
    await page.goto(`${BASE_URL}/login`);
    const newUser = {
      email: `test-${Date.now()}@example.com`,
      username: `testuser${Date.now()}`,
      password: 'TestPassword123!',
    };
    await page.click('text=Need an account? Register');
    await page.fill('input[placeholder*="Username"]', newUser.username);
    await page.fill('input[type="email"]', newUser.email);
    await page.fill('input[type="password"]', newUser.password);
    await page.click('button:has-text("Register")');
    await page.waitForURL(`${BASE_URL}/recommendations`);

    await page.click('button:has-text("Logout")');
    await page.waitForURL(`${BASE_URL}/`);

    await page.goto(`${BASE_URL}/login`);
    await page.fill('input[type="email"]', newUser.email);
    await page.fill('input[type="password"]', newUser.password);
    await page.click('button:has-text("Login")');
    await page.waitForURL(`${BASE_URL}/recommendations`);
    await expect(page).toContainText(newUser.username);
  });

  test('should navigate discovery and search anime', async ({ page }) => {
    await page.goto(`${BASE_URL}/login`);
    const newUser = {
      email: `test-${Date.now()}@example.com`,
      username: `testuser${Date.now()}`,
      password: 'TestPassword123!',
    };
    await page.click('text=Need an account? Register');
    await page.fill('input[placeholder*="Username"]', newUser.username);
    await page.fill('input[type="email"]', newUser.email);
    await page.fill('input[type="password"]', newUser.password);
    await page.click('button:has-text("Register")');
    await page.waitForURL(`${BASE_URL}/recommendations`);

    await page.click('a:has-text("Discovery")');
    await page.waitForURL(`${BASE_URL}/discovery`);

    await page.fill('input[placeholder*="Search"]', 'Naruto');
    await page.click('button:has-text("Search")');

    await page.waitForSelector('text=Showing', { timeout: 10000 });
  });

  test('should add anime to watchlist and view in profile', async ({ page }) => {
    await page.goto(`${BASE_URL}/login`);
    const newUser = {
      email: `test-${Date.now()}@example.com`,
      username: `testuser${Date.now()}`,
      password: 'TestPassword123!',
    };
    await page.click('text=Need an account? Register');
    await page.fill('input[placeholder*="Username"]', newUser.username);
    await page.fill('input[type="email"]', newUser.email);
    await page.fill('input[type="password"]', newUser.password);
    await page.click('button:has-text("Register")');
    await page.waitForURL(`${BASE_URL}/recommendations`);

    await page.click('a:has-text("Discovery")');
    await page.fill('input[placeholder*="Search"]', 'Naruto');
    await page.click('button:has-text("Search")');
    await page.waitForSelector('text=Showing', { timeout: 10000 });

    const firstAnimeCard = page.locator('[class*="card"]').first();
    const addButton = firstAnimeCard.locator('button:has-text("Add")');
    await addButton.click();
    await page.waitForTimeout(1000);

    await page.click('a:has-text("Profile")');
    await page.waitForURL(`${BASE_URL}/profile`);

    await expect(page).toContainText(/Watching|Watchlist/i);
  });

  test('should filter search results by genre', async ({ page }) => {
    await page.goto(`${BASE_URL}/login`);
    const newUser = {
      email: `test-${Date.now()}@example.com`,
      username: `testuser${Date.now()}`,
      password: 'TestPassword123!',
    };
    await page.click('text=Need an account? Register');
    await page.fill('input[placeholder*="Username"]', newUser.username);
    await page.fill('input[type="email"]', newUser.email);
    await page.fill('input[type="password"]', newUser.password);
    await page.click('button:has-text("Register")');
    await page.waitForURL(`${BASE_URL}/recommendations`);

    await page.click('a:has-text("Discovery")');

    await page.fill('input[placeholder*="Search"]', 'anime');
    await page.click('button:has-text("Search")');
    await page.waitForSelector('text=Showing', { timeout: 10000 });

    const genreSelect = page.locator('select').first();
    await genreSelect.selectOption('Action');
    await page.waitForTimeout(1000);

    await expect(page).toContainText('Filters');
  });

  test('should update watchlist status', async ({ page }) => {
    await page.goto(`${BASE_URL}/login`);
    const newUser = {
      email: `test-${Date.now()}@example.com`,
      username: `testuser${Date.now()}`,
      password: 'TestPassword123!',
    };
    await page.click('text=Need an account? Register');
    await page.fill('input[placeholder*="Username"]', newUser.username);
    await page.fill('input[type="email"]', newUser.email);
    await page.fill('input[type="password"]', newUser.password);
    await page.click('button:has-text("Register")');
    await page.waitForURL(`${BASE_URL}/recommendations`);

    await page.click('a:has-text("Discovery")');
    await page.fill('input[placeholder*="Search"]', 'Naruto');
    await page.click('button:has-text("Search")');
    await page.waitForSelector('text=Showing', { timeout: 10000 });
    const firstAnimeCard = page.locator('[class*="card"]').first();
    await firstAnimeCard.locator('button:has-text("Add")').click();

    await page.click('a:has-text("Profile")');
    await page.waitForURL(`${BASE_URL}/profile`);

    const statusSelect = page.locator('select').first();
    await statusSelect.selectOption('completed');
    await page.waitForTimeout(500);

    const updatedStatus = page.locator('select').first();
    const value = await updatedStatus.inputValue();
    expect(value).toBe('completed');
  });

  test('should navigate all main pages without errors', async ({ page }) => {
    await page.goto(`${BASE_URL}/login`);
    const newUser = {
      email: `test-${Date.now()}@example.com`,
      username: `testuser${Date.now()}`,
      password: 'TestPassword123!',
    };
    await page.click('text=Need an account? Register');
    await page.fill('input[placeholder*="Username"]', newUser.username);
    await page.fill('input[type="email"]', newUser.email);
    await page.fill('input[type="password"]', newUser.password);
    await page.click('button:has-text("Register")');
    await page.waitForURL(`${BASE_URL}/recommendations`);

    const pages = [
      { href: '/recommendations', text: 'Your Recommendations' },
      { href: '/discovery', text: 'Discover Anime' },
      { href: '/profile', text: 'Profile' },
    ];

    for (const p of pages) {
      await page.goto(`${BASE_URL}${p.href}`);
      await expect(page).toContainText(p.text);
    }
  });
});

test.describe('Error Handling', () => {
  test('should handle failed login gracefully', async ({ page }) => {
    await page.goto(`${BASE_URL}/login`);
    await page.fill('input[type="email"]', 'nonexistent@example.com');
    await page.fill('input[type="password"]', 'WrongPassword123!');
    await page.click('button:has-text("Login")');

    // Check for error message
    await expect(page).toContainText(/failed|error|invalid/i);
  });

  test('should redirect unauthenticated users from protected pages', async ({ page }) => {
    await page.goto(`${BASE_URL}/profile`);
    // Should redirect to login or show error
    const url = page.url();
    expect(url).toContain('/login');
  });
});
