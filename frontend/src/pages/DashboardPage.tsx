import { useEffect, useState } from "react";
import { useNavigate } from "react-router-dom";
import { useAuth } from "../context/AuthContext";
import { Button } from "../components/Button";
import { Card } from "../components/Card";
import { api } from "../lib/api";
import type { Snapshot } from "../types";
import logo from "../logo.png";
import { toast } from "sonner";



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

  const loadingToast = toast.loading("Uploading snapshot...");

  setError(null);

  try {
    await api.uploadSnapshot(
      user.id,
      snapshot.device_name,
      snapshot.hostname,
      snapshot.os,
      accessToken
    );

    toast.dismiss(loadingToast);
    toast.success("Snapshot uploaded successfully!");

  } catch (err) {
    console.error("Upload snapshot failed:", err);

    toast.error(
      err instanceof Error ? err.message : "Upload failed.",
      {
        id: loadingToast,
      }
    );

    setError(
      err instanceof Error ? err.message : String(err)
    );
  }
}
return (
  <div className="relative max-w-5xl">

    {/* Floating Cloud Emojis
    <div className="fixed inset-0 z-5 pointer-events-none overflow-hidden">

  {Array.from({ length: 12 }).map((_, i) => {
  const edge = Math.floor(Math.random() * 4);

  let left = 0;
  let top = 0;

  switch (edge) {
    // Top
    case 0:
      left = Math.random() * 100;
      top = Math.random() * 15;
      break;

    // Right
    case 1:
      left = 85 + Math.random() * 15;
      top = Math.random() * 100;
      break;

    // Bottom
    case 2:
      left = Math.random() * 100;
      top = 85 + Math.random() * 15;
      break;

    // Left
    case 3:
      left = Math.random() * 15;
      top = Math.random() * 100;
      break;
  }

  return (
    <div
      key={i}
      className="absolute select-none"
      style={{
        left: `${left}%`,
        top: `${top}%`,
        fontSize: `${35 + Math.random() * 45}px`,
        opacity: 0.05 + Math.random() * 0.15,
        filter: "blur(1px)",
        transform: `rotate(${Math.random() * 30 - 15}deg)`,
        animation: `float ${20 + Math.random() * 20}s ease-in-out infinite`,
        animationDelay: `${Math.random() * 10}s`,
      }}
    >
      ☁️
    </div>
  );
})}

    </div> */}
      <div className="flex items-center gap-4 mb-8">
<div className="relative">

  <div
    className="
      absolute
      inset-0
      rounded-full
      bg-blue-500/20
      blur-3xl
      scale-125
    "
  />

  <img
    src={logo}
    alt="UPENV"
    className="relative w-45 h-45 object-contain"
  />

</div>

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