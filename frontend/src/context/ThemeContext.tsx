import {
  createContext,
  useContext,
  useEffect,
  useState,
} from "react";

type Theme = "light" | "dark" | "system";

interface ThemeContextType {
  theme: Theme;
  setTheme: (theme: Theme) => void;
}

const ThemeContext = createContext<ThemeContextType | null>(null);

export function ThemeProvider({
  children,
}: {
  children: React.ReactNode;
}) {
  const [theme, setThemeState] = useState<Theme>(() => {
    return (localStorage.getItem("theme") as Theme) ?? "system";
  });

  useEffect(() => {
    const root = document.documentElement;

    const prefersDark = window.matchMedia(
      "(prefers-color-scheme: dark)"
    ).matches;

    const active =
      theme === "system"
        ? prefersDark
          ? "dark"
          : "light"
        : theme;

    root.dataset.theme = active;

    localStorage.setItem("theme", theme);
  }, [theme]);

  return (
    <ThemeContext.Provider
      value={{
        theme,
        setTheme: setThemeState,
      }}
    >
      {children}
    </ThemeContext.Provider>
  );
}

export function useTheme() {
  return useContext(ThemeContext)!;
}