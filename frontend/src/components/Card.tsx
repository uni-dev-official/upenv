import type { HTMLAttributes } from "react";

export function Card({ className = "", ...props }: HTMLAttributes<HTMLDivElement>) {
  return (
    <div
      className={`
rounded-2xl
border
border-[var(--color-border)]
bg-[var(--color-surface)]
p-6
transition-all
duration-200
hover:border-[var(--color-accent)]
hover:-translate-y-1
hover:shadow-xl
hover:shadow-blue-500/10
${className}
`}
      {...props}
    />
  );
}
