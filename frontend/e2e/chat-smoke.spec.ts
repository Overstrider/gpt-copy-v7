import { expect, test } from "@playwright/test";

const apiBase = "http://localhost:8080";
const providerHostFragment = ["open", "router"].join("");

test("sends one streamed message through mocked backend calls only", async ({ page }) => {
  const forbiddenRequests: string[] = [];

  page.on("request", (request) => {
    const url = request.url().toLowerCase();
    if (url.includes(providerHostFragment)) {
      forbiddenRequests.push(request.url());
    }
  });

  await page.route(`${apiBase}/api/conversations`, async (route, request) => {
    if (request.method() === "POST") {
      await route.fulfill({
        status: 201,
        contentType: "application/json",
        body: JSON.stringify({
          id: "c1",
          title: "Smoke test",
          created_at: "2026-05-02T12:00:00Z",
          updated_at: "2026-05-02T12:00:00Z"
        })
      });
      return;
    }

    await route.fulfill({
      status: 200,
      contentType: "application/json",
      body: JSON.stringify([
        {
          id: "c1",
          title: "Smoke test",
          created_at: "2026-05-02T12:00:00Z",
          updated_at: "2026-05-02T12:00:00Z"
        }
      ])
    });
  });

  await page.route(`${apiBase}/api/conversations/c1/messages`, async (route) => {
    await route.fulfill({
      status: 200,
      contentType: "application/json",
      body: JSON.stringify([])
    });
  });

  await page.route(`${apiBase}/api/conversations/c1/messages/stream`, async (route) => {
    await route.fulfill({
      status: 200,
      contentType: "text/event-stream",
      body:
        'event: assistant_delta\ndata: {"content":"Hello"}\n\n' +
        'event: assistant_delta\ndata: {"content":" from mocked stream"}\n\n' +
        "event: done\ndata: {}\n\n"
    });
  });

  await page.goto("/");
  await expect(page.getByRole("heading", { name: "Smoke test" })).toBeVisible();

  await page.getByRole("textbox", { name: "Message" }).fill("Say hello");
  await page.getByRole("button", { name: "Send message" }).click();

  await expect(page.getByText("Say hello")).toBeVisible();
  await expect(page.getByText("Hello from mocked stream")).toBeVisible();
  expect(forbiddenRequests).toEqual([]);
});
