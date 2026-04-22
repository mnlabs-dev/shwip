import { useCallback, useEffect, useState } from "react";

export type Theme = "system" | "light" | "dark";

const STORAGE_KEY = "shwip-theme";

function getSystemDark(): boolean {
	if (typeof window === "undefined" || !window.matchMedia) return false;
	return window.matchMedia("(prefers-color-scheme: dark)").matches;
}

function applyTheme(theme: Theme) {
	if (typeof document === "undefined") return;
	const isDark = theme === "dark" || (theme === "system" && getSystemDark());
	document.documentElement.classList.toggle("dark", isDark);
}

function readStoredTheme(): Theme {
	try {
		const stored = localStorage.getItem(STORAGE_KEY);
		if (stored === "light" || stored === "dark" || stored === "system")
			return stored;
	} catch {
		// localStorage may not be available in test environments
	}
	return "system";
}

export function useDarkMode() {
	const [theme, setThemeState] = useState<Theme>(readStoredTheme);

	const setTheme = useCallback((t: Theme) => {
		setThemeState(t);
		try {
			localStorage.setItem(STORAGE_KEY, t);
		} catch {
			// localStorage may not be available
		}
		applyTheme(t);
	}, []);

	useEffect(() => {
		applyTheme(theme);
	}, [theme]);

	useEffect(() => {
		if (theme !== "system") return;
		if (typeof window === "undefined" || !window.matchMedia) return;

		const mq = window.matchMedia("(prefers-color-scheme: dark)");
		const handler = () => applyTheme("system");
		mq.addEventListener("change", handler);
		return () => mq.removeEventListener("change", handler);
	}, [theme]);

	const isDark = theme === "dark" || (theme === "system" && getSystemDark());

	return { theme, setTheme, isDark };
}
