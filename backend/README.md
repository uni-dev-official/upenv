# Backend (Supabase)

SQL migrations and Row Level Security policies land here starting at
Milestone 6 (Snapshot upload) and Milestone 7 (Devices page):

- `devices` table (id, user_id, device_name, last_backup, snapshot_id)
- `snapshots` table or Storage bucket for machine.json blobs
- RLS: users can only read/write their own rows
