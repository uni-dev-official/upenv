import type { ButtonHTMLAttributes } from "react";

interface ButtonProps extends ButtonHTMLAttributes<HTMLButtonElement> {
  variant?: "primary" | "secondary" | "danger";
}

export function Button({
  variant = "primary",
  className = "",
  ...props
}: ButtonProps) {
  const base =
    "inline-flex items-center justify-center rounded-xl px-5 py-2.5 text-sm font-semibold transition-all duration-200 disabled:opacity-50 disabled:cursor-not-allowed";

  const styles =
    variant === "primary"
      ? `
        bg-[var(--color-accent)]
        text-white
        shadow-lg
        shadow-blue-500/20
        hover:bg-[var(--color-accent-hover)]
        hover:-translate-y-0.5
        hover:shadow-blue-500/40
        active:translate-y-0
      `
      : variant === "secondary"
      ? `
        border
        border-[var(--color-border)]
        bg-[var(--color-surface)]
        text-[var(--color-text)]
        hover:bg-[var(--color-surface-hover)]
        hover:border-[var(--color-accent)]
        hover:-translate-y-0.5
      `
      : `
        bg-red-600
        text-white
        hover:bg-red-700
      `;

  return (
    <button
      className={`${base} ${styles} ${className}`}
      {...props}
    />
  );
}