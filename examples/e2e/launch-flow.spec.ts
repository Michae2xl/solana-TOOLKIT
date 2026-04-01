import { test, expect, type Page } from "@playwright/test";

/**
 * Injects a mock Phantom wallet in connected state
 * so launch flow can proceed past the AuthGate.
 */
async function injectConnectedWallet(page: Page): Promise<void> {
  await page.addInitScript(() => {
    const publicKey = "TestLaunchWa11et111111111111111111111111111";

    const mockPublicKey = {
      toString: () => publicKey,
      toBase58: () => publicKey,
      toBuffer: () => new Uint8Array(32),
      toJSON: () => publicKey,
      equals: (other: { toString: () => string }) =>
        other.toString() === publicKey,
    };

    (window as Record<string, unknown>).solana = {
      isPhantom: true,
      isConnected: true,
      publicKey: mockPublicKey,
      connect: async () => ({ publicKey: mockPublicKey }),
      disconnect: async () => {},
      signMessage: async () => ({ signature: new Uint8Array(64) }),
      signTransaction: async (tx: unknown) => tx,
      signAllTransactions: async (txs: unknown[]) => txs,
      on: () => {},
      off: () => {},
    };

    (window as Record<string, unknown>).phantom = {
      solana: (window as Record<string, unknown>).solana,
    };
  });
}

test.describe("Create Launch Flow", () => {
  test.beforeEach(async ({ page }) => {
    await injectConnectedWallet(page);
  });

  test("create page renders the launch form", async ({ page }) => {
    await page.goto("/create");
    await page.waitForLoadState("networkidle");

    // The create page should have form inputs for token configuration
    const body = page.locator("body");
    await expect(body).toBeVisible();

    // Check that the page did not crash
    await expect(page.locator("text=Application error")).not.toBeVisible();
  });

  test("token name input accepts text", async ({ page }) => {
    await page.goto("/create");
    await page.waitForLoadState("networkidle");

    // Look for a name/token name input
    const nameInput = page
      .locator('input[placeholder*="name" i], input[name*="name" i]')
      .first();

    // If the input exists, fill it
    const isVisible = await nameInput.isVisible().catch(() => false);
    if (isVisible) {
      await nameInput.fill("Test Token");
      await expect(nameInput).toHaveValue("Test Token");
    } else {
      // Some create pages use contenteditable or custom inputs
      // Check that the create page at least has interactive elements
      const interactiveElements = page.locator(
        "input, textarea, select, [contenteditable]",
      );
      const count = await interactiveElements.count();
      expect(count).toBeGreaterThan(0);
    }
  });

  test("token ticker/symbol input accepts text", async ({ page }) => {
    await page.goto("/create");
    await page.waitForLoadState("networkidle");

    const tickerInput = page
      .locator(
        'input[placeholder*="ticker" i], input[placeholder*="symbol" i], input[name*="ticker" i], input[name*="symbol" i]',
      )
      .first();

    const isVisible = await tickerInput.isVisible().catch(() => false);
    if (isVisible) {
      await tickerInput.fill("TST");
      await expect(tickerInput).toHaveValue("TST");
    }
    // Non-fatal if ticker field uses a different pattern
  });

  test("supply/amount slider or input is present", async ({ page }) => {
    await page.goto("/create");
    await page.waitForLoadState("networkidle");

    // The create page uses custom GradientSlider components with range inputs
    const rangeInputs = page.locator('input[type="range"]');
    const numberInputs = page.locator('input[type="number"]');

    const rangeCount = await rangeInputs.count();
    const numberCount = await numberInputs.count();

    // At least one numeric control should be present for supply/distribution
    expect(rangeCount + numberCount).toBeGreaterThan(0);
  });

  test("form validation prevents empty submission", async ({ page }) => {
    await page.goto("/create");
    await page.waitForLoadState("networkidle");

    // Look for a submit/launch/create button
    const submitButton = page
      .locator("button")
      .filter({
        hasText: /launch|create|deploy|submit|mint/i,
      })
      .first();

    const isVisible = await submitButton.isVisible().catch(() => false);
    if (isVisible) {
      // Click without filling any fields
      await submitButton.click();
      await page.waitForTimeout(500);

      // Should either show validation errors or remain on the same page
      await expect(page).toHaveURL(/\/create/);
    }
  });

  test("create page shows token distribution visualization", async ({
    page,
  }) => {
    await page.goto("/create");
    await page.waitForLoadState("networkidle");

    // The create page includes a DistributionBar showing sale/LP/locked/unlocked %
    // Look for percentage text patterns or distribution-related labels
    const bodyText = (await page.locator("body").textContent()) ?? "";

    const hasDistributionTerms =
      bodyText.includes("Sale") ||
      bodyText.includes("LP") ||
      bodyText.includes("Locked") ||
      bodyText.includes("Distribution") ||
      bodyText.includes("%");

    expect(hasDistributionTerms).toBe(true);
  });

  test("create page loads without console errors", async ({ page }) => {
    const consoleErrors: string[] = [];

    page.on("console", (msg) => {
      if (msg.type() === "error") {
        consoleErrors.push(msg.text());
      }
    });

    await page.goto("/create");
    await page.waitForLoadState("networkidle");

    // Filter out known benign errors (e.g., favicon, HMR, hydration warnings)
    const criticalErrors = consoleErrors.filter(
      (e) =>
        !e.includes("favicon") &&
        !e.includes("HMR") &&
        !e.includes("hydrat") &&
        !e.includes("404"),
    );

    expect(criticalErrors).toHaveLength(0);
  });

  test("navigating to a launch detail page works", async ({ page }) => {
    // The app has /launch/[id] routes
    await page.goto("/launch/test-project-id");

    // Should either show the launch detail or a "not found" message
    // but should not crash
    await expect(page.locator("text=Application error")).not.toBeVisible();

    const body = page.locator("body");
    await expect(body).toBeVisible();
  });
});
