# Upenv

Upenv recreates a developer's working environment on a new Mac —
applications, dev tools, configs, and workflow — in minutes.

This is **not** a backup tool. It's a workspace restoration tool.

## Stack

- **Desktop**: Tauri v2, React, TypeScript, Rust
- **Backend**: Supabase (Postgres, Auth, Storage)

## Project layout

```
Upenv/
  frontend/       React + TypeScript + Vite + Tailwind UI
  src-tauri/      Rust backend (Tauri commands, scanner, restore engine)
  backend/        Supabase SQL migrations / policies (added in Milestone 6)
  docs/           Architecture & milestone notes
```

See `docs/ARCHITECTURE.md` for the module map and `docs/MILESTONES.md` for
build order and current status.

## Development

```bash
cd frontend && npm install
cd ../src-tauri && cargo build   # requires Rust + Tauri CLI locally
npm run tauri dev                # from frontend/, once tauri-cli is set up
```

## Security

Upenv never uploads passwords, SSH private keys, API keys, tokens,
Keychain contents, or secret-bearing environment variables. See
`src-tauri/src/utils/redact.rs` for the enforcement point.
