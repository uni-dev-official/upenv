import { createContext, useContext, useState, type ReactNode } from "react";
import type { User } from "../types";
import { api } from "../lib/api";

interface AuthContextValue {
  user: User | null;
  isAuthenticated: boolean;
  login: (email: string, password: string) => Promise<void>;
  register: (email: string, password: string) => Promise<void>;
  logout: () => Promise<void>;
}

const AuthContext = createContext<AuthContextValue | undefined>(undefined);

export function AuthProvider({ children }: { children: ReactNode }) {
  const [user, setUser] = useState<User | null>(null);

  async function login(email: string, password: string) {
    const res = await api.login(email, password);
    setUser({ id: res.user_id, email: res.email });
  }

  async function register(email: string, password: string) {
    const res = await api.register(email, password);
    setUser({ id: res.user_id, email: res.email });
  }

  async function logout() {
    await api.logout();
    setUser(null);
  }

  return (
    <AuthContext.Provider
      value={{ user, isAuthenticated: !!user, login, register, logout }}
    >
      {children}
    </AuthContext.Provider>
  );
}

export function useAuth() {
  const ctx = useContext(AuthContext);
  if (!ctx) throw new Error("useAuth must be used within AuthProvider");
  return ctx;
}
