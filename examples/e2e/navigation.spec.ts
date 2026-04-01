import { test, expect } from "@playwright/test";

test.describe("Navigation — critical pages load", () => {
  test("homepage loads with expected content", async ({ page }) => {
    await page.goto("/");
    await expect(page).toHaveTitle(/your-app/i);
    await expect(page.locator("body")).toBeVisible();
  });

  test("discover page loads", async ({ page }) => {
    await page.goto("/discover");
    await expect(page).toHaveURL(/\/discover/);
    await expect(page.locator("body")).toBeVisible();
    // Page should render without a full-page error
    await expect(page.locator("text=Application error")).not.toBeVisible();
  });

  test("dashboard page loads", async ({ page }) => {
    await page.goto("/dashboard");
    await expect(page).toHaveURL(/\/dashboard/);
    await expect(page.locator("body")).toBeVisible();
    await expect(page.locator("text=Application error")).not.toBeVisible();
  });

  test("create page loads", async ({ page }) => {
    await page.goto("/create");
    await expect(page).toHaveURL(/\/create/);
    await expect(page.locator("body")).toBeVisible();
    await expect(page.locator("text=Application error")).not.toBeVisible();
  });

  test("agents page loads", async ({ page }) => {
    await page.goto("/agents");
    await expect(page).toHaveURL(/\/agents/);
    await expect(page.locator("body")).toBeVisible();
    await expect(page.locator("text=Application error")).not.toBeVisible();
  });

  test("marketplace-agents page loads", async ({ page }) => {
    await page.goto("/marketplace-agents");
    await expect(page).toHaveURL(/\/marketplace-agents/);
    await expect(page.locator("body")).toBeVisible();
    await expect(page.locator("text=Application error")).not.toBeVisible();
  });

  test("navigation links are accessible from the app shell", async ({
    page,
  }) => {
    await page.goto("/");
    // Verify at least one nav element exists
    const nav = page.locator("nav").first();
    await expect(nav).toBeVisible();
  });

  test("dark theme is applied by default", async ({ page }) => {
    await page.goto("/");
    const html = page.locator("html");
    await expect(html).toHaveClass(/dark/);
  });

  test("navigating between pages preserves app shell", async ({ page }) => {
    await page.goto("/discover");
    const shellBefore = await page.locator("nav").first().isVisible();

    await page.goto("/dashboard");
    const shellAfter = await page.locator("nav").first().isVisible();

    expect(shellBefore).toBe(true);
    expect(shellAfter).toBe(true);
  });
});
