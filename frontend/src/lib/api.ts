// Thin wrapper around Tauri's invoke() — the only place the frontend calls
// into Rust commands. Keeps `@tauri-apps/api` usage centralized so pages
// stay framework-agnostic and easy to test.

import { invoke } from "@tauri-apps/api/core";
import type { AuthResponse, Device, Snapshot, SystemInfo } from "../types";

export const api = {
  // Auth (Milestone 2)
  login: (email: string, password: string) =>
    invoke<AuthResponse>("login", { email, password }),

  register: (email: string, password: string) =>
    invoke<AuthResponse>("register", { email, password }),

  logout: () => invoke<void>("logout"),

  // System / Scanner (Milestone 1 smoke test, Milestone 4-5 full scan)
  scanSystem: () => invoke<SystemInfo>("scan_system"),
  scanSnapshot: () => invoke<Snapshot>("scan_snapshot"),

  runFullScan: () => invoke<Snapshot>("run_full_scan"),

  // Devices (Milestone 7)
  listDevices: (userId: string) => invoke<Device[]>("list_devices", { userId }),

  // Restore (Milestone 8)
  runRestore: (snapshotId: string) =>
    invoke<void>("run_restore", { snapshotId }),

  uploadSnapshot: (
  userId: string,
  deviceName: string,
) =>
  invoke<string>("upload_snapshot", {
    userId,
    deviceName,
  }),
};
