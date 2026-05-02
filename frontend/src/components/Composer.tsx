"use client";

import { Loader2, SendHorizontal } from "lucide-react";
import { FormEvent, useState } from "react";

type ComposerProps = {
  disabled?: boolean;
  isSending: boolean;
  onSend: (content: string) => Promise<void> | void;
};

export function Composer({ disabled = false, isSending, onSend }: ComposerProps) {
  const [content, setContent] = useState("");
  const blocked = disabled || isSending;

  const submit = async (event: FormEvent<HTMLFormElement>) => {
    event.preventDefault();
    const trimmed = content.trim();
    if (!trimmed || blocked) {
      return;
    }

    await onSend(trimmed);
    setContent("");
  };

  return (
    <form
      aria-label="Chat composer"
      className="border-t border-line bg-panel px-3 py-3 sm:px-6"
      onSubmit={submit}
    >
      <div className="mx-auto flex max-w-3xl items-end gap-2 rounded-lg border border-line bg-white p-2 shadow-sm focus-within:border-accent focus-within:ring-2 focus-within:ring-accent/15">
        <textarea
          aria-label="Message"
          className="max-h-40 min-h-11 flex-1 resize-none bg-transparent px-2 py-2 text-sm leading-6 text-ink outline-none placeholder:text-muted disabled:cursor-not-allowed disabled:opacity-60"
          disabled={blocked}
          placeholder="Message gpt-copy-v7"
          rows={1}
          value={content}
          onChange={(event) => setContent(event.target.value)}
          onKeyDown={(event) => {
            if (event.key === "Enter" && !event.shiftKey) {
              event.preventDefault();
              event.currentTarget.form?.requestSubmit();
            }
          }}
        />
        <button
          aria-label={isSending ? "Sending message" : "Send message"}
          className="grid h-10 w-10 flex-none place-items-center rounded-md bg-accent text-white transition hover:bg-teal-800 focus:outline-none focus:ring-2 focus:ring-accent focus:ring-offset-2 disabled:cursor-not-allowed disabled:bg-neutral-300"
          disabled={blocked}
          type="submit"
        >
          {isSending ? (
            <Loader2 aria-hidden="true" className="h-4 w-4 animate-spin" />
          ) : (
            <SendHorizontal aria-hidden="true" className="h-4 w-4" />
          )}
        </button>
      </div>
    </form>
  );
}
