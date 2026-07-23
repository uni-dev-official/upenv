# Backend (Supabase)

SQL migrations and Row Level Security policies land here starting at
Milestone 6 (Snapshot upload) and Milestone 7 (Devices page):

- `devices` table (id, user_id, name)
- `snapshots` table (id, user_id, device_id, storage_path, size)
- RLS: users can only read/write their own rows

Apply `migrations/001_enable_rls.sql` to enable the policies required by
the frontend upload flow.
