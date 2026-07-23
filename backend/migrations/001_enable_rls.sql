-- Restorely Supabase security policies.
-- Apply this migration in your Supabase project.

alter table public.devices enable row level security;
alter table public.snapshots enable row level security;

drop policy if exists "devices_select_own" on public.devices;
create policy "devices_select_own"
on public.devices
for select
using (auth.uid()::text = user_id);

drop policy if exists "devices_insert_own" on public.devices;
create policy "devices_insert_own"
on public.devices
for insert
with check (auth.uid()::text = user_id);

drop policy if exists "devices_update_own" on public.devices;
create policy "devices_update_own"
on public.devices
for update
using (auth.uid()::text = user_id)
with check (auth.uid()::text = user_id);

drop policy if exists "devices_delete_own" on public.devices;
create policy "devices_delete_own"
on public.devices
for delete
using (auth.uid()::text = user_id);

drop policy if exists "snapshots_select_own" on public.snapshots;
create policy "snapshots_select_own"
on public.snapshots
for select
using (auth.uid()::text = user_id);

drop policy if exists "snapshots_insert_own" on public.snapshots;
create policy "snapshots_insert_own"
on public.snapshots
for insert
with check (auth.uid()::text = user_id);

drop policy if exists "snapshots_update_own" on public.snapshots;
create policy "snapshots_update_own"
on public.snapshots
for update
using (auth.uid()::text = user_id)
with check (auth.uid()::text = user_id);

drop policy if exists "snapshots_delete_own" on public.snapshots;
create policy "snapshots_delete_own"
on public.snapshots
for delete
using (auth.uid()::text = user_id);
