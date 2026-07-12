import { NavLink } from "react-router-dom";
import { useAuth } from "../context/AuthContext";

const links = [
  { to: "/dashboard", label: "Dashboard" },
  { to: "/devices", label: "Devices" },
  { to: "/restore", label: "Restore" },
  { to: "/settings", label: "Settings" },
];

export function Sidebar() {
  const { logout, user } = useAuth();

  return (
    <aside className="w-56 shrink-0 border-r border-[var(--color-border)] bg-[var(--color-surface)] flex flex-col h-full">
      <div className="px-5 py-6">
        <span className="text-lg font-semibold tracking-tight">Restorely</span>
      </div>

      <nav className="flex-1 px-3 space-y-1">
        {links.map((link) => (
          <NavLink
            key={link.to}
            to={link.to}
            className={({ isActive }) =>
              `block rounded-md px-3 py-2 text-sm transition-colors ${
                isActive
                  ? "bg-[var(--color-accent)] text-white"
                  : "text-[var(--color-text-muted)] hover:bg-[var(--color-surface-hover)] hover:text-[var(--color-text)]"
              }`
            }
          >
            {link.label}
          </NavLink>
        ))}
      </nav>

      <div className="px-3 py-4 border-t border-[var(--color-border)]">
        <p className="px-3 text-xs text-[var(--color-text-muted)] truncate">
          {user?.email}
        </p>
        <button
          onClick={() => void logout()}
          className="mt-2 w-full text-left rounded-md px-3 py-2 text-sm text-[var(--color-text-muted)] hover:bg-[var(--color-surface-hover)] hover:text-[var(--color-danger)] transition-colors"
        >
          Log out
        </button>
      </div>
    </aside>
  );
}
