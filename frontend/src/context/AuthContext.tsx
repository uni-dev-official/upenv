import { createContext, useContext, useState, type ReactNode } from "react";
import type { User } from "../types";
import { api } from "../lib/api";


interface AuthContextValue {
  user: User | null;
  accessToken: string | null;
  isAuthenticated: boolean;

  login: (
    email: string,
    password: string
  ) => Promise<void>;

  register: (
    email: string,
    password: string
  ) => Promise<void>;

  logout: () => Promise<void>;
}


const AuthContext = createContext<AuthContextValue | undefined>(
  undefined
);


export function AuthProvider({
  children,
}: {
  children: ReactNode;
}) {

  const [user, setUser] = useState<User | null>(null);

  const [accessToken, setAccessToken] =
    useState<string | null>(null);

async function login(
  email: string,
  password: string
) {

  const res = await api.login(
    email,
    password
  );

  console.log("LOGIN RESPONSE:", res);

  setUser({
    id: res.user_id,
    email: res.email,
  });

  setAccessToken(
    res.access_token
  );

  console.log(
    "JWT TOKEN:",
    res.access_token
  );
}


  async function register(
    email: string,
    password: string
  ) {

    const res = await api.register(
      email,
      password
    );


    setUser({
      id: res.user_id,
      email: res.email,
    });


    setAccessToken(
      res.access_token
    );
  }


  async function logout() {

    await api.logout();

    setUser(null);

    setAccessToken(null);
  }


  return (
    <AuthContext.Provider
      value={{
        user,
        accessToken,
        isAuthenticated: !!user,
        login,
        register,
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
    throw new Error(
      "useAuth must be used within AuthProvider"
    );
  }

  return ctx;
}