import { NavLink } from "react-router-dom";
import {
  LayoutDashboard,
  Monitor,
  RotateCcw,
  Settings,
  LogOut,
} from "lucide-react";
import { useAuth } from "../context/AuthContext";
import logo from "../logo.png";

const links = [
  {
    to: "/dashboard",
    label: "Dashboard",
    icon: LayoutDashboard,
  },
  {
    to: "/devices",
    label: "Devices",
    icon: Monitor,
  },
  {
    to: "/restore",
    label: "Restore",
    icon: RotateCcw,
  },
  {
    to: "/settings",
    label: "Settings",
    icon: Settings,
  },
];

export function Sidebar() {
  const { logout, user } = useAuth();

  return (
    <aside className="w-72 shrink-0 border-r border-[var(--color-border)] bg-[var(--color-surface)] flex flex-col">

      {/* Logo */}

      <div className="px-6 py-7 border-b border-[var(--color-border)]">

        <div className="flex items-center gap-4">

          <img
            src={logo}
            alt="UPENV"
            className="w-30 h-30 object-contain"
          />

          <div>
            <h1 className="text-xl font-bold tracking-tight">
              UpEnv
            </h1>

            <p className="text-xs text-[var(--color-text-muted)]">
              Continue where you left off.
            </p>

          </div>

        </div>

      </div>

      {/* Navigation */}

      <nav className="flex-1 px-4 py-6 space-y-2">

        {links.map((link) => {
          const Icon = link.icon;

          return (
            <NavLink
              key={link.to}
              to={link.to}
              className={({ isActive }) =>
                `
                flex
                items-center
                gap-3
                rounded-xl
                px-4
                py-3
                text-sm
                font-medium
                transition-all
                duration-200

                ${
                  isActive
                    ? `
                      bg-[var(--color-accent)]
                      text-white
                      shadow-lg
                      shadow-blue-500/20
                    `
                    : `
                      text-[var(--color-text-muted)]
                      hover:bg-[var(--color-surface-hover)]
                      hover:text-[var(--color-text)]
                    `
                }
                `
              }
            >
              <Icon size={18} />

              {link.label}
            </NavLink>
          );
        })}

      </nav>

      {/* User */}

      <div className="border-t border-[var(--color-border)] p-5">

        <div className="mb-4">

          <p className="text-xs text-[var(--color-text-muted)]">
            Signed in as
          </p>

          <p className="mt-1 truncate text-sm font-medium">
            {user?.email}
          </p>

        </div>

        <button
          onClick={() => void logout()}
          className="
          flex
          w-full
          items-center
          gap-3
          rounded-xl
          px-4
          py-3
          text-sm
          transition-all
          duration-200
          text-[var(--color-text-muted)]
          hover:bg-red-500/10
          hover:text-red-400
          "
        >
          <LogOut size={18} />

          Log out

        </button>

      </div>

    </aside>
  );
}