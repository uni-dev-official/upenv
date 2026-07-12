# Architecture

## Layers (src-tauri)

```
commands/   Thin Tauri command handlers — parse args, call into a lower
            layer, map Result<T, E> -> Result<T, String> for the frontend.
scanner/    Read-only collectors. One file per concern (git, brew, vscode,
            node, python, docker, applications, system). snapshot.rs
            aggregates them all into a single Snapshot.
restore/    Appliers that reverse the scan — install/restore each concern.
            restore.rs orchestrates the pipeline and emits progress events.
models/     Shared serde structs: Snapshot, Device, User, SystemInfo.
            This is the wire format persisted in Supabase.
services/   Supabase integration (auth + storage). The only layer allowed
            to make network calls.
utils/      Stateless helpers: shell execution, path resolution, and the
            secret-redaction guard (utils/redact.rs) that every scanner
            must pass file contents through before including them in a
            snapshot.
```

## Data flow

1. **Scan**: `commands::scan::run_full_scan` -> `scanner::snapshot::build_snapshot`
   -> fans out to each scanner module -> assembles `models::snapshot::Snapshot`.
2. **Upload**: frontend calls a future `upload_snapshot` command ->
   `services::supabase_storage::upload_snapshot` -> Postgres row + Storage blob.
3. **Restore**: `commands::restore::run_restore` -> `restore::restore::run`
   -> downloads snapshot via `services::supabase_storage::fetch_snapshot`
   -> runs each `restore_*` step in order -> emits `restore://progress`
   events the frontend listens for on the Restore page.

## Frontend

```
src/
  pages/       One component per route (Login, Register, Dashboard,
               Devices, Restore, Settings)
  components/  Shared UI (Sidebar, AppLayout, Button, Card, RequireAuth)
  context/     AuthContext — session state, wraps lib/api auth calls
  lib/api.ts   The ONLY place the frontend calls tauri `invoke()`
  types/       TypeScript mirrors of the Rust models — keep in sync
```

Routing is guarded: unauthenticated users are redirected to `/login` via
`RequireAuth`. Authenticated routes render inside `AppLayout`, which owns
the sidebar navigation (Dashboard / Devices / Restore / Settings).

## Security boundary

`utils::redact::is_safe_to_upload` and `is_forbidden_filename` are the
single choke point for the "never upload secrets" requirement. Every
scanner that reads file contents (git, vscode, shell configs) must run
them through this check before they land in a `Snapshot`. SSH is only
ever checked for *existence* (`utils::fs_paths::ssh_dir_exists`) — its
contents are never read.
