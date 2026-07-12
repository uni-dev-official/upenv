import { useState, type FormEvent } from "react";
import { Link, useNavigate } from "react-router-dom";
import { useAuth } from "../context/AuthContext";
import { Button } from "../components/Button";
import { Card } from "../components/Card";

export function LoginPage() {
  const { login } = useAuth();
  const navigate = useNavigate();
  const [email, setEmail] = useState("");
  const [password, setPassword] = useState("");
  const [error, setError] = useState<string | null>(null);
  const [loading, setLoading] = useState(false);

  async function handleSubmit(e: FormEvent) {
    e.preventDefault();
    setError(null);
    setLoading(true);
    try {
      await login(email, password);
      navigate("/dashboard");
    } catch (err) {
      setError(err instanceof Error ? err.message : "Login failed");
    } finally {
      setLoading(false);
    }
  }

  return (
    <div className="flex h-screen items-center justify-center bg-[var(--color-bg)] text-[var(--color-text)]">
      <Card className="w-full max-w-sm">
        <h1 className="text-xl font-semibold mb-1">Welcome back</h1>
        <p className="text-sm text-[var(--color-text-muted)] mb-6">
          Log in to Restorely
        </p>

        <form onSubmit={handleSubmit} className="space-y-4">
          <div>
            <label className="block text-xs mb-1 text-[var(--color-text-muted)]">
              Email
            </label>
            <input
              type="email"
              required
              value={email}
              onChange={(e) => setEmail(e.target.value)}
              className="w-full rounded-md border border-[var(--color-border)] bg-[var(--color-bg)] px-3 py-2 text-sm outline-none focus:border-[var(--color-accent)]"
            />
          </div>

          <div>
            <label className="block text-xs mb-1 text-[var(--color-text-muted)]">
              Password
            </label>
            <input
              type="password"
              required
              value={password}
              onChange={(e) => setPassword(e.target.value)}
              className="w-full rounded-md border border-[var(--color-border)] bg-[var(--color-bg)] px-3 py-2 text-sm outline-none focus:border-[var(--color-accent)]"
            />
          </div>

          {error && <p className="text-sm text-[var(--color-danger)]">{error}</p>}

          <Button type="submit" disabled={loading} className="w-full">
            {loading ? "Logging in..." : "Log in"}
          </Button>
        </form>

        <p className="mt-4 text-sm text-[var(--color-text-muted)]">
          No account?{" "}
          <Link to="/register" className="text-[var(--color-accent)] hover:underline">
            Register
          </Link>
        </p>
      </Card>
    </div>
  );
}
