import { AlertTriangle, X } from "lucide-react";

import { toAppError } from "@/lib/errors";

type ErrorBannerProps = {
  error: unknown;
  onDismiss?: () => void;
};

export function ErrorBanner({ error, onDismiss }: ErrorBannerProps) {
  if (!error) {
    return null;
  }

  const appError = toAppError(error);

  return (
    <div
      role="alert"
      className="flex items-start gap-3 border-b border-amber-200 bg-amber-50 px-4 py-3 text-sm text-amber-950"
    >
      <AlertTriangle aria-hidden="true" className="mt-0.5 h-4 w-4 flex-none" />
      <div className="min-w-0 flex-1">
        <p className="font-medium">{appError.message}</p>
        <p className="mt-0.5 text-xs uppercase tracking-normal text-amber-800">{appError.code}</p>
      </div>
      {onDismiss ? (
        <button
          aria-label="Dismiss error"
          className="rounded p-1 text-amber-900 hover:bg-amber-100 focus:outline-none focus:ring-2 focus:ring-amber-700"
          type="button"
          onClick={onDismiss}
        >
          <X aria-hidden="true" className="h-4 w-4" />
        </button>
      ) : null}
    </div>
  );
}
