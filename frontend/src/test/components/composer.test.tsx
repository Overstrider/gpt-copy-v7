import { render, screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { describe, expect, test, vi } from "vitest";

import { Composer } from "@/components/Composer";

describe("Composer", () => {
  test("rejects empty and whitespace-only messages", async () => {
    const onSend = vi.fn();
    const user = userEvent.setup();

    render(<Composer disabled={false} isSending={false} onSend={onSend} />);

    await user.type(screen.getByLabelText("Message"), "   ");
    await user.click(screen.getByRole("button", { name: "Send message" }));

    expect(onSend).not.toHaveBeenCalled();
  });

  test("submits trimmed messages and clears the input", async () => {
    const onSend = vi.fn().mockResolvedValue(undefined);
    const user = userEvent.setup();

    render(<Composer disabled={false} isSending={false} onSend={onSend} />);

    await user.type(screen.getByLabelText("Message"), "  Hello assistant  ");
    await user.click(screen.getByRole("button", { name: "Send message" }));

    expect(onSend).toHaveBeenCalledWith("Hello assistant");
    expect(screen.getByLabelText("Message")).toHaveValue("");
  });

  test("disables text entry and send control while sending", () => {
    render(<Composer disabled={false} isSending onSend={vi.fn()} />);

    expect(screen.getByLabelText("Message")).toBeDisabled();
    expect(screen.getByRole("button", { name: "Sending message" })).toBeDisabled();
  });
});
