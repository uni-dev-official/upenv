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

  // ---------------------------------------------------------
  // Listen for restore progress events from Rust
  // ---------------------------------------------------------

  useEffect(() => {
    const unlisten = listen<RestoreProgress>(
      "restore://progress",
      (event) => {
        setLog((prev) => [...prev, event.payload]);
      },
    );

    return () => {
      void unlisten.then((fn) => fn());
    };
  }, []);

  // ---------------------------------------------------------
  // Start restore
  // ---------------------------------------------------------

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

  // ---------------------------------------------------------
  // Render
  // ---------------------------------------------------------

  return (
    <div className="max-w-3xl">
      <h1 className="text-2xl font-semibold mb-1">
        Restore
      </h1>

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
          <p className="text-[var(--color-text-muted)] mb-4">
            Starting restore...
          </p>
        )}

        {log.length === 0 && !starting && (
          <p className="text-[var(--color-text-muted)]">
            No restore in progress. Start one from the Devices page.
          </p>
        )}

        <div className="space-y-4">
          {log.map((entry, index) => {
            const hasProgress =
              entry.current != null &&
              entry.total != null &&
              entry.total > 0;

            const percentage = hasProgress
              ? Math.min(
                  100,
                  (entry.current! / entry.total!) * 100,
                )
              : 0;

            return (
              <div key={index}>
                <div
                  className={
                    entry.done
                      ? "text-[var(--color-success)]"
                      : "text-[var(--color-text)]"
                  }
                >
                  {hasProgress && (
                    <span className="text-[var(--color-text-muted)] mr-2">
                      [{entry.current}/{entry.total}]
                    </span>
                  )}

                  {entry.message}
                </div>

                {hasProgress && !entry.done && (
                  <div className="mt-2 h-1.5 w-full rounded-full bg-[var(--color-border)] overflow-hidden">
                    <div
                      className="h-full rounded-full bg-[var(--color-primary)] transition-all duration-300"
                      style={{
                        width: `${percentage}%`,
                      }}
                    />
                  </div>
                )}
              </div>
            );
          })}
        </div>
      </Card>
    </div>
  );
}