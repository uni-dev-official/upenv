import {
  createContext,
  useContext,
  useState,
  type ReactNode,
} from "react";

import type { User } from "../types";
import { api } from "../lib/api";

interface AuthContextValue {
  user: User | null;
  accessToken: string | null;
  refreshToken: string | null;
  isAuthenticated: boolean;

  login: (
    email: string,
    password: string
  ) => Promise<void>;

  register: (
    email: string,
    password: string
  ) => Promise<void>;

  refreshSession: () => Promise<string | null>;

  logout: () => Promise<void>;
}

const AuthContext = createContext<AuthContextValue | undefined>(
  undefined
);

const fallbackAuthContext: AuthContextValue = {
  user: null,
  accessToken: null,
  refreshToken: null,
  isAuthenticated: false,

  login: async () => {
    console.error("useAuth called outside AuthProvider");
  },

  register: async () => {
    console.error("useAuth called outside AuthProvider");
  },

  refreshSession: async () => {
    console.error("useAuth called outside AuthProvider");
    return null;
  },

  logout: async () => {
    console.error("useAuth called outside AuthProvider");
  },
};

export function AuthProvider({
  children,
}: {
  children: ReactNode;
}) {
  const [user, setUser] = useState<User | null>(null);

  const [accessToken, setAccessToken] =
    useState<string | null>(null);

  const [refreshToken, setRefreshToken] =
    useState<string | null>(null);

  async function login(
    email: string,
    password: string
  ) {
    const res = await api.login(
      email,
      password
    );

    console.log("LOGIN RESPONSE:", {
      user_id: res.user_id,
      email: res.email,
      access_token_length: res.access_token?.length,
      refresh_token_length: res.refresh_token?.length,
    });

    setUser({
      id: res.user_id,
      email: res.email,
    });

    setAccessToken(res.access_token);
    setRefreshToken(res.refresh_token);
  }

  async function register(
    email: string,
    password: string
  ) {
    const res = await api.register(
      email,
      password
    );

    console.log("REGISTER RESPONSE:", {
      user_id: res.user_id,
      email: res.email,
      access_token_length: res.access_token?.length,
      refresh_token_length: res.refresh_token?.length,
    });

    setUser({
      id: res.user_id,
      email: res.email,
    });

    setAccessToken(res.access_token);
    setRefreshToken(res.refresh_token);
  }

  async function refreshSession(): Promise<string | null> {
    if (!refreshToken) {
      console.error("Cannot refresh session: no refresh token");
      return null;
    }

    try {
      console.log("Refreshing Supabase session...");

      const res = await api.refreshSession(
        refreshToken
      );

      console.log("SESSION REFRESHED:", {
        access_token_length: res.access_token?.length,
        refresh_token_length: res.refresh_token?.length,
      });

      setAccessToken(res.access_token);

      if (res.refresh_token) {
        setRefreshToken(res.refresh_token);
      }

      if (res.user_id && res.email) {
        setUser({
          id: res.user_id,
          email: res.email,
        });
      }

      return res.access_token;
    } catch (error) {
      console.error(
        "Session refresh failed:",
        error
      );

      // Refresh token is no longer valid.
      setUser(null);
      setAccessToken(null);
      setRefreshToken(null);

      return null;
    }
  }

  async function logout() {
    try {
      await api.logout();
    } finally {
      setUser(null);
      setAccessToken(null);
      setRefreshToken(null);
    }
  }

  return (
    <AuthContext.Provider
      value={{
        user,
        accessToken,
        refreshToken,
        isAuthenticated: !!user,

        login,
        register,
        refreshSession,
        logout,
      }}
    >
      {children}
    </AuthContext.Provider>
  );
}

export function useAuth() {
  const ctx = useContext(AuthContext);

  if (!ctx) {
    console.error("useAuth called outside AuthProvider");
    return fallbackAuthContext;
  }

  return ctx;
}