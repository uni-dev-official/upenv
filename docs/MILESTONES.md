# Milestones

Status legend: ✅ done · 🚧 scaffolded (stub, compiles, not implemented) · ⬜ not started

| # | Milestone              | Status | Notes |
|---|-------------------------|--------|-------|
| 1 | Project initialization  | ✅     | Vite+React+TS+Tailwind v4 frontend builds clean. Rust module skeleton in place (compiles once `cargo`/Tauri CLI available locally). |
| 2 | Authentication           | 🚧     | UI (Login/Register pages, AuthContext) done. `services::supabase_auth` needs real Supabase REST calls. |
| 3 | Dashboard                | 🚧     | UI done incl. live `scan_system` wiring. Upload/Restore buttons disabled pending M6/M8. |
| 4 | Scanner Engine           | ⬜     | All scanner files stubbed with correct return types; need real `utils::shell::run` calls. |
| 5 | Snapshot generation      | 🚧     | `scanner::snapshot::build_snapshot` aggregates scanners; needs real data once M4 lands. |
| 6 | Snapshot upload          | ⬜     | `services::supabase_storage::upload_snapshot` stubbed. Needs `backend/` SQL schema + Storage bucket. |
| 7 | Devices page             | 🚧     | UI done, calls `list_devices`; backend stub returns empty list. |
| 8 | Restore Engine           | 🚧     | Full pipeline + progress-event wiring done end-to-end (frontend listens on `restore://progress`); each `restore_*` step is a no-op stub. |
| 9 | UI polish                | ⬜     | Base dark theme + component kit in place; final pass later. |

## Next up

Milestone 2 (Authentication) is the natural next step — wire
`services::supabase_auth::{sign_up,sign_in,sign_out}` to real Supabase
REST endpoints once you share your Supabase project URL/anon key (as env
vars, never hardcoded).
