import { useEffect, useState } from "react";
import { useAuth } from "../context/AuthContext";
import { Card } from "../components/Card";
import { Button } from "../components/Button";
import { api } from "../lib/api";
import type { Device } from "../types";

export function DevicesPage() {
  const { user } = useAuth();

  const [devices, setDevices] = useState<Device[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    if (!user) return;

    api
      .listDevices(user.id)
      .then(setDevices)
      .finally(() => setLoading(false));
  }, [user]);

  return (
    <div className="max-w-4xl">
      <h1 className="text-2xl font-semibold mb-1">Devices</h1>

      <p className="text-sm text-[var(--color-text-muted)] mb-8">
        Machines you've backed up with Restorely.
      </p>

      {loading && (
        <p className="text-sm text-[var(--color-text-muted)]">
          Loading...
        </p>
      )}

      {!loading && devices.length === 0 && (
        <Card className="text-center py-12">
          <p className="text-sm text-[var(--color-text-muted)]">
            No devices yet. Scan and upload a snapshot from the Dashboard to
            see it here.
          </p>
        </Card>
      )}

      <div className="space-y-4">
        {devices.map((d) => (
          <Card key={d.id}>
            <div className="flex items-start justify-between">
              <div className="space-y-2">
                <h2 className="text-lg font-semibold">
                  {d.device_name}
                </h2>

                <p className="text-sm text-[var(--color-text-muted)]">
                  {d.os} {d.os_version}
                </p>

                <div className="grid grid-cols-2 gap-x-8 gap-y-1 text-sm">
                  <p>
                    <strong>Hostname:</strong> {d.hostname}
                  </p>

                  <p>
                    <strong>CPU:</strong> {d.cpu}
                  </p>

                  <p>
                    <strong>RAM:</strong> {d.ram_gb} GB
                  </p>

                  <p>
                    <strong>Disk:</strong> {d.disk_gb} GB
                  </p>

                  <p>
                    <strong>Last Backup:</strong>{" "}
                    {new Date(d.last_backup).toLocaleString()}
                  </p>
                </div>
              </div>

              <Button variant="secondary">
                Restore
              </Button>
            </div>
          </Card>
        ))}
      </div>
    </div>
  );
}