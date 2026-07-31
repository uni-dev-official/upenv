import { Outlet } from "react-router-dom";
import { Sidebar } from "../components/Sidebar";

export function AppLayout() {
  return (
    <div className="flex h-screen bg-[var(--color-bg)] text-[var(--color-text)]">
      <Sidebar />
      <main className="flex-1 overflow-y-auto p-10">
        <Outlet />
      </main>
    </div>
  );
}
