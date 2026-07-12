import { useAuth } from "../context/AuthContext";
import { Card } from "../components/Card";

export function SettingsPage() {
  const { user } = useAuth();

  return (
    <div className="max-w-3xl">
      <h1 className="text-2xl font-semibold mb-1">Settings</h1>
      <p className="text-sm text-[var(--color-text-muted)] mb-8">
        Account and application preferences.
      </p>

      <Card>
        <p className="text-xs text-[var(--color-text-muted)] mb-1">Email</p>
        <p className="text-sm font-medium">{user?.email}</p>
      </Card>
    </div>
  );
}
