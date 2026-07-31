import { useEffect, useState } from "react";
import { useNavigate } from "react-router-dom";
import { useAuth } from "../context/AuthContext";
import { Button } from "../components/Button";
import { Card } from "../components/Card";
import { api } from "../lib/api";
import type { Snapshot } from "../types";
import logo from "../logo.png";



export function DashboardPage() {
  const { user, accessToken } = useAuth();
  const navigate = useNavigate();

  const [snapshot, setSnapshot] = useState<Snapshot | null>(null);
  const [lastScan, setLastScan] = useState<string | null>(null);
  const [scanning, setScanning] = useState(false);
  const [error, setError] = useState<string | null>(null);


  useEffect(() => {
    const savedSnapshot = localStorage.getItem("snapshot");
    const savedLastScan = localStorage.getItem("lastScan");

    if (savedSnapshot) {
      setSnapshot(JSON.parse(savedSnapshot));
    }

    if (savedLastScan) {
      setLastScan(savedLastScan);
    }
  }, []);


  async function handleScan() {
    setScanning(true);
    setError(null);

    try {
      const data = await api.scanSnapshot();

      setSnapshot(data);

      const now = new Date().toLocaleString();

      setLastScan(now);

      localStorage.setItem(
        "snapshot",
        JSON.stringify(data)
      );

      localStorage.setItem(
        "lastScan",
        now
      );

    } catch (err) {
      setError(
        err instanceof Error
          ? err.message
          : "Scan failed"
      );
    } finally {
      setScanning(false);
    }
  }

async function handleUpload() {
  if (!snapshot || !user || !accessToken?.trim()) {
    return;
  }

  setError(null);

  try {
    await api.uploadSnapshot(
      user.id,
      snapshot.device_name,
      snapshot.hostname,
      snapshot.os,
      accessToken
    );
  } catch (err) {
    console.error("Upload snapshot failed:", err);
    setError(
      err instanceof Error ? err.message : String(err)
    );
  }
}


  return (
    <div className="max-w-5xl">

      <div className="flex items-center gap-4 mb-8">
  <img
    src={logo}
    alt="UPENV"
    className="w-20 h-20 object-contain"
  />

  <div>
    <h1 className="text-2xl font-semibold">
      Welcome{user ? `, ${user.email}` : ""}
    </h1>

    <p className="text-sm text-[var(--color-text-muted)]">
      Here's the state of your workspace.
    </p>
  </div>
</div>

      {error && (
        <p className="text-sm text-[var(--color-danger)] mb-6">
          {error}
        </p>
      )}


      <div className="grid grid-cols-2 md:grid-cols-3 gap-4 mb-8">


        <Card>
          <p className="text-xs text-[var(--color-text-muted)] mb-1">
            Current Device
          </p>

          <p className="text-lg font-medium">
            {snapshot?.device_name ?? "Not scanned yet"}
          </p>
        </Card>


        <Card>
          <p className="text-xs text-[var(--color-text-muted)] mb-1">
            Last Scan
          </p>

          <p className="text-lg font-medium">
            {lastScan ?? "Never"}
          </p>
        </Card>


        <Card>
          <p className="text-xs text-[var(--color-text-muted)] mb-1">
            Operating System
          </p>

          <p className="text-lg font-medium">
            {snapshot?.os ?? "-"}
          </p>
        </Card>


        <Card>
          <p className="text-xs text-[var(--color-text-muted)] mb-1">
            OS Version
          </p>

          <p className="text-lg font-medium">
            {snapshot?.os_version ?? "-"}
          </p>
        </Card>


        <Card>
          <p className="text-xs text-[var(--color-text-muted)] mb-1">
            CPU
          </p>

          <p className="text-lg font-medium">
            {snapshot?.cpu ?? "-"}
          </p>
        </Card>


        <Card>
          <p className="text-xs text-[var(--color-text-muted)] mb-1">
            RAM
          </p>

          <p className="text-lg font-medium">
            {snapshot?.ram_gb ?? 0} GB
          </p>
        </Card>


        <Card>
          <p className="text-xs text-[var(--color-text-muted)] mb-1">
            Disk
          </p>

          <p className="text-lg font-medium">
            {snapshot?.disk_gb ?? 0} GB
          </p>
        </Card>


        <Card>
          <p className="text-xs text-[var(--color-text-muted)] mb-1">
            Applications
          </p>

          <p className="text-lg font-medium">
            {snapshot?.applications?.length ?? 0}
          </p>
        </Card>


        <Card>
          <p className="text-xs text-[var(--color-text-muted)] mb-1">
            VS Code Extensions
          </p>

          <p className="text-lg font-medium">
            {snapshot?.vscode_extensions?.length ?? 0}
          </p>
        </Card>


      </div>


      <div className="flex gap-3">

        <Button
          onClick={() => void handleScan()}
          disabled={scanning}
        >
          {scanning
            ? "Scanning..."
            : "Scan Computer"}
        </Button>


        <Button
          variant="secondary"
          disabled={!snapshot || !user || !accessToken?.trim()}
          onClick={() => void handleUpload()}
        >
          Upload Snapshot
        </Button>


        <Button
          variant="secondary"
          onClick={() => navigate("/devices")}
        >
          Restore Device
        </Button>

      </div>

    </div>
  );
}