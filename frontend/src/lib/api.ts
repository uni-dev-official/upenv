// Thin wrapper around Tauri's invoke() — the only place the frontend calls
// into Rust commands. Keeps `@tauri-apps/api` usage centralized so pages
// stay framework-agnostic and easy to test.

import { invoke } from "@tauri-apps/api/core";
import type { AuthResponse, Device, Snapshot, SystemInfo } from "../types";

export const api = {
  // Auth (Milestone 2)
  login: (email: string, password: string) =>
    invoke<AuthResponse>("login", {
      email,
      password,
    }),

  register: (email: string, password: string) =>
    invoke<AuthResponse>("register", {
      email,
      password,
    }),

  // Refresh Supabase session
  refreshSession: (refreshToken: string) =>
    invoke<AuthResponse>("refresh_session", {
      refreshToken,
    }),

  logout: () => invoke<void>("logout"),

  // System / Scanner
  scanSystem: () => invoke<SystemInfo>("scan_system"),

  scanSnapshot: () => invoke<Snapshot>("scan_snapshot"),

  runFullScan: () => invoke<Snapshot>("run_full_scan"),

  // Devices (Milestone 7)
  listDevices: (userId: string, accessToken: string) =>
    invoke<Device[]>("list_devices", {
      userId,
      accessToken,
    }),

  // Restore (Milestone 8)
  runRestore: (deviceId: string, accessToken: string) =>
    invoke<void>("run_restore", {
      deviceId,
      accessToken,
    }),

  uploadSnapshot: (
    userId: string,
    deviceName: string,
    hostname: string,
    operatingSystem: string,
    accessToken: string,
  ) =>
    invoke("upload_snapshot", {
      userId,
      deviceName,
      hostname,
      operatingSystem,
      accessToken,
    }),
};
