import type { ButtonHTMLAttributes } from "react";

interface ButtonProps extends ButtonHTMLAttributes<HTMLButtonElement> {
  variant?: "primary" | "secondary";
}

export function Button({ variant = "primary", className = "", ...props }: ButtonProps) {
  const base =
    "inline-flex items-center justify-center rounded-md px-4 py-2 text-sm font-medium transition-colors disabled:opacity-50 disabled:cursor-not-allowed";

  const styles =
    variant === "primary"
      ? "bg-[var(--color-accent)] text-white hover:bg-[var(--color-accent-hover)]"
      : "bg-[var(--color-surface-hover)] text-[var(--color-text)] hover:bg-[var(--color-border)] border border-[var(--color-border)]";

  return <button className={`${base} ${styles} ${className}`} {...props} />;
}
