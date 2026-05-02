import { render, screen } from "@testing-library/react";
import { describe, expect, test } from "vitest";

import { Transcript } from "@/components/Transcript";

describe("Transcript", () => {
  test("renders assistant markdown with GFM table and code", () => {
    render(
      <Transcript
        isLoading={false}
        messages={[
          {
            id: "m1",
            conversation_id: "c1",
            role: "assistant",
            content: "| Item | Status |\n| --- | --- |\n| Tests | Passing |\n\n```ts\nconst ok = true;\n```",
            created_at: "2026-05-02T12:00:00Z"
          }
        ]}
      />
    );

    expect(screen.getByRole("table")).toBeInTheDocument();
    expect(screen.getByText("const ok = true;")).toBeInTheDocument();
  });

  test("does not render raw HTML or script markup from assistant content", () => {
    render(
      <Transcript
        isLoading={false}
        messages={[
          {
            id: "m1",
            conversation_id: "c1",
            role: "assistant",
            content: "Safe text\n\n<script>window.evil = true</script>",
            created_at: "2026-05-02T12:00:00Z"
          }
        ]}
      />
    );

    expect(screen.getByText("Safe text")).toBeInTheDocument();
    expect(screen.queryByText(/window\.evil/)).not.toBeInTheDocument();
  });
});
