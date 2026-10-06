// Turns any error (from Rust or elsewhere) into a message a non-technical
// user can understand. The texts live in `i18n/fr.ts` (`errors`).
import { toast } from "sonner";
import { t } from "@/i18n";
import type { AppError } from "@/types";

function isAppError(error: unknown): error is AppError {
  return (
    typeof error === "object" &&
    error !== null &&
    "code" in error &&
    typeof error.code === "string" &&
    error.code in t.errors
  );
}

export function errorMessage(error: unknown): string {
  return isAppError(error) ? t.errors[error.code] : t.errors.unknown;
}

/** Shows the error in a toast and logs the technical detail. */
export function showError(error: unknown) {
  console.error(error);
  toast.error(errorMessage(error));
}
