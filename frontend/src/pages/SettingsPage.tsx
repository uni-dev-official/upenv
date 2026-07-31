import { useAuth } from "../context/AuthContext";
import { useTheme } from "../context/ThemeContext";

import { Card } from "../components/Card";
import { Button } from "../components/Button";

export function SettingsPage() {
  const { user } = useAuth();
  const { theme, setTheme } = useTheme();

  return (
    <div className="max-w-3xl space-y-8">
      <div>
        <h1 className="text-3xl font-bold">Settings</h1>
        <p className="mt-1 text-sm text-[var(--color-text-muted)]">
          Manage your account and application preferences.
        </p>
      </div>

      {/* Account */}
      <Card className="space-y-4">
        <h2 className="text-lg font-semibold">Account</h2>

        <div>
          <p className="text-xs uppercase tracking-wide text-[var(--color-text-muted)]">
            Email
          </p>
          <p className="mt-1 text-sm font-medium">
            {user?.email ?? "Not signed in"}
          </p>
        </div>
      </Card>

      {/* Appearance */}
      <Card className="space-y-5">
        <div>
          <h2 className="text-lg font-semibold">Appearance</h2>
          <p className="text-sm text-[var(--color-text-muted)]">
            Choose how UPENV looks.
          </p>
        </div>

        <div className="flex gap-3 flex-wrap">
          <Button
            variant={theme === "light" ? "primary" : "secondary"}
            onClick={() => setTheme("light")}
          >
            ☀️ Light
          </Button>

          <Button
            variant={theme === "dark" ? "primary" : "secondary"}
            onClick={() => setTheme("dark")}
          >
            🌙 Dark
          </Button>

          <Button
            variant={theme === "system" ? "primary" : "secondary"}
            onClick={() => setTheme("system")}
          >
            💻 System
          </Button>
        </div>
      </Card>

      {/* About */}
      <Card className="space-y-3">
        <h2 className="text-lg font-semibold">About</h2>

        <div className="flex justify-between">
          <span className="text-[var(--color-text-muted)]">Application</span>
          <span className="font-medium">UPENV</span>
        </div>

        <div className="flex justify-between">
          <span className="text-[var(--color-text-muted)]">Version</span>
          <span className="font-medium">0.1.0</span>
        </div>
      </Card>
    </div>
  );
}