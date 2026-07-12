import { useEffect, useState } from "react";
import { listen } from "@tauri-apps/api/event";
import { Card } from "../components/Card";
import type { RestoreProgress } from "../types";

export function RestorePage() {
  const [log, setLog] = useState<RestoreProgress[]>([]);

  useEffect(() => {
    const unlisten = listen<RestoreProgress>("restore://progress", (event) => {
      setLog((prev) => [...prev, event.payload]);
    });

    return () => {
      void unlisten.then((fn) => fn());
    };
  }, []);

  return (
    <div className="max-w-3xl">
      <h1 className="text-2xl font-semibold mb-1">Restore</h1>
      <p className="text-sm text-[var(--color-text-muted)] mb-8">
        Live progress of your workspace restoration.
      </p>

      <Card className="font-mono text-sm">
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
