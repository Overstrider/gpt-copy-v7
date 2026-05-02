import ReactMarkdown from "react-markdown";
import remarkGfm from "remark-gfm";

import type { Message } from "@/lib/schemas";

type TranscriptProps = {
  isLoading: boolean;
  messages: Message[];
};

export function Transcript({ isLoading, messages }: TranscriptProps) {
  if (isLoading) {
    return (
      <div className="mx-auto flex w-full max-w-3xl flex-1 flex-col gap-4 px-4 py-6">
        <div className="h-20 animate-pulse rounded-lg bg-neutral-200" />
        <div className="h-28 animate-pulse rounded-lg bg-neutral-200" />
      </div>
    );
  }

  if (messages.length === 0) {
    return (
      <div className="flex min-h-full flex-1 items-center justify-center px-4 py-12">
        <div className="max-w-lg text-center">
          <h2 className="text-2xl font-semibold text-ink">What can I help with?</h2>
        </div>
      </div>
    );
  }

  return (
    <div className="mx-auto flex w-full max-w-3xl flex-1 flex-col gap-5 px-4 py-6 sm:px-6">
      {messages.map((message) => {
        const assistant = message.role === "assistant";
        return (
          <article
            key={message.id}
            className={assistant ? "flex justify-start" : "flex justify-end"}
          >
            <div
              className={
                assistant
                  ? "max-w-[92%] rounded-lg border border-line bg-panel px-4 py-3 text-sm leading-6 text-ink shadow-sm sm:max-w-[82%]"
                  : "max-w-[92%] rounded-lg bg-accent px-4 py-3 text-sm leading-6 text-white shadow-sm sm:max-w-[74%]"
              }
            >
              {assistant ? (
                <ReactMarkdown
                  components={{
                    code({ children }) {
                      return (
                        <code className="rounded bg-neutral-100 px-1.5 py-0.5 font-mono text-[0.85em] text-neutral-950">
                          {children}
                        </code>
                      );
                    },
                    pre({ children }) {
                      return (
                        <pre className="my-3 overflow-x-auto rounded-md bg-neutral-950 p-3 text-xs text-white">
                          {children}
                        </pre>
                      );
                    },
                    table({ children }) {
                      return (
                        <div className="my-3 overflow-x-auto">
                          <table className="min-w-full border-collapse text-left text-sm">
                            {children}
                          </table>
                        </div>
                      );
                    },
                    th({ children }) {
                      return <th className="border border-line bg-neutral-100 px-2 py-1">{children}</th>;
                    },
                    td({ children }) {
                      return <td className="border border-line px-2 py-1">{children}</td>;
                    }
                  }}
                  remarkPlugins={[remarkGfm]}
                  skipHtml
                >
                  {message.content}
                </ReactMarkdown>
              ) : (
                <p className="whitespace-pre-wrap">{message.content}</p>
              )}
            </div>
          </article>
        );
      })}
    </div>
  );
}
