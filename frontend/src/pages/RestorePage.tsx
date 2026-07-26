import { useEffect, useRef, useState } from "react";
import { listen } from "@tauri-apps/api/event";
import { useLocation } from "react-router-dom";
import { Card } from "../components/Card";
import { useAuth } from "../context/AuthContext";
import { api } from "../lib/api";
import type { RestoreProgress } from "../types";

interface RestoreRouteState {
  deviceId?: string;
}

export function RestorePage() {
  const location = useLocation();
  const { accessToken } = useAuth();
  const [log, setLog] = useState<RestoreProgress[]>([]);
  const [error, setError] = useState<string | null>(null);
  const [starting, setStarting] = useState(false);
  const startedDeviceRef = useRef<string | null>(null);

  useEffect(() => {
    const unlisten = listen<RestoreProgress>("restore://progress", (event) => {
      setLog((prev) => [...prev, event.payload]);
    });

    return () => {
      void unlisten.then((fn) => fn());
    };
  }, []);

  useEffect(() => {
    const state = location.state as RestoreRouteState | null;
    const deviceId = state?.deviceId;

    if (!deviceId || startedDeviceRef.current === deviceId) {
      return;
    }

    if (!accessToken?.trim()) {
      setError("Missing access token for restore");
      return;
    }

    startedDeviceRef.current = deviceId;
    setLog([]);
    setError(null);
    setStarting(true);

    void api
      .runRestore(deviceId, accessToken)
      .catch((err) => {
        setError(err instanceof Error ? err.message : String(err));
      })
      .finally(() => {
        setStarting(false);
      });
  }, [location.state, accessToken]);

  return (
    <div className="max-w-3xl">
      <h1 className="text-2xl font-semibold mb-1">Restore</h1>
      <p className="text-sm text-[var(--color-text-muted)] mb-8">
        Live progress of your workspace restoration.
      </p>

      {error && (
        <p className="text-sm text-[var(--color-danger)] mb-6">
          {error}
        </p>
      )}

      <Card className="font-mono text-sm">
        {starting && (
          <p className="text-[var(--color-text-muted)] mb-3">
            Starting restore...
          </p>
        )}
        {log.length === 0 && (
          <p className="text-[var(--color-text-muted)]">
            No restore in progress. Start one from the Devices page.
          </p>
        )}
        <ul className="space-y-1">
          {log.map((entry, i) => (
            <li key={i} className={entry.done ? "text-[var(--color-success)]" : ""}>
              {entry.message}
            </li>
          ))}
        </ul>
      </Card>
    </div>
  );
}
